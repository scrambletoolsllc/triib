#!/usr/bin/env bash
# Installs a built triib, triib-cli, triib-endpointd and triib-netraw and
# their data files into a system layout, for packages: the binaries,
# triib-netraw, the one granted CAP_NET_RAW, beside them, desktop entry,
# icons when there are any, AppStream metadata and licenses, and the
# ptp4l configurations, systemd units and polkit rule triib-endpointd
# switches PTP profiles with, and the check the packages run as they
# install for an interface that can carry host entities.
#
#   scripts/dist-install.sh BINARY_DIR [DESTDIR] [PREFIX]
#
# BINARY_DIR holds the four programs from a release build made with
# TRIIB_PRODUCTION=1. DESTDIR is the staging root (empty for a direct
# install) and PREFIX defaults to /usr. Packages then grant triib-netraw
# CAP_NET_RAW when installed; the others hold no capability, their raw
# sockets coming from it.
set -euo pipefail

binaries=${1:?usage: dist-install.sh BINARY_DIR [DESTDIR] [PREFIX]}
destdir=${2:-}
prefix=${3:-/usr}
root=$(cd "$(dirname "$0")/.." && pwd)
id=io.github.scrambletools.triib
share="$destdir$prefix/share"

install -Dm755 "$binaries/triib" "$destdir$prefix/bin/triib"
install -Dm755 "$binaries/triib-cli" "$destdir$prefix/bin/triib-cli"
install -Dm755 "$binaries/triib-endpointd" "$destdir$prefix/bin/triib-endpointd"
install -Dm755 "$binaries/triib-netraw" "$destdir$prefix/lib/triib/triib-netraw"
install -Dm755 "$root/packaging/linux/endpoint-check" "$destdir$prefix/lib/triib/endpoint-check"
install -Dm644 "$root/data/$id.desktop" "$share/applications/$id.desktop"
install -Dm644 "$root/data/$id.metainfo.xml" "$share/metainfo/$id.metainfo.xml"
if [ -d "$root/data/icons" ]; then
    (cd "$root/data/icons" && find hicolor -type f) | while read -r icon; do
        install -Dm644 "$root/data/icons/$icon" "$share/icons/$icon"
    done
fi
for license in LICENSE-MIT LICENSE-APACHE; do
    install -Dm644 "$root/$license" "$share/licenses/triib/$license"
done
for font_license in OFL.txt LICENSE-MaterialSymbols.txt; do
    install -Dm644 "$root/packaging/licenses/$font_license" \
        "$share/licenses/triib/fonts/$font_license"
done
install -Dm644 "$root/README.md" "$share/doc/triib/README.md"
# The units name the configurations in /etc, wherever PREFIX is.
ptp4l="$root/packaging/linux/ptp4l"
for config in ptp4l-gptp.cfg ptp4l-lite.cfg; do
    install -Dm644 "$ptp4l/$config" "$destdir/etc/triib/$config"
done
for unit in triib-ptp4l-gptp@.service triib-ptp4l-lite@.service triib-link@.service \
    triib-link-reset@.service; do
    install -Dm644 "$ptp4l/$unit" "$destdir$prefix/lib/systemd/system/$unit"
done
install -Dm644 "$ptp4l/50-triib-ptp4l.rules" "$share/polkit-1/rules.d/50-triib-ptp4l.rules"
