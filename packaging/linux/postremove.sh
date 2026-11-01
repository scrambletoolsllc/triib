#!/bin/sh
# The ptp4l configurations older releases installed, as preinstall.sh
# says: purging takes a changed one kept as .dpkg-bak, and their folder.
if [ -n "${DPKG_MAINTSCRIPT_PACKAGE:-}" ]; then
    for config in ptp4l-gptp.cfg ptp4l-lite.cfg; do
        dpkg-maintscript-helper rm_conffile "/etc/triib/$config" 1.3.0-1~ -- "$@" || exit 1
    done
    if [ "$1" = purge ]; then
        rmdir /etc/triib 2>/dev/null || true
    fi
fi
exit 0
