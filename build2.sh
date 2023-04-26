#!/usr/bin/env bash

set -e
PROJECT_NAME="gloam_engine"

# Build
cargo build --target wasm32-unknown-unknown --release

wasm-bindgen target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm --out-dir dist

# Shim to tie the thing together
sed -i "s/import \* as __wbg_star0 from 'env';//" dist/$PROJECT_NAME.js
sed -i "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" dist/$PROJECT_NAME.js
sed -i "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" dist/$PROJECT_NAME.js
sed -i "s/const imports = getImports();/return getImports();/" dist/$PROJECT_NAME.js
