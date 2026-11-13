# Celdas app icon

**Figure:** Blaise Pascal. He built the first mechanical calculator, a fitting figure for a spreadsheet.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, in
the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "Portrait of Blaise Pascal MET DP828975.jpg" from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Portrait_of_Blaise_Pascal_MET_DP828975.jpg) (CC0). The figure was separated from its
background, converted to black and white and placed over the app colour.
Licence: see [LICENSE.txt](LICENSE.txt).

**Linux:** the hicolor icons (`hicolor/scalable` and every PNG size) put the tile on the Nodalix
app grid, like the Colloid icons around it: a 224 px tile inside a 256 px canvas (16 px margin),
a 51.5 px corner radius and a soft shadow. The scalable SVG is made by `tools/nodalia-app-icon.py`
in [nodalix-icon-theme](https://github.com/danielmigueltejedor/nodalix-icon-theme) and
`packaging/icons.sh` renders the PNG sizes from it. macOS, Windows and the 1024 px render keep the
full-bleed tile.

## Files

| File | Use |
|---|---|
| `celdas.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `celdas-1024.png` | 1024 px render of the full tile |
| `celdas-macos-512.png` | Runtime Dock icon on macOS (for `apps/celdas` to embed) |
| `celdas.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `celdas.ico` | Windows exe icon, 16–256 px (embedded by `apps/celdas/build.rs`) |
| `hicolor/<size>/apps/dev.danielmiguelt.Celdas.png` | Linux icon theme, 16–512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/dev.danielmiguelt.Celdas.svg` | Linux scalable icon |
| `LICENSE.txt` | MIT OR Apache-2.0 |

The app ID is `dev.danielmiguelt.Celdas`. It's used for the hicolor icon names, the Wayland app
ID and `packaging/linux/dev.danielmiguelt.Celdas.desktop`.

## Regenerate

The icon is a raster portrait, so the SVG files embed an image instead of traced paths. The
derived files in this directory were rendered directly from the portrait. `packaging/icons.sh`
still works on `celdas.svg` (resvg renders embedded images), but its output will differ slightly
from the committed files in resampling.
