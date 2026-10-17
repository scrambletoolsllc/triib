#!/bin/bash
# Installs triib's package on this Mac and checks it as someone installing
# it would meet it: Gatekeeper's verdict on a downloaded copy, libraries
# that only macOS itself provides, what the package sets up, BPF access
# for the user, the window, and what the uninstall script leaves behind.
# For a Mac with nothing to lose, such as a CI runner: it replaces any
# installed triib.app and removes it at the end, and needs sudo.
#
#   packaging/tests/macos/test-pkg.sh PACKAGE [PREVIOUS]
#
# PREVIOUS, an older package, goes in first, so PACKAGE upgrades it.
# TRIIB_EXPECT_SIGNED=true also asks for Scramble Tools' Developer ID
# signatures and notarization.
set -euo pipefail

package=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
previous=${2:+$(cd "$(dirname "$2")" && pwd)/$(basename "$2")}
id=io.github.scrambletools.triib
app=/Applications/triib.app
daemon=/Library/LaunchDaemons/$id.bpf.plist
support="/Library/Application Support/triib"
console=$(stat -f%Su /dev/console)

section() { printf '\n== %s\n' "$*"; }
fail() {
    printf 'FAIL: %s\n' "$*" >&2
    exit 1
}
# Whether version $1 is at most $2, both as dotted numbers.
at_most() {
    awk -v a="$1" -v b="$2" 'BEGIN {
        n = split(a, x, "."); m = split(b, y, ".")
        for (i = 1; i <= (n > m ? n : m); i++) {
            if (x[i] + 0 < y[i] + 0) exit 0
            if (x[i] + 0 > y[i] + 0) exit 1
        }
        exit 0
    }'
}

section "macOS $(sw_vers -productVersion), $(uname -m)"
work=$(mktemp -d)
pkgutil --expand-full "$package" "$work/expanded"
want=$(xmllint --xpath 'string(//pkg-ref[@version]/@version)' "$work/expanded/Distribution")
bundle="$work/expanded/$id.pkg/Payload/Applications/triib.app"
[ -d "$bundle" ] || fail "the package has no triib.app"
echo "triib $want"

section "Gatekeeper, for a downloaded copy"
if [ "${TRIIB_EXPECT_SIGNED:-}" = true ]; then
    mkdir "$work/downloaded"
    cp "$package" "$work/downloaded/"
    copy="$work/downloaded/$(basename "$package")"
    xattr -w com.apple.quarantine "0081;$(printf %x "$(date +%s)");Safari;" "$copy"
    spctl --assess --type install --verbose=2 "$copy" || fail "Gatekeeper rejects the downloaded package"
    pkgutil --check-signature "$copy" | grep -q 'Notarization: trusted by the Apple notary service' ||
        fail "the package is not notarized"
else
    echo "this build is not signed, so skipped"
fi

# Any library outside macOS, such as one from Homebrew on the build
# machine, would be missing on another Mac; and the programs must run on
# the oldest macOS the app claims.
section "Libraries and the oldest macOS"
minimum=$(plutil -extract LSMinimumSystemVersion raw "$bundle/Contents/Info.plist")
problems=()
while IFS= read -r -d '' binary; do
    file -b "$binary" | grep -q Mach-O || continue
    name=${binary#"$bundle/"}
    while read -r library; do
        case $library in
        /usr/lib/* | /System/Library/*) ;;
        *) problems+=("$name links $library, which macOS does not have") ;;
        esac
    done < <(otool -L "$binary" | tail -n +2 | awk '{print $1}')
    lipo -archs "$binary" | grep -qw arm64 || problems+=("$name has no arm64 code")
    minos=$(vtool -show-build "$binary" | awk '$1 == "minos" {print $2; exit}')
    echo "$name: macOS $minos and later"
    at_most "$minos" "$minimum" || problems+=("$name needs macOS $minos, the app claims $minimum")
done < <(find "$bundle/Contents/MacOS" -type f -print0)
[ ${#problems[@]} -eq 0 ] || fail "$(printf '\n  %s' "${problems[@]}")"

if [ -n "$previous" ]; then
    section "Install $(basename "$previous"), to upgrade from"
    sudo installer -pkg "$previous" -target /
fi

section "Install $(basename "$package")"
sudo installer -pkg "$package" -target / || {
    tail -n 60 /var/log/install.log
    fail "the installer failed"
}
installed=$(pkgutil --pkg-info "$id" | awk '$1 == "version:" {print $2}')
[ "$installed" = "$want" ] || fail "installed $installed, not $want"

section "What it installed"
[ -d "$app" ] || fail "$app is missing"
codesign --verify --strict "$app" || fail "the installed app's signature does not hold"
[ "$(plutil -extract CFBundleShortVersionString raw "$app/Contents/Info.plist")" = "$want" ] ||
    fail "the installed app is not $want"
for program in triib triib-cli; do
    printed=$("$app/Contents/MacOS/$program" --version)
    echo "$printed"
    [ "$printed" = "$program $want" ] || fail "$program --version printed '$printed'"
done
[ "$(readlink /usr/local/bin/triib-cli)" = "$app/Contents/MacOS/triib-cli" ] ||
    fail "/usr/local/bin/triib-cli does not lead to the app's"
sudo launchctl print "system/$id.bpf" >/dev/null || fail "the BPF launch daemon is not loaded"
# The daemon runs as it loads, a moment after the installer is done.
for _ in $(seq 40); do
    [ "$(stat -f '%Sg' /dev/bpf0)" = access_bpf ] && break
    sleep 0.5
done
for device in /dev/bpf*; do
    [ "$(stat -f '%Sg' "$device")" = access_bpf ] || fail "$device is not in access_bpf"
    mode=$(stat -f '%Lp' "$device")
    [ $((8#$mode & 8#060)) -eq $((8#060)) ] || fail "$device is not open to access_bpf"
done
dseditgroup -o checkmember -m "$console" access_bpf || fail "$console did not join access_bpf"

# sudo -u takes the groups from the directory, as a new login would.
section "BPF, as $console"
listing=$(sudo -u "$console" /usr/local/bin/triib-cli interfaces)
echo "$listing"
grep -qx 'raw Ethernet: ready' <<<"$listing" || fail "triib-cli has no BPF access as $console"
interface=$(sed -n '2{s/ .*//;p;}' <<<"$listing")
if [ -n "$interface" ] && [ "$interface" != no ]; then
    sudo -u "$console" /usr/local/bin/triib-cli discover "$interface" 1 ||
        fail "triib-cli could not discover on $interface"
fi

# The window server names each window's owner without any permission.
section "The window"
cat >"$work/windows.swift" <<'SWIFT'
import CoreGraphics
import Foundation
let options: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
for window in CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] ?? [] {
    if let owner = window[kCGWindowOwnerName as String] as? String { print(owner) }
}
SWIFT
xcrun swiftc -O -o "$work/windows" "$work/windows.swift"
sudo -u "$console" open "$app"
shown=
for _ in $(seq 60); do
    if "$work/windows" | grep -qx triib; then
        shown=1
        break
    fi
    sleep 0.5
done
[ -n "$shown" ] || fail "triib's window did not open"
sleep 5
pid=$(pgrep -x triib) || fail "triib quit after opening its window"
echo "the window opened"
kill "$pid"

section "Uninstall"
sudo "$support/uninstall"
leftovers=()
for path in "$app" "$daemon" /usr/local/bin/triib-cli "$support"; do
    [ -e "$path" ] || [ -L "$path" ] && leftovers+=("$path")
done
sudo launchctl print "system/$id.bpf" >/dev/null 2>&1 && leftovers+=("the launch daemon")
pkgutil --pkgs | grep -qx "$id" && leftovers+=("the package receipt")
[ ${#leftovers[@]} -eq 0 ] || fail "uninstalling left ${leftovers[*]}"
echo "nothing left"

section "Passed: $(basename "$package")"
