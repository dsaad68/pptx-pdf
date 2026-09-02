FROM debian:trixie-slim
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
      libreoffice-impress-nogui libreofficekit-dev \
      fonts-liberation fonts-crosextra-carlito fonts-crosextra-caladea fonts-dejavu-core \
      gcc libc6-dev strace poppler-utils file patchelf \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /work
