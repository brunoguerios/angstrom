#!/bin/bash


ANG_DIR=/root/angstrom
PROD_BIN=/root/prod-bin
INSTALL_DEBUG_DIR=/usr/lib/debug/.build-id


RUSTFLAGS="-C target-cpu=native -C force-frame-pointers=yes" cargo build --bin angstrom --profile maxperf-ss-debug --features jemalloc --manifest-path ${ANG_DIR}/Cargo.toml -j 11

ulimit -c unlimited
systemctl restart systemd-coredump.socket

systemctl stop angstrom

BIN=${PROD_BIN}/angstrom-new
DBG=${BIN}.debug

cp ${BIN} ${PROD_BIN}/angstrom-old
cp ${ANG_DIR}/target/maxperf-ss-debug/angstrom ${BIN}

# Extract symbols to a separate file
objcopy --only-keep-debug "$BIN" "$DBG"

# Strip debug from the binary you deploy (fastest runtime, smallest RSS/I-cache)
strip --strip-debug --preserve-dates "$BIN"

# Add a link in the binary that points to the external debug file
objcopy --add-gnu-debuglink="$DBG" "$BIN"


# Install the .debug by Build-ID so coredumpctl/gdb can auto-find it
buildid=$(readelf -n "$BIN" | awk '/Build ID/ {print $3; exit}')
prefix=${buildid:0:2}; rest=${buildid:2}
install -D "$DBG" "${INSTALL_DEBUG_DIR}/${prefix}/${rest}.debug"


systemctl restart angstrom


