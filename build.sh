#!/bin/bash
set -ex

CUR_DIR=$(realpath "$(dirname "${BASH_SOURCE:-$0}")")

function Build() {
    cd "${CUR_DIR}"

    cargo build --release

    cmake -S . -B build -DSANITIZE=address
    cmake --build build
}

function Run() {
    cd "${CUR_DIR}"
    build/pco_clib/src/main
}

Build

[ "$1" == "run" ] && Run
