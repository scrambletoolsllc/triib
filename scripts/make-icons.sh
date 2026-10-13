#!/usr/bin/env bash
# Renders triib's icons from data/icons/triib-icon.svg: the hicolor PNGs
# and the scalable SVG the Linux packages install, which the window icon
# and the website use too, the 1024-pixel PNG the macOS bundle takes, and
# the Windows .ico. Run it again whenever the SVG changes. Needs
# rsvg-convert (librsvg) and ImageMagick.
#
#   scripts/make-icons.sh
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
svg=$root/data/icons/triib-icon.svg
id=io.github.scrambletools.triib
hicolor=$root/data/icons/hicolor
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

for size in 16 22 24 32 48 64 96 128 256 512; do
  mkdir -p "$hicolor/${size}x$size/apps"
  rsvg-convert -w "$size" -h "$size" "$svg" -o "$hicolor/${size}x$size/apps/$id.png"
done
mkdir -p "$hicolor/scalable/apps"
cp "$svg" "$hicolor/scalable/apps/$id.svg"

rsvg-convert -w 1024 -h 1024 "$svg" -o "$root/data/icons/triib-1024.png"

ico=()
for size in 16 20 24 32 40 48 64 96 128 256; do
  rsvg-convert -w "$size" -h "$size" "$svg" -o "$work/$size.png"
  ico+=("$work/$size.png")
done
magick "${ico[@]}" "$root/packaging/windows/triib.ico"

echo "Rendered the icons from $svg"
