#!/usr/bin/env bash
# Builds the Arch package from packaging/arch/triib, as someone using the
# PKGBUILD would, but from SOURCE, an archive of this tree, rather than
# the tag's, in a fresh Arch container. run.sh then tests the package.
#
#   git archive --prefix=triib-1.1.1/ -o triib-1.1.1.tar.gz HEAD
#   docker run --rm -v "$PWD:/work" -w /work archlinux:latest \
#       packaging/tests/linux/build-arch.sh triib-1.1.1.tar.gz dist-arch
#
# The archive's name and folder must carry the PKGBUILD's pkgver.
set -euo pipefail

source_archive=$(readlink -f "${1:?usage: build-arch.sh SOURCE OUT}")
out=${2:?usage: build-arch.sh SOURCE OUT}
recipe=$(cd "$(dirname "$0")/../../arch/triib" && pwd)

pacman -Syu --noconfirm --needed base-devel
# makepkg runs as a regular user, so the dependencies go in first.
# shellcheck source=/dev/null disable=SC2154
(. "$recipe/PKGBUILD" && pacman -S --noconfirm --needed "${depends[@]}" "${makedepends[@]}")
useradd -m builder
build=$(mktemp -d)
cp "$recipe/PKGBUILD" "$recipe/triib.install" "$source_archive" "$build/"
chown -R builder "$build"
mkdir -p "$out"
(cd "$build" && runuser -u builder -- makepkg --nocheck --noconfirm)
cp "$build"/triib-[0-9]*.pkg.tar.* "$out/"
ls -l "$out"
