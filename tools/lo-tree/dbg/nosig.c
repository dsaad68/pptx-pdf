/* LD_PRELOAD shim: keep LibreOffice/Rust from installing crash handlers, install ours that
 * prints a backtrace, so we learn where a headless crash actually happens. */
#define _GNU_SOURCE
#include <signal.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <dlfcn.h>
#include <execinfo.h>
static int (*real_sigaction)(int, const struct sigaction *, struct sigaction *);
static void dump(int sig) {
    void *frames[64]; int n = backtrace(frames, 64);
    fprintf(stderr, "=== shim: signal %d, %d frames ===\n", sig, n);
    backtrace_symbols_fd(frames, n, 2);
    _exit(128 + sig);
}
static int crash_sig(int s) { return s == SIGSEGV || s == SIGBUS || s == SIGILL || s == SIGFPE || s == SIGABRT || s == SIGTRAP; }
int sigaction(int sig, const struct sigaction *act, struct sigaction *old) {
    if (!real_sigaction) real_sigaction = dlsym(RTLD_NEXT, "sigaction");
    if (act && crash_sig(sig)) { if (old) memset(old, 0, sizeof *old); return 0; }
    return real_sigaction(sig, act, old);
}
__attribute__((constructor)) static void install(void) {
    if (!real_sigaction) real_sigaction = dlsym(RTLD_NEXT, "sigaction");
    struct sigaction sa; memset(&sa, 0, sizeof sa); sa.sa_handler = dump; sa.sa_flags = SA_NODEFER;
    int sigs[] = {SIGSEGV, SIGBUS, SIGILL, SIGFPE, SIGABRT, SIGTRAP};
    for (unsigned i = 0; i < sizeof sigs / sizeof *sigs; i++) real_sigaction(sigs[i], &sa, NULL);
}
