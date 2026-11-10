#!/usr/bin/env bash
# Installs a built triib and triib-cli and their data files into a system
# layout, for packages: the binaries, desktop entry, icons when there are
# any, AppStream metadata and licenses, and the check the packages run as
# they install for an interface that can carry host entities. Host
# entities run in avb-endpointd, which installs with its own
# scripts/dist-install.sh, as do avb-netraw, the one program granted
# CAP_NET_RAW, which opens triib's raw sockets, and the link units.
#
#   scripts/dist-install.sh BINARY_DIR [DESTDIR] [PREFIX]
#
# BINARY_DIR holds the two programs from a release build made with
# TRIIB_PRODUCTION=1. DESTDIR is the staging root (empty for a direct
# install) and PREFIX defaults to /usr.
set -euo pipefail

binaries=${1:?usage: dist-install.sh BINARY_DIR [DESTDIR] [PREFIX]}
destdir=${2:-}
prefix=${3:-/usr}
root=$(cd "$(dirname "$0")/.." && pwd)
id=run.triib
share="$destdir$prefix/share"

install -Dm755 "$binaries/triib" "$destdir$prefix/bin/triib"
install -Dm755 "$binaries/triib-cli" "$destdir$prefix/bin/triib-cli"
install -Dm755 "$root/packaging/linux/endpoint-check" "$destdir$prefix/lib/triib/endpoint-check"
install -Dm644 "$root/data/$id.desktop" "$share/applications/$id.desktop"
install -Dm644 "$root/data/$id.metainfo.xml" "$share/metainfo/$id.metainfo.xml"
if [ -d "$root/data/icons" ]; then
    (cd "$root/data/icons" && find hicolor -type f) | while read -r icon; do
        install -Dm644 "$root/data/icons/$icon" "$share/icons/$icon"
    done
fi
# triib's model for avb-endpointd, which its entities show controllers:
# with triib's version, and triib's app icon as its logo.
models="$share/avb-endpointd/models"
version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$root/Cargo.toml" | head -n 1)
icon="$root/data/icons/hicolor/128x128/apps/$id.png"
install -d "$models"
{
    cat "$root/packaging/linux/models/triib.toml"
    echo "firmware_version = \"$version\""
    if [ -f "$icon" ]; then
        echo 'logo = "triib.png"'
    fi
} >"$models/triib.toml"
chmod 0644 "$models/triib.toml"
if [ -f "$icon" ]; then
    install -m644 "$icon" "$models/triib.png"
fi
for license in LICENSE-MIT LICENSE-APACHE; do
    install -Dm644 "$root/$license" "$share/licenses/triib/$license"
done
for font_license in OFL.txt LICENSE-MaterialSymbols.txt; do
    install -Dm644 "$root/packaging/licenses/$font_license" \
        "$share/licenses/triib/fonts/$font_license"
done
install -Dm644 "$root/README.md" "$share/doc/triib/README.md"
