#!/usr/bin/env bash

set -e # error -> trap -> exit
function info() { echo -e "[\033[0;34m $@ \033[0m]"; } # blue: [ info message ]
function pass() { echo -e "[\033[0;32mPASS\033[0m] $@"; } # green: [PASS]
function fail() { FAIL="true"; echo -e "[\033[0;31mFAIL\033[0m] $@"; } # red: [FAIL]
trap 'LASTRES=$?; LAST=$BASH_COMMAND; if [[ LASTRES -ne 0 ]]; then fail "Command: \"$LAST\" exited with exit code: $LASTRES"; elif [ "$FAIL" == "true"  ]; then fail finished with error; else pass "finished";fi' EXIT
SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir

cd $SRCDIR # ensure current dir is this dir

EXAMPLE=$1
TARGET_DIR=../target/wasm32-unknown-unknown/debug
if [ ! -z "$CARGO_TARGET_DIR" ]; then 
    TARGET_DIR=$CARGO_TARGET_DIR/wasm32-unknown-unknown/debug
fi 

if [ -d $EXAMPLE ]; then
    cd $EXAMPLE
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen --target web --no-typescript --out-dir $SRCDIR/static/ --out-name wasm $TARGET_DIR/example$EXAMPLE.wasm
fi
