#!/bin/sh
# Up to 1.2.0 the packages installed ptp4l configurations in /etc/triib,
# which dpkg keeps after an upgrade that no longer has them. With
# postinstall.sh and postremove.sh, this has a Debian upgrade remove them,
# keeping a changed one as .dpkg-bak; RPM removes them by itself.
if [ -n "${DPKG_MAINTSCRIPT_PACKAGE:-}" ]; then
    for config in ptp4l-gptp.cfg ptp4l-lite.cfg; do
        dpkg-maintscript-helper rm_conffile "/etc/triib/$config" 1.3.0-1~ -- "$@" || exit 1
    done
fi
exit 0
