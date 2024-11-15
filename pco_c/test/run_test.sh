#!/bin/bash
set -ex

CUR_DIR=$(realpath "$(dirname "${BASH_SOURCE:-$0}")")

cd "${CUR_DIR}"

cargo build
gcc -g test_cpcodec.c -o test_cpcodec -L../../target/debug -lcpcodec -Wl,-rpath,../../target/debug
./test_cpcodec
rm test_cpcodec
