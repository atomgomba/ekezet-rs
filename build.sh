#!/bin/bash
pushd lib
./build.sh
popd
cargo build --release
