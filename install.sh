#!/bin/bash
set -e

cd "./install"

if [ -f target/release/babydra-installer ]; then
    ./target/release/babydra-installer "$@"
else
    cargo run --release -- "$@"
fi
