#!/usr/bin/env bash
# Renders the website's screenshots into docs/screenshots: the picture
# tests draw the window and the network map on the test bench, and
# ImageMagick scales them to the site's width. Run it again whenever
# what they show changes.
#
#   scripts/site-screenshots.sh
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
out=$root/docs/screenshots
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

cd "$root"
WINDOW_PICTURE=$work/window \
  cargo test -q -p triib --bin triib -- --ignored --exact view::tests::pictures
NETWORK_PICTURE=$work/net NETWORK_LITE_PICTURE=$work/lite \
  cargo test -q -p triib --bin triib -- --ignored --exact \
  netmap::map::tests::pictures netmap::tests::lite_picture

mkdir -p "$out"
# shot PICTURE NAME [CROP] [WIDTH]: one picture, cropped first where CROP,
# such as 2560x2100+0+0, is given, and scaled to WIDTH, 1360 unless given.
shot() {
  local crop=()
  [ -n "${3:-}" ] && crop=(-crop "$3" +repage)
  magick "$work/$1-tiny-skia.png" "${crop[@]}" -resize "${4:-1360}x" -strip "$out/$2.png"
}
shot window-matrix-desktop matrix
shot window-entities-desktop entities
shot window-inspector-lite inspector 2560x2100+0+0
shot window-inspector-controls controls
shot window-matrix-formats matrix-formats
# The inspector alone, as its entity is a stand-in for this computer's.
shot window-inspector-device device 640x1500+1920+140 640
shot window-inspector-wireless wireless 2560x2100+0+0
shot window-settings-general settings
shot window-presets-desktop presets
shot window-log-desktop log
shot net-show network
shot net-show-clock network-clock
shot lite-overview avb-lite
echo "Rendered the screenshots in $out"
