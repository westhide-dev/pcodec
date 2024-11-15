#!/bin/bash
set -ex

CUR_DIR=$(realpath "$(dirname "${BASH_SOURCE:-$0}")")

cd "${CUR_DIR}"

cbindgen --config cbindgen.toml --crate cpcodec --output include/cpcodec_generated.h
