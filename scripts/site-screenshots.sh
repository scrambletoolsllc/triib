#!/usr/bin/env bash
# Renders the website's screenshots into docs/screenshots: the picture
# tests draw the window and the network map on the test bench, dark with
# Settings' gray accent, and ImageMagick crops them and scales them to the
# site's width. The network maps are GIFs of one beat of their wires'
# animation. Run it again whenever what they show changes.
#
#   scripts/site-screenshots.sh
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
out=$root/docs/screenshots
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

# The map's wires move a step each beat of 0.4 s: ten frames of 4 cs.
frames=10
export PICTURE_ACCENT='#757575'

cd "$root"
WINDOW_PICTURE=$work/window \
  cargo test -q -p triib --bin triib -- --ignored --exact view::tests::pictures
NETWORK_PICTURE=$work/net NETWORK_ONLY=show,show-clock,venue NETWORK_FRAMES=$frames \
  cargo test -q -p triib --bin triib -- --ignored --exact netmap::map::tests::pictures

mkdir -p "$out"
# shot PICTURE NAME [CROP] [WIDTH]: one picture, cropped first where CROP,
# such as 2560x2100+0+0, is given, and scaled to WIDTH, 1360 unless given.
shot() {
  local crop=()
  [ -n "${3:-}" ] && crop=(-crop "$3" +repage)
  magick "$work/$1-tiny-skia.png" "${crop[@]}" -resize "${4:-1360}x" -strip "$out/$2.png"
}
# panel PICTURE NAME HEIGHT: the inspector alone, from its title down.
panel() {
  shot "$1" "$2" "640x$3+1920+140" 640
}
# animated PICTURE NAME [WIDTH] [SIZE]: the frames as a looping GIF,
# scaled to WIDTH, else drawn as they are, or set in the middle of SIZE,
# such as 1200x916, on the map's background; in one palette, so nothing
# still flickers.
animated() {
  local pictures=("$work/$1"-[0-9][0-9]-tiny-skia.png) resize=()
  [ -n "${3:-}" ] && resize=(-resize "$3x")
  [ -n "${4:-}" ] && resize=(-background "$(magick "${pictures[0]}" -format '%[pixel:p{0,0}]' info:)"
    -gravity center -extent "$4")
  magick "${pictures[@]}" "${resize[@]}" +append +dither -colors 256 -unique-colors \
    "$work/$1-palette.png"
  magick -delay $((40 / frames)) -loop 0 "${pictures[@]}" "${resize[@]}" \
    +dither -remap "$work/$1-palette.png" -layers OptimizeTransparency "$out/$2.gif"
}
shot window-matrix-desktop matrix
shot window-matrix-formats matrix-formats
shot window-entities-host entities
panel window-inspector-lectern inspector 1500
panel window-inspector-mappings streams 1500
panel window-inspector-diagnostics diagnostics 1500
panel window-inspector-controls controls 530
panel window-inspector-wireless-section wireless 810
# One of this computer's entities, and on a Mac its own entity, whose
# view would have Linux's Add entity bar.
panel window-inspector-device device 1100
panel window-inspector-listen listen 1120
shot window-settings-general settings
shot window-presets-desktop presets
shot window-log-desktop log
animated net-show network 1360
animated net-show-clock network-clock 1360
# As wide for its height as the clock tree beside it on the front page.
animated net-venue avb-lite '' 1200x916
# Stills of the show too, for the AppStream data, which takes no GIFs.
shot net-show-00 network
shot net-show-clock-00 network-clock
echo "Rendered the screenshots in $out"
