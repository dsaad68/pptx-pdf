| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

#| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

x| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

-| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

(| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

P| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

y| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

)| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

y| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

m| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

x| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

_| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

x| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

_| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

v| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

(| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

"| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

k| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

x| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

"| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

,| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

"| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

k| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

"| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

,| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

g| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

=| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

"| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

2| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

-| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

5| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

"| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

,| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

=| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

T| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

,| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

_| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

=| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

[| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

"| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

/| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

b| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

"| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

]| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

)| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

x| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

_| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

(| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

)| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

#| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

-| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

w| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

L| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

b| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

O| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

(| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

b| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

8| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

0| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

M| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

B| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

,| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

v| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

)| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

L| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

b| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

O| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

'| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

,| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

-| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

g| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

L| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

b| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

O| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

K| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

;| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

g| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

T| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

y| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

(| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

w| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

x| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

-| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

C| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

L| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

I| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

)| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

v| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

g| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

b| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

S| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

P| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

P| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

T| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

X| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

_| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

P| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

D| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

F| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

_| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

L| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

O| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

_| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

D| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

I| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

R| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

x| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

g| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

L| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

b| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

O| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

x| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

_| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

C| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

v| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

(| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

=| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

T| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

)| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

`| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

v| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

y| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

w| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

m| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

,| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

L| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

b| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

O| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

k| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

y| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

u| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

w| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

n| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

S| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

h| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

p| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

j| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

c| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

R| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

E| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

A| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

D| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

M| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

E| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

f| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

o| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

r| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

 | | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

d| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

e| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

t| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

a| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

i| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

l| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

s| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

.| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.


| | Linux arm64 | Linux x86_64 | macOS arm64 |
|---|---|---|---|
| LibreOffice as installed | 199 MB | 199 MB | 804 MB |
| self-contained binary | 62 MB | 64 MB | 80 MB |
| slim binary / Python wheel | 4 MB / 2 MB | 4 MB / 2 MB | 4 MB / 1 MB |
| cache after first run | 225 MB | 230 MB | 360 MB |
| install (`curl \| sh`, 80 MB) | | | 44 s |
| first run (unpack + first boot + convert) | 1.8 s | | 18 s |

Conversion time, cold process every run, against LibreOffice itself started cold
(`soffice --headless --convert-to pdf` with a fresh profile), same LibreOffice build on each
platform:

| deck | Linux: `soffice` cold | Linux: `pptx-pdf` | macOS: `soffice` cold | macOS: `pptx-pdf` |
|---|---|---|---|---|
| 6 slides, text and shapes | 0.30-0.35 s | **0.18 s** | 0.6-0.9 s | 1.3-1.6 s |
| 12 slides, images and icons, 5.6 MB | 1.5-1.9 s | **1.4-1.5 s** | 2.0-2.2 s | 2.6 s |

On Linux the difference is startup: LibreOfficeKit boots in ~0.24 s where `soffice` needs
~0.3 s, and rendering dominates on heavy decks. On macOS `pptx-pdf` is slower, and that is the
cost of seeing the machine's fonts: fontconfig enumerates 700+ system font files each start
(~0.6 s here), while `soffice --headless` never looks beyond the 128 fonts it bundles and
substitutes silently for everything else. With the same bundled-only view `pptx-pdf` runs in
1.9-2.1 s, matching `soffice`.

