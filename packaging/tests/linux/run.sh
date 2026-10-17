#!/usr/bin/env bash
# Runs test-package.sh in a fresh container of IMAGE, with Docker, under
# QEMU when the package's architecture is not the host's:
#
#   packaging/tests/linux/run.sh IMAGE PACKAGE [PREVIOUS]
#   packaging/tests/linux/run.sh debian:13 dist/triib_1.1.0-1_amd64.deb
#
# TRIIB_TEST_SHOTS names a directory for the screenshot of the window.
set -euo pipefail

image=${1:?usage: run.sh IMAGE PACKAGE [PREVIOUS]}
package=$(readlink -f "${2:?usage: run.sh IMAGE PACKAGE [PREVIOUS]}")
previous=${3:+$(readlink -f "$3")}
here=$(cd "$(dirname "$0")" && pwd)

case $(basename "$package") in
*_amd64.deb | *.x86_64.rpm | *-x86_64.pkg.tar.*) platform=amd64 ;;
*_arm64.deb | *.aarch64.rpm | *-aarch64.pkg.tar.*) platform=arm64 ;;
*_riscv64.deb | *.riscv64.rpm | *-riscv64.pkg.tar.*) platform=riscv64 ;;
*) echo "run.sh: no architecture in $(basename "$package")" >&2; exit 2 ;;
esac
emulated=
[ "$platform" = "$(docker version -f '{{.Server.Arch}}')" ] || emulated=1

options=(--rm --platform "linux/$platform" -e TRIIB_TEST_EMULATED="$emulated"
    -v "$here:/tests:ro" -v "$package:/packages/$(basename "$package"):ro")
arguments=("/packages/$(basename "$package")")
if [ -n "$previous" ]; then
    options+=(-v "$previous:/packages/previous/$(basename "$previous"):ro")
    arguments+=("/packages/previous/$(basename "$previous")")
fi
if [ -n "${TRIIB_TEST_SHOTS:-}" ]; then
    mkdir -p "$TRIIB_TEST_SHOTS"
    options+=(-v "$(readlink -f "$TRIIB_TEST_SHOTS"):/shots" -e TRIIB_TEST_SHOTS=/shots)
fi
docker run "${options[@]}" "$image" /tests/test-package.sh "${arguments[@]}"
