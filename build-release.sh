#!/usr/bin/env bash

trap 'exit' ERR

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/ca_3d.wasm --out-dir html/generated --no-modules --no-typescript
mv html/generated/ca_3d_bg.wasm html/generated/ca_3d_bg_unoptimized.wasm
wasm-opt -O3 -o html/generated/ca_3d_bg.wasm html/generated/ca_3d_bg_unoptimized.wasm
rm html/generated/ca_3d_bg_unoptimized.wasm
terser -m -o html/generated/ca_3d.js html/generated/ca_3d.js
