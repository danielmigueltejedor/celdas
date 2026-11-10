# Celdas app icon

**Figure:** Blaise Pascal. He built the first mechanical calculator, a fitting figure for a spreadsheet.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, in
the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "Portrait of Blaise Pascal MET DP828975.jpg" from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Portrait_of_Blaise_Pascal_MET_DP828975.jpg) (CC0). The figure was separated from its
background, converted to black and white and placed over the app colour.
Licence: see [LICENSE.txt](LICENSE.txt).

## Files

| File | Use |
|---|---|
| `gridcraft.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `gridcraft-1024.png` | 1024 px render of the full tile |
| `gridcraft-macos-512.png` | Runtime Dock icon on macOS (for `apps/gridcraft` to embed) |
| `gridcraft.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `gridcraft.ico` | Windows exe icon, 16–256 px (embedded by `apps/gridcraft/build.rs`) |
| `hicolor/<size>/apps/ai.storyteller.gridcraft.png` | Linux icon theme, 16–512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/ai.storyteller.gridcraft.svg` | Linux scalable icon |
| `LICENSE.txt` | MIT OR Apache-2.0 |

The app ID is `ai.storyteller.gridcraft`. It's used for the hicolor icon names, the Wayland app
ID and `packaging/linux/ai.storyteller.gridcraft.desktop`.

## Regenerate

The icon is a raster portrait, so the SVG files embed an image instead of traced paths. The
derived files in this directory were rendered directly from the portrait. `packaging/icons.sh`
still works on `gridcraft.svg` (resvg renders embedded images), but its output will differ slightly
from the committed files in resampling.
