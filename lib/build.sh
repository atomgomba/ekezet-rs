#!/bin/sh
rm -rf pkg
wasm-pack build --out-dir pkg --target web --out-name lib --release --no-typescript
