/* paint: render every slide of a deck to PNG through LibreOfficeKit's tile API.
 * usage: paint <program-dir> <profile-file-url> <in-url> <out-dir> [dpi]
 * program-dir: directory holding libmergedlo (Contents/Frameworks on macOS, program/ on Linux). */

#include <LibreOfficeKit/LibreOfficeKit.h>
enum { LOK_TILEMODE_RGBA = 0, LOK_TILEMODE_BGRA = 1 };
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>
#include <time.h>
#include <zlib.h>

static double now(void) { struct timespec t; clock_gettime(CLOCK_MONOTONIC, &t); return t.tv_sec + t.tv_nsec / 1e9; }

static void put32(unsigned char* p, unsigned v) { p[0] = v >> 24; p[1] = v >> 16; p[2] = v >> 8; p[3] = v; }
static void chunk(FILE* f, const char* type, const unsigned char* data, unsigned len) {
    unsigned char hdr[8]; put32(hdr, len); memcpy(hdr + 4, type, 4);
    fwrite(hdr, 1, 8, f); if (len) fwrite(data, 1, len, f);
    unsigned crc = crc32(0, (const unsigned char*)type, 4); if (len) crc = crc32(crc, data, len);
    unsigned char c[4]; put32(c, crc); fwrite(c, 1, 4, f);
}
/* buf is 4 bytes per pixel, tile mode decides order; write opaque RGB PNG */
static int write_png(const char* path, const unsigned char* buf, int w, int h, int rgba) {
    size_t raw_len = (size_t)h * (1 + 3 * (size_t)w);
    unsigned char* raw = malloc(raw_len);
    for (int y = 0; y < h; y++) {
        unsigned char* row = raw + (size_t)y * (1 + 3 * w); row[0] = 0;
        for (int x = 0; x < w; x++) {
            const unsigned char* px = buf + 4 * ((size_t)y * w + x);
            if (rgba) { row[1 + 3 * x] = px[0]; row[2 + 3 * x] = px[1]; row[3 + 3 * x] = px[2]; }
            else      { row[1 + 3 * x] = px[2]; row[2 + 3 * x] = px[1]; row[3 + 3 * x] = px[0]; }
        }
    }
    uLongf zlen = compressBound(raw_len); unsigned char* z = malloc(zlen);
    if (compress2(z, &zlen, raw, raw_len, 6) != Z_OK) return -1;
    FILE* f = fopen(path, "wb"); if (!f) return -1;
    fwrite("\x89PNG\r\n\x1a\n", 1, 8, f);
    unsigned char ihdr[13]; put32(ihdr, w); put32(ihdr + 4, h); ihdr[8] = 8; ihdr[9] = 2; ihdr[10] = 0; ihdr[11] = 0; ihdr[12] = 0;
    chunk(f, "IHDR", ihdr, 13); chunk(f, "IDAT", z, (unsigned)zlen); chunk(f, "IEND", NULL, 0);
    fclose(f); free(raw); free(z); return 0;
}

typedef LibreOfficeKit* (*hook2)(const char*, const char*);

int main(int argc, char** argv) {
    if (argc < 5) { fprintf(stderr, "usage: paint <program-dir> <profile-url> <in-url> <out-dir> [dpi]\n"); return 64; }
    int dpi = argc > 5 ? atoi(argv[5]) : 150;
    char lib[4096]; snprintf(lib, sizeof lib, "%s/%s", argv[1],
#ifdef __APPLE__
        "libmergedlo.dylib"
#else
        "libmergedlo.so"
#endif
    );
    double t0 = now();
    void* h = dlopen(lib, RTLD_LAZY | RTLD_LOCAL);
    if (!h) { fprintf(stderr, "dlopen: %s\n", dlerror()); return 1; }
    hook2 hook = (hook2)dlsym(h, "libreofficekit_hook_2");
    if (!hook) { fprintf(stderr, "no hook\n"); return 1; }
    LibreOfficeKit* lok = hook(argv[1], argv[2]);
    if (!lok) { fprintf(stderr, "hook failed\n"); return 1; }
    double t1 = now();
    LibreOfficeKitDocument* doc = lok->pClass->documentLoad(lok, argv[3]);
    if (!doc) { char* e = lok->pClass->getError(lok); fprintf(stderr, "load failed: %s\n", e); lok->pClass->freeError(e); return 2; }
    double t2 = now();
    size_t n = doc->pClass->nSize;
    fprintf(stderr, "doc class nSize=%zu paintTile@%zu paintPartTile@%zu getPartInfo@%zu\n", n,
            offsetof(LibreOfficeKitDocumentClass, paintTile), offsetof(LibreOfficeKitDocumentClass, paintPartTile),
            offsetof(LibreOfficeKitDocumentClass, getPartInfo));
    if (!LIBREOFFICEKIT_DOCUMENT_HAS(doc, paintTile)) { fprintf(stderr, "no paintTile\n"); return 3; }
    doc->pClass->initializeForRendering(doc, getenv("LOK_INIT_ARGS"));
    int mode = doc->pClass->getTileMode(doc);
    int parts = doc->pClass->getParts(doc);
    fprintf(stderr, "type=%d tileMode=%s parts=%d  boot %.3fs load %.3fs\n", doc->pClass->getDocumentType(doc),
            mode == LOK_TILEMODE_RGBA ? "RGBA" : "BGRA", parts, t1 - t0, t2 - t1);
    for (int i = 0; i < parts; i++) {
        double a = now();
        doc->pClass->setPart(doc, i);
        long w = 0, hh = 0; doc->pClass->getDocumentSize(doc, &w, &hh);
        int cw = (int)(w * dpi / 1440.0 + 0.5), ch = (int)(hh * dpi / 1440.0 + 0.5);
        unsigned char* buf = calloc((size_t)cw * ch, 4);
        doc->pClass->paintTile(doc, buf, cw, ch, 0, 0, (int)w, (int)hh);
        double b = now();
        char path[4096]; snprintf(path, sizeof path, "%s/slide-%02d.png", argv[4], i + 1);
        write_png(path, buf, cw, ch, mode == LOK_TILEMODE_RGBA);
        char* info = LIBREOFFICEKIT_DOCUMENT_HAS(doc, getPartInfo) ? doc->pClass->getPartInfo(doc, i) : NULL;
        char* name = doc->pClass->getPartName(doc, i);
        fprintf(stderr, "slide %2d: %ldx%ld twips -> %dx%d px  paint %.3fs png %.3fs  name=%s info=%s\n",
                i + 1, w, hh, cw, ch, b - a, now() - b, name ? name : "", info ? info : "");
        if (info) lok->pClass->freeError(info);
        if (name) lok->pClass->freeError(name);
        free(buf);
    }
    /* what does saveAs png give? */
    char out[4096]; snprintf(out, sizeof out, "file://%s/saveas.png", argv[4]);
    double s = now();
    int ok = doc->pClass->saveAs(doc, out, "png", NULL);
    fprintf(stderr, "saveAs png: ok=%d %.3fs\n", ok, now() - s);
    doc->pClass->destroy(doc);
    lok->pClass->destroy(lok);
    fprintf(stderr, "total %.3fs\n", now() - t0);
    return 0;
}
