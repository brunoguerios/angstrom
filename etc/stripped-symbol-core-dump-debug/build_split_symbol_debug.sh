#!/bin/bash


ANG_DIR=/home/shared/angstrom
PROD_BIN=/root/prod-bin-debug

RUSTFLAGS="-C target-cpu=native -C force-frame-pointers=yes" cargo build --bin angstrom --profile maxperf-ss-debug --bin angstrom --features jemalloc --manifest-path ${ANG_DIR}/Cargo.toml -j 11


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


ulimit -c unlimited


coredumpctl gdb ${BIN} \
  --batch -ex "set pagination off" \
  -ex "thread apply all bt full" \
  -ex "info registers" \
  -ex "quit" | sed -n '1,200p'
