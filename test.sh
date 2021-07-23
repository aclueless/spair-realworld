#!/bin/bash
set -ex

#cargo fmt -- --check
#cargo clippy -- -D warnings

# --chrome and --firefox on separate lines to easily disable one of them if the driver has problems
wasm-pack test --headless --chrome
wasm-pack test --headless --firefox

