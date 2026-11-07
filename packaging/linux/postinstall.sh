#!/bin/sh
# Lets triib, triib-cli and triib-endpointd send and receive raw Ethernet
# without root, through triib-netraw, the one program granted CAP_NET_RAW,
# which opens their raw sockets, and has systemd read triib's units, which
# triib-endpointd runs as host entities need them, keeping EEE and PAUSE
# off the link. Says what installing changes on the system and why.
# Stops the ptp4l units older releases started, as gptpd keeps the
# network's time now, and on Debian removes their unchanged
# configurations (see preinstall.sh), then says when no interface can
# carry host entities.
if [ -n "${DPKG_MAINTSCRIPT_PACKAGE:-}" ]; then
    for config in ptp4l-gptp.cfg ptp4l-lite.cfg; do
        dpkg-maintscript-helper rm_conffile "/etc/triib/$config" 1.3.0-1~ -- "$@" || exit 1
    done
    rmdir /etc/triib 2>/dev/null || true
fi
program=/usr/lib/triib/triib-netraw
if ! setcap cap_net_raw+ep "$program"; then
    echo "triib: could not grant $program CAP_NET_RAW; run: sudo setcap cap_net_raw+ep $program" >&2
fi
cat <<'EOF'
triib: to reach AVB and Milan devices, and to run this computer's own
entities, this package
  - gives /usr/lib/triib/triib-netraw CAP_NET_RAW, so only that small
    helper opens raw network sockets, and triib, triib-cli and
    triib-endpointd run without root;
  - relies on RealtimeKit (rtkit), as PipeWire does, to run host
    entities' stream threads at realtime priority, so a busy computer
    doesn't hold up their audio;
  - installs triib-link@.service, triib-link-reset@.service and a polkit
    rule that lets you run them. When a host entity starts on an
    interface, triib-link@ keeps its frames on one transmit queue, with
    room for about 130 microseconds of the link's traffic, and turns
    Energy-Efficient Ethernet and PAUSE off, reconnecting the link once
    if they were on. triib-link-reset@ reconnects a link whose driver
    leaves PTP time stamps on received frames.
Nothing on an interface changes until a host entity starts there.
EOF
if [ -d /run/systemd/system ]; then
    systemctl daemon-reload || true
    # Started, never enabled, so one runs until stopped or a reboot.
    systemctl stop 'triib-ptp4l-gptp@*.service' 'triib-ptp4l-lite@*.service' 2>/dev/null || true
fi
/usr/lib/triib/endpoint-check || true
exit 0
