FROM debian:trixie-slim
ENV DEBIAN_FRONTEND=noninteractive
# Oldest LibreOffice the tree may be built from. trixie's point release can lag trixie-security
# (CVE-2026-63277 was fixed in 4:25.2.3-2+deb13u8 while trixie still had +deb13u6), and a cached
# apt layer keeps whatever it installed last, so the build checks rather than trusts. Raise it
# with each Debian security update to LibreOffice; changing it also re-runs the apt layer.
ARG LO_MIN_VERSION=4:25.2.3-2+deb13u8
RUN apt-get update && apt-get install -y --no-install-recommends \
      libreoffice-impress-nogui libreofficekit-dev \
      fonts-liberation fonts-crosextra-carlito fonts-crosextra-caladea fonts-dejavu-core \
      gcc libc6-dev strace poppler-utils file patchelf \
    && v=$(dpkg-query -W -f='${Version}' libreoffice-core-nogui) \
    && echo "libreoffice-core-nogui $v" \
    && if ! dpkg --compare-versions "$v" ge "$LO_MIN_VERSION"; then \
         echo "libreoffice-core-nogui $v is older than $LO_MIN_VERSION"; exit 1; fi \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /work
