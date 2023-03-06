# syntax = docker/dockerfile:1.3

ARG RUST_VERSION=1.67

# rust source compile
FROM --platform=$BUILDPLATFORM rust:$RUST_VERSION-bullseye as builder

ARG BUILDPLATFORM
ARG BUILDOS
ARG BUILDARCH
ARG BUILDVARIANT
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT
ARG RUST_TOOLCHAIN
ARG VERSION=0.1.0
ARG CARGO_FLAGS="--release --locked"

COPY . .

RUN cargo build $CARGO_FLAGS && \
    # Copy executable out of the cache so it is available in the runtime image.
    mkdir -p /tmp/gh-pilot && \
    find target -type f \( -name "ghp-server" -o -name "ghp" \) -exec cp -v {} /tmp/gh-pilot/ \;

# Create runtime base minimal image for the target platform executables
FROM --platform=$TARGETPLATFORM bitnami/minideb:bullseye as runtime

ARG BUILDPLATFORM
ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT

ARG VERSION

ENV dockerfile_version=$VERSION
ENV dockerfile_build_arch=$BUILDPLATFORM

# Disable Prompt During Packages Installation
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get --no-install-recommends install -y \
      apt-transport-https \
      ca-certificates \
      openssl

RUN groupadd --gid 10001 gh-pilot && \
    useradd --create-home --no-log-init \
      --uid 10000 --gid 10001 gh-pilot

USER gh-pilot:gh-pilot

COPY --from=builder /tmp/gh-pilot/ghp-server /usr/local/bin/ghp-server
COPY --from=builder /tmp/gh-pilot/ghp /usr/local/bin/ghp-cli

EXPOSE 8330

ENTRYPOINT [ "ghp-server" ]
#CMD [ "--non-interactive-mode" ]
