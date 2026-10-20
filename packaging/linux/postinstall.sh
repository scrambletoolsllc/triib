#!/bin/sh
# Lets triib, triib-cli and triib-endpointd send and receive raw Ethernet
# without root, through triib-netraw, the one program granted CAP_NET_RAW,
# which opens their raw sockets, and has systemd read triib's units, which
# triib-endpointd
# starts as host entities need them: ptp4l for the network's time, and
# EEE and PAUSE off the link. Then says when no interface can carry host
# entities.
program=/usr/lib/triib/triib-netraw
if ! setcap cap_net_raw+ep "$program"; then
    echo "triib: could not grant $program CAP_NET_RAW; run: sudo setcap cap_net_raw+ep $program" >&2
fi
if [ -d /run/systemd/system ]; then
    systemctl daemon-reload || true
fi
/usr/lib/triib/endpoint-check || true
exit 0
