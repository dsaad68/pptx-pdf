#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <unistd.h>
#include <LibreOfficeKit/LibreOfficeKit.h>
#include <LibreOfficeKit/LibreOfficeKitInit.h>
/* usage: lokconv <program-dir> <profile-file-url> <in-url> <out-url> [filter-options]
 * LOKCONV_SETTLE=<seconds>: pause after load and after save so LibreOffice's main-loop thread
 * runs its deferred startup work (frame/UI construction) — those file opens must be traced too. */
static void settle(void) { const char* s = getenv("LOKCONV_SETTLE"); if (s && atoi(s) > 0) sleep(atoi(s)); }
int main(int argc, char** argv) {
    if (argc < 5) { fprintf(stderr, "usage\n"); return 64; }
    LibreOfficeKit* lok = lok_init_2(argv[1], argv[2]);
    if (!lok) { fprintf(stderr, "lok_init_2 failed\n"); return 1; }
    LibreOfficeKitDocument* doc = lok->pClass->documentLoad(lok, argv[3]);
    if (!doc) { char* e = lok->pClass->getError(lok); fprintf(stderr, "load failed: %s\n", e); lok->pClass->freeError(e); return 2; }
    settle();
    int ok = doc->pClass->saveAs(doc, argv[4], "pdf", argc > 5 ? argv[5] : NULL);
    if (!ok) { char* e = lok->pClass->getError(lok); fprintf(stderr, "saveAs failed: %s\n", e); lok->pClass->freeError(e); return 3; }
    settle();
    doc->pClass->destroy(doc);
    lok->pClass->destroy(lok);
    return 0;
}
