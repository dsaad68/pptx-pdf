/* DYLD_INSERT_LIBRARIES interposer: log every path the process opens or stats.
 * macOS has no strace; fs_usage needs root and dtrace is blocked by SIP. This runs in-process
 * and writes one path per line to $FSTRACE_OUT (append). Build:
 *   cc -dynamiclib -o fstrace.dylib fstrace.c
 * Use:
 *   FSTRACE_OUT=/tmp/t.log DYLD_INSERT_LIBRARIES=./fstrace.dylib ./program ... */
#include <dlfcn.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <dirent.h>

static int out_fd = -1;
static void note(char kind, const char *path) {
    if (!path) return;
    if (out_fd < 0) {
        const char *o = getenv("FSTRACE_OUT");
        if (!o) return;
        out_fd = open(o, O_WRONLY | O_APPEND | O_CREAT, 0644);
        if (out_fd < 0) return;
    }
    size_t n = strlen(path);
    if (n == 0) return;
    char buf[4096];
    if (n > sizeof buf - 4) n = sizeof buf - 4;
    buf[0] = kind; buf[1] = ' ';
    memcpy(buf + 2, path, n); buf[n + 2] = '\n';
    write(out_fd, buf, n + 3);
}

#define INTERPOSE(_r, _o) __attribute__((used)) static const struct { const void *r, *o; } _i_##_o \
    __attribute__((section("__DATA,__interpose"))) = { (const void *)&_r, (const void *)&_o };

static int my_open(const char *p, int f, ...) { note('o', p); mode_t m = 0; if (f & O_CREAT) { va_list a; va_start(a, f); m = (mode_t)va_arg(a, int); va_end(a); } return open(p, f, m); }
static int my_openat(int d, const char *p, int f, ...) { note('o', p); mode_t m = 0; if (f & O_CREAT) { va_list a; va_start(a, f); m = (mode_t)va_arg(a, int); va_end(a); } return openat(d, p, f, m); }
static int my_stat(const char *p, struct stat *s) { note('s', p); return stat(p, s); }
static int my_lstat(const char *p, struct stat *s) { note('s', p); return lstat(p, s); }
static int my_access(const char *p, int m) { note('s', p); return access(p, m); }
static DIR *my_opendir(const char *p) { note('d', p); return opendir(p); }
static FILE *my_fopen(const char *p, const char *m) { note('o', p); return fopen(p, m); }
static ssize_t my_readlink(const char *p, char *b, size_t n) { note('s', p); return readlink(p, b, n); }
static void *my_dlopen(const char *p, int m) { note('o', p); return dlopen(p, m); }

INTERPOSE(my_open, open)
INTERPOSE(my_openat, openat)
INTERPOSE(my_stat, stat)
INTERPOSE(my_lstat, lstat)
INTERPOSE(my_access, access)
INTERPOSE(my_opendir, opendir)
INTERPOSE(my_fopen, fopen)
INTERPOSE(my_readlink, readlink)
INTERPOSE(my_dlopen, dlopen)
