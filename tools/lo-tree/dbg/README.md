# Debugging a headless LibreOffice crash without ptrace

Neither `strace` nor `gdb` work under CPU emulation (Rosetta/QEMU), and LibreOffice's own
crash handler turns every fatal signal into "Unspecified Application Error" with no location.

- `nosig.c` — `LD_PRELOAD` shim. Intercepts `sigaction` so LibreOffice (and Rust's runtime)
  cannot install crash handlers, installs its own that prints `backtrace()`. Resolve the
  `libmergedlo.so+0x...` offsets with `nm -D -n` against the same library.
- `harness2.c` — the LOK conversion harness with env toggles that mimic what a Rust process
  does (`X_SIGPIPE`, `X_ALTSTACK`, `X_MKPROFILE`, `X_VERSION`, `X_DOCTYPE`) and `X_SLEEP=<s>`
  to let LibreOffice's main-loop thread run its deferred work before exit. `X_SLEEP=5` against
  a candidate tree is the quickest "is this tree complete" check there is.

- `paint.c` — renders every slide of a deck to PNG through LOK's tile API (`setPart`,
  `getDocumentSize`, `paintTile`) and prints per-slide timings, the tile mode and
  `getPartInfo`. This is the experiment behind `--png`; `LOK_INIT_ARGS` passes a JSON object
  to `initializeForRendering` (the spell-check squiggle fix was found this way).

```
cc -O2 -DLOK_USE_UNSTABLE_API -I ../include -o paint paint.c -lz
./paint /Applications/LibreOffice.app/Contents/Frameworks file:///tmp/prof file:///x/deck.pptx /tmp/out 150
gcc -shared -fPIC -o nosig.so nosig.c -ldl
LD_PRELOAD=./nosig.so pptx-pdf deck.pptx --lo-dir /opt/lo
gcc -o h2 harness2.c -ldl && X_SLEEP=5 ./h2 /opt/lo/program file:///tmp/p file:///x/deck.pptx /tmp/o.pdf
```
