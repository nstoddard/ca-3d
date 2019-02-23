#!/usr/bin/env bash

trap 'exit' ERR

cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/ca_3d.wasm --out-dir html --no-modules --no-typescript --debug
