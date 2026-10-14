#!/bin/bash
# Builds triib.app on macOS, with triib-cli beside the app's own binary,
# and with --pkg an installer package that also sets up capture access.
# The app is signed ad hoc, which Apple Silicon needs to run it at all; a
# Developer ID signature and notarization come later (docs/RELEASING.md).
#
#   packaging/macos/bundle.sh [--pkg] [--out DIR]
#
# triib reaches the network through the BPF devices, which are root's
# alone at every start. The package installs a launch daemon that opens
# them to the access_bpf group at each start, as Wireshark's ChmodBPF
# does with the same group, and adds the user installing it to the group.
#
# TRIIB_PRODUCTION=1 makes a release build, as in CI; without it the app
# is the development build, with its own settings and cache.
#
# TRIIB_SIGN_IDENTITY signs with that identity rather than ad hoc, from
# the keychain TRIIB_SIGN_KEYCHAIN when given, unlocked with
# TRIIB_SIGN_KEYCHAIN_PASSWORD. An ad hoc signature changes with every
# build, so macOS asks again for the microphone and audio recording; one
# that stays, such as a local certificate's, keeps what was allowed.
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
here="$root/packaging/macos"
out="$root/target/macos"
pkg=false
while [ $# -gt 0 ]; do
    case "$1" in
        --pkg) pkg=true ;;
        --out) out=$2; shift ;;
        *) echo "unknown option: $1" >&2; exit 2 ;;
    esac
    shift
done

version=$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)
if [ -n "${TRIIB_PRODUCTION:-}" ]; then
    name=triib; id=io.github.scrambletools.triib
else
    name=triib-dev; id=io.github.scrambletools.triib.dev
fi

cargo build --release --locked -p triib -p triib-cli --manifest-path "$root/Cargo.toml"

app="$out/$name.app"
rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources/licenses/fonts"
cp "$root/target/release/triib" "$root/target/release/triib-cli" "$app/Contents/MacOS/"
cp "$root/LICENSE-MIT" "$root/LICENSE-APACHE" "$app/Contents/Resources/licenses/"
cp "$root"/packaging/licenses/* "$app/Contents/Resources/licenses/fonts/"

# The icon, when there is one: each size an .iconset asks for, from the
# PNGs the Linux packages use and the 1024 one they are made from.
icons="$root/data/icons/hicolor"
png() { echo "$icons/$1x$1/apps/io.github.scrambletools.triib.png"; }
icon_entry=""
if [ -e "$(png 512)" ]; then
    iconset=$(mktemp -d)/triib.iconset
    mkdir -p "$iconset"
    for size in 16 32 128 256 512; do
        cp "$(png $size)" "$iconset/icon_${size}x${size}.png"
    done
    cp "$(png 32)" "$iconset/icon_16x16@2x.png"
    cp "$(png 64)" "$iconset/icon_32x32@2x.png"
    cp "$(png 256)" "$iconset/icon_128x128@2x.png"
    cp "$(png 512)" "$iconset/icon_256x256@2x.png"
    cp "$root/data/icons/triib-1024.png" "$iconset/icon_512x512@2x.png"
    iconutil -c icns "$iconset" -o "$app/Contents/Resources/triib.icns"
    icon_entry="<key>CFBundleIconFile</key><string>triib</string>"
fi

cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key><string>$name</string>
    <key>CFBundleDisplayName</key><string>$name</string>
    <key>CFBundleIdentifier</key><string>$id</string>
    <key>CFBundleExecutable</key><string>triib</string>
    $icon_entry
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>$version</string>
    <key>CFBundleVersion</key><string>$version</string>
    <key>LSMinimumSystemVersion</key><string>11.0</string>
    <key>LSApplicationCategoryType</key><string>public.app-category.music</string>
    <key>NSHighResolutionCapable</key><true/>
    <key>NSAudioCaptureUsageDescription</key><string>triib plays what apps send to an audio device, such as this Mac's own AVB streams, through an output you pick, when you ask it to listen.</string>
    <key>NSMicrophoneUsageDescription</key><string>triib plays an audio input, such as this Mac's own AVB streams, through an output you pick, when you ask it to listen.</string>
    <key>NSHumanReadableCopyright</key><string>MIT OR Apache-2.0</string>
</dict>
</plist>
PLIST
plutil -lint "$app/Contents/Info.plist" >/dev/null

# Extended attributes would go into the package as AppleDouble files.
xattr -cr "$app"
sign=(--sign "${TRIIB_SIGN_IDENTITY:--}")
if [ -n "${TRIIB_SIGN_KEYCHAIN:-}" ]; then
    security unlock-keychain -p "${TRIIB_SIGN_KEYCHAIN_PASSWORD:-}" "$TRIIB_SIGN_KEYCHAIN"
    sign+=(--keychain "$TRIIB_SIGN_KEYCHAIN")
fi
codesign --force --deep "${sign[@]}" "$app"
echo "$app"

if $pkg; then
    payload=$(mktemp -d)
    mkdir -p "$payload/Applications" "$payload/Library/LaunchDaemons" \
        "$payload/Library/Application Support/triib"
    cp -R "$app" "$payload/Applications/"
    cp "$here/io.github.scrambletools.triib.bpf.plist" "$payload/Library/LaunchDaemons/"
    install -m 755 "$here/bpf-access" "$here/uninstall" "$payload/Library/Application Support/triib/"
    scripts=$(mktemp -d)
    install -m 755 "$here/postinstall" "$scripts/postinstall"
    # The app goes to /Applications even when a copy is elsewhere.
    components=$(mktemp -d)/components.plist
    pkgbuild --analyze --root "$payload" "$components" >/dev/null
    plutil -replace 0.BundleIsRelocatable -bool false "$components"
    package="$out/$name-$version-$(uname -m).pkg"
    rm -f "$package"
    pkgbuild --root "$payload" --component-plist "$components" --scripts "$scripts" \
        --identifier "$id" --version "$version" --install-location / "$package" >/dev/null
    echo "$package"
fi
