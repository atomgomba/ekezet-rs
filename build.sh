#!/bin/bash
pushd lib || exit
./build.sh
popd || exit
cargo build --release
