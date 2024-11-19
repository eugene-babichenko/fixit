#!/bin/bash
set -xeuo pipefail
export RUSTFLAGS="-C instrument-coverage"
export RUST_BACKTRACE=1
TARGET_DIR="target/coverage"
export LLVM_PROFILE_FILE="$TARGET_DIR/default_%m.profraw"
cargo test --target-dir $TARGET_DIR -- --nocapture
PATH="$(rustc --print=target-libdir)/../bin:$PATH"
BINARIES="$(cargo test --no-run --message-format=json | jq -rj 'select(.profile.test == true) | .filenames[] | "--object " + . + " "')"
ARGS="-instr-profile $TARGET_DIR/fixit.profdata $BINARIES --object $PWD/target/debug/fixit"
llvm-profdata merge -sparse $TARGET_DIR/default_*.profraw -o $TARGET_DIR/fixit.profdata
llvm-cov report --use-color --ignore-filename-regex='/.cargo/registry' $ARGS
llvm-cov export -format=lcov $ARGS -sources src/{,**/}*.rs > $TARGET_DIR/fixit.lcov
rm $TARGET_DIR/*.profraw
