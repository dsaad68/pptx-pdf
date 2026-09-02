/* lokconv with env toggles that mimic what a Rust std process does before/around LOK calls. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <signal.h>
#include <unistd.h>
#include <sys/stat.h>
#include <LibreOfficeKit/LibreOfficeKit.h>
#include <LibreOfficeKit/LibreOfficeKitInit.h>
static void segv(int s) { (void)s; _exit(99); }
int main(int argc, char** argv) {
    if (argc < 5) return 64;
    if (getenv("X_SIGPIPE")) signal(SIGPIPE, SIG_IGN);                     /* Rust ignores SIGPIPE at startup */
    if (getenv("X_ALTSTACK")) {                                              /* Rust: sigaltstack + SA_ONSTACK SIGSEGV/SIGBUS handler */
        stack_t ss; ss.ss_sp = malloc(65536); ss.ss_size = 65536; ss.ss_flags = 0; sigaltstack(&ss, NULL);
        struct sigaction sa; memset(&sa, 0, sizeof sa); sa.sa_handler = segv; sa.sa_flags = SA_ONSTACK | SA_SIGINFO;
        sigaction(SIGSEGV, &sa, NULL); sigaction(SIGBUS, &sa, NULL);
    }
    if (getenv("X_MKPROFILE")) { const char* u = argv[2]; if (!strncmp(u, "file://", 7)) mkdir(u + 7, 0755); }  /* profile dir pre-created */
    LibreOfficeKit* lok = lok_init_2(argv[1], argv[2]);
    if (!lok) { fprintf(stderr, "init failed\n"); return 1; }
    if (getenv("X_VERSION")) { char* v = lok->pClass->getVersionInfo(lok); lok->pClass->freeError(v); }
    LibreOfficeKitDocument* doc = lok->pClass->documentLoad(lok, argv[3]);
    if (!doc) { char* e = lok->pClass->getError(lok); fprintf(stderr, "load failed: %s\n", e); return 2; }
    if (getenv("X_DOCTYPE")) (void)doc->pClass->getDocumentType(doc);
    if (getenv("X_SLEEP")) sleep(atoi(getenv("X_SLEEP")));                /* give the main loop time to run its startup events */
    int ok = doc->pClass->saveAs(doc, argv[4], "pdf", NULL);
    if (!ok) { char* e = lok->pClass->getError(lok); fprintf(stderr, "saveAs failed: %s\n", e); return 3; }
    doc->pClass->destroy(doc); lok->pClass->destroy(lok); return 0;
}
