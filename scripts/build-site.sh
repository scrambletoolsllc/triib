#!/usr/bin/env bash
# Assembles triib's website (triib.run) into OUT: the pages in site/, in
# every language the app speaks, with the latest release's downloads,
# plus the icon and the screenshots, from the repository.
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
# The latest release, whose files fill in the download table, and its
# PKGBUILD, whose arch=() the Arch row lists; gh needs to be signed in,
# as GH_TOKEN in CI.
release=$(mktemp)
pkgbuild=$(mktemp)
trap 'rm -f "$release" "$pkgbuild"' EXIT
gh api repos/scrambletoolsllc/triib/releases/latest >"$release"
gh release download "$(jq -r .tag_name "$release")" --repo scrambletoolsllc/triib \
    --pattern PKGBUILD --output "$pkgbuild" --clobber
python3 "$root/scripts/site-i18n.py" build "$out" "$release" "$pkgbuild"
# The icon as drawn, for the front page's large one, and the header's
# small one rendered at each pixel density it shows at, as browsers
# draw a vector or shrink a large image that small with jagged edges.
cp "$root/data/icons/hicolor/scalable/apps/run.triib.svg" "$out/icon.svg"
for size in 32 48 64 96 128; do
  cp "$root/data/icons/hicolor/${size}x$size/apps/run.triib.png" "$out/icon-$size.png"
done
cp "$root/data/icons/hicolor/32x32/apps/run.triib.png" "$out/favicon.png"
cp -r "$root/docs/screenshots/." "$out/screenshots/"
echo "Built the site in $out"
