#!/usr/bin/env bash
# Assembles triib's website (triib.run) into OUT: the pages in site/, in
# every language the app speaks, plus the icon and the screenshots, from
# the repository.
#
#   scripts/build-site.sh [OUT]
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
out=${1:-$root/target/site}

rm -rf "$out"
mkdir -p "$out/screenshots"
# site/. rather than site/*, so .well-known, which Azure checks the
# publisher domain by, comes along.
cp -r "$root"/site/. "$out/"
# The translations go into the pages rather than beside them.
rm -rf "$out/i18n"
python3 "$root/scripts/site-i18n.py" build "$out"
cp "$root/data/icons/hicolor/256x256/apps/run.triib.png" "$out/icon.png"
cp "$root/data/icons/hicolor/32x32/apps/run.triib.png" "$out/favicon.png"
cp -r "$root/docs/screenshots/." "$out/screenshots/"
echo "Built the site in $out"
