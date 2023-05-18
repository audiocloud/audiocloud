#!/bin/bash

set -euxo pipefail

TARGET_ARCH=${TARGET_ARCH:-armv7-unknown-linux-gnueabihf}

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc

export PKG_CONFIG_DIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
export PKG_CONFIG_SYSROOT_DIR=/usr/arm-linux-gnueabihf/sysroot

cargo build --release --target="${TARGET_ARCH}" -p domain-server --bin domain_server
# rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
# ssh -t ${TARGET_HOST} ${TARGET_PATH}