#!/usr/bin/env bash
# Installs a triib package in a fresh container, as root, and checks it as
# someone installing it would meet it: its dependencies alone are enough,
# the files and capabilities it sets up, each program, raw Ethernet as a
# regular user, the window under a headless Wayland compositor, and what
# removing it leaves behind. run.sh beside it starts the container.
#
#   test-package.sh PACKAGE [PREVIOUS]
#
# PACKAGE is a .deb, .rpm or Arch package. PREVIOUS, an older package of
# the same kind, goes in first, so PACKAGE upgrades it.
# TRIIB_TEST_EMULATED=1, for an architecture running under QEMU, leaves
# out raw Ethernet and the window. TRIIB_TEST_SHOTS names a directory for
# a screenshot of the window.
set -euo pipefail

package=$(readlink -f "${1:?usage: test-package.sh PACKAGE [PREVIOUS]}")
previous=${2:+$(readlink -f "$2")}
emulated=${TRIIB_TEST_EMULATED:-}
shots=${TRIIB_TEST_SHOTS:-}
id=run.triib
programs=(triib triib-cli triib-endpointd)

section() { printf '\n== %s\n' "$*"; }
fail() {
    printf 'FAIL: %s\n' "$*" >&2
    exit 1
}

case $package in
*.deb) kind=deb ;;
*.rpm) kind=rpm ;;
*.pkg.tar.*) kind=arch ;;
*) fail "not a package: $package" ;;
esac

# Only what each package depends on, none of what it recommends, so a
# dependency left out of a package shows.
install_package() {
    case $kind in
    deb) apt-get install -y --no-install-recommends -o Dpkg::Options::=--force-confold "$1" ;;
    rpm) dnf install -y --setopt=install_weak_deps=False "$1" ;;
    arch) pacman -U --noconfirm "$1" ;;
    esac
}
add_tools() {
    case $kind in
    deb) apt-get install -y --no-install-recommends "$@" ;;
    rpm) dnf install -y --setopt=install_weak_deps=False "$@" ;;
    arch) pacman -S --noconfirm --needed "$@" ;;
    esac
}
remove_package() {
    case $kind in
    deb) apt-get purge -y triib ;;
    rpm) dnf remove -y triib ;;
    arch) pacman -Rns --noconfirm triib ;;
    esac
}
installed_version() {
    case $kind in
    deb) dpkg-query -W -f '${Version}' triib ;;
    rpm) rpm -q --qf '%{VERSION}-%{RELEASE}' triib ;;
    arch) pacman -Q triib | cut -d' ' -f2 ;;
    esac
}
package_version() {
    case $kind in
    deb) dpkg-deb -f "$1" Version ;;
    rpm) rpm -qp --qf '%{VERSION}-%{RELEASE}' "$1" ;;
    arch) pacman -Qp "$1" | cut -d' ' -f2 ;;
    esac
}

section "$(. /etc/os-release && echo "$PRETTY_NAME"), $(uname -m)"
case $kind in
deb)
    export DEBIAN_FRONTEND=noninteractive
    apt-get update -q
    ;;
arch) pacman -Syu --noconfirm ;;
esac

if [ -n "$previous" ]; then
    section "Install $(basename "$previous"), to upgrade from"
    install_package "$previous"
    # A configuration the user changed must survive the upgrade.
    echo '# changed before the upgrade' >>/etc/triib/ptp4l-gptp.cfg
fi

section "Install $(basename "$package")"
install_package "$package" 2>&1 | tee /tmp/install.log
want=$(package_version "$package")
[ "$(installed_version)" = "$want" ] || fail "installed $(installed_version), not $want"
version=${want%-*}
if [ -n "$previous" ]; then
    grep -qx '# changed before the upgrade' /etc/triib/ptp4l-gptp.cfg ||
        fail "the upgrade replaced the changed /etc/triib/ptp4l-gptp.cfg"
fi

section Files
for program in "${programs[@]}"; do
    [ -x "/usr/bin/$program" ] || fail "/usr/bin/$program is missing"
done
[ -x /usr/lib/triib/endpoint-check ] || fail "/usr/lib/triib/endpoint-check is missing"
for file in \
    "/usr/share/applications/$id.desktop" \
    "/usr/share/metainfo/$id.metainfo.xml" \
    "/usr/share/icons/hicolor/512x512/apps/$id.png" \
    "/usr/share/icons/hicolor/scalable/apps/$id.svg" \
    /usr/share/licenses/triib/LICENSE-MIT \
    /usr/share/licenses/triib/LICENSE-APACHE \
    /usr/share/licenses/triib/fonts/OFL.txt \
    /usr/lib/systemd/system/triib-ptp4l-gptp@.service \
    /usr/lib/systemd/system/triib-ptp4l-lite@.service \
    /usr/lib/systemd/system/triib-link@.service \
    /usr/lib/systemd/system/triib-link-reset@.service \
    /usr/share/polkit-1/rules.d/50-triib-ptp4l.rules \
    /etc/triib/ptp4l-gptp.cfg \
    /etc/triib/ptp4l-lite.cfg; do
    [ -f "$file" ] || fail "$file is missing"
done
echo "all there"

# Only the helper holds the capability; the programs get their raw
# sockets from it.
section Capabilities
capabilities=$(getcap /usr/lib/triib/triib-netraw)
echo "$capabilities"
grep -Eq 'cap_net_raw[+=]ep' <<<"$capabilities" || fail "/usr/lib/triib/triib-netraw lacks CAP_NET_RAW"
for program in "${programs[@]}"; do
    [ -z "$(getcap "/usr/bin/$program")" ] || fail "/usr/bin/$program holds a capability"
done

# A container's interfaces have no PTP hardware clock, so installing must
# have said so.
section "No interface for host entities"
grep -q 'requires an AVB Ethernet' /tmp/install.log ||
    fail "installing did not say that no interface can carry host entities"
echo "said so as it installed"

section "Shared libraries"
for program in "${programs[@]}"; do
    missing=$(ldd "/usr/bin/$program" | grep 'not found' || true)
    [ -z "$missing" ] || fail "/usr/bin/$program needs libraries the package does not bring: $missing"
done
echo "all found"

section "Each program, as a regular user"
command -v runuser >/dev/null || add_tools util-linux >/dev/null
useradd -m tester
as_tester() { runuser -u tester -- "$@"; }
for program in "${programs[@]}"; do
    printed=$(as_tester "/usr/bin/$program" --version)
    echo "$printed"
    [ "$printed" = "$program $version" ] || fail "$program --version printed '$printed', not '$program $version'"
done

if [ -z "$emulated" ]; then
    section "Raw Ethernet, as a regular user"
    listing=$(as_tester triib-cli interfaces)
    echo "$listing"
    grep -qx 'raw Ethernet: ready' <<<"$listing" || fail "triib-cli has no raw Ethernet as a regular user"
    interface=$(sed -n '2{s/ .*//;p}' <<<"$listing")
    if [ -n "$interface" ] && [ "$interface" != "no" ]; then
        as_tester triib-cli discover "$interface" 1 || fail "triib-cli could not discover on $interface"
    fi

    # sway, as its IPC names the windows open, and grim for the picture.
    section "The window, under a headless Wayland compositor"
    case $kind in
    deb) add_tools sway grim procps >/dev/null ;;
    rpm) add_tools sway grim procps-ng >/dev/null ;;
    arch) add_tools sway grim procps-ng >/dev/null ;;
    esac
    # Arch's sway carries CAP_SYS_NICE, which a container cannot grant,
    # so it would not start at all.
    setcap -r "$(command -v sway)" 2>/dev/null || true
    runtime=/tmp/runtime-tester
    install -d -m 700 -o tester -g tester "$runtime"
    printf 'output HEADLESS-1 resolution 1280x800\n' >/tmp/sway.conf
    session=(env XDG_RUNTIME_DIR="$runtime" WLR_BACKENDS=headless WLR_RENDERER=pixman
        WLR_LIBINPUT_NO_DEVICES=1)
    # Older sway will not start where NVIDIA's driver is loaded, which a
    # container sees of its host.
    as_tester "${session[@]}" sway --unsupported-gpu -c /tmp/sway.conf >/tmp/sway.log 2>&1 &
    for _ in $(seq 50); do
        display=$(cd "$runtime" && ls wayland-[0-9] 2>/dev/null | head -1 || true)
        ipc=$(ls "$runtime"/sway-ipc.*.sock 2>/dev/null | head -1 || true)
        [ -n "$display" ] && [ -n "$ipc" ] && break
        sleep 0.2
    done
    [ -n "$display" ] && [ -n "$ipc" ] || {
        cat /tmp/sway.log
        fail "sway did not start"
    }
    session+=(WAYLAND_DISPLAY="$display" SWAYSOCK="$ipc")
    as_tester "${session[@]}" triib >/tmp/triib.log 2>&1 &
    shown=
    for _ in $(seq 60); do
        if as_tester "${session[@]}" swaymsg -t get_tree | grep -q "\"app_id\": \"$id\""; then
            shown=1
            break
        fi
        pgrep -x -u tester triib >/dev/null || break
        sleep 0.5
    done
    [ -n "$shown" ] || {
        cat /tmp/triib.log
        fail "triib's window did not open"
    }
    # Running a few seconds on, and drawing more than a blank window.
    sleep 5
    pgrep -x -u tester triib >/dev/null || {
        cat /tmp/triib.log
        fail "triib quit after opening its window"
    }
    as_tester "${session[@]}" grim -t ppm /tmp/window.ppm
    if [ -n "$shots" ]; then
        as_tester "${session[@]}" grim -t png - >"$shots/$(. /etc/os-release && echo "$ID-${VERSION_ID:-rolling}")-$(uname -m)-$kind.png"
    fi
    values=$(tail -c +100 /tmp/window.ppm | od -An -v -tu1 | tr -s ' ' '\n' | sort -u | wc -l)
    [ "$values" -gt 32 ] || fail "triib's window looks blank"
    echo "the window opened and drew"
    pkill -x -u tester triib || true
    pkill -x -u tester sway || true
fi

section Remove
remove_package
leftovers=()
for path in /usr/bin/triib /usr/bin/triib-cli /usr/bin/triib-endpointd /usr/lib/triib \
    "/usr/share/applications/$id.desktop" "/usr/share/metainfo/$id.metainfo.xml" \
    /usr/lib/systemd/system/triib-link@.service /usr/share/polkit-1/rules.d/50-triib-ptp4l.rules \
    /etc/triib/ptp4l-lite.cfg; do
    [ -e "$path" ] && leftovers+=("$path")
done
# Purging a Debian package, or removing with pacman's -n, takes the
# configuration too; RPM keeps a changed one as .rpmsave.
if [ "$kind" != rpm ] && [ -e /etc/triib ]; then
    leftovers+=(/etc/triib)
fi
[ ${#leftovers[@]} -eq 0 ] || fail "removing left ${leftovers[*]}"
echo "nothing left"

section "Passed: $(basename "$package")"
