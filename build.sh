#!/usr/bin/env bash

set -e
PROJECT_NAME="gloam_engine"

# Build
cargo build --target wasm32-unknown-unknown --release

wasm-bindgen target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm --out-dir dist --target web

# Shim to tie the thing together
sed -i "s/import \* as __wbg_star0 from 'env';//" dist/$PROJECT_NAME.js
sed -i "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" dist/$PROJECT_NAME.js
sed -i "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" dist/$PROJECT_NAME.js
#sed -i "s/const imports = getImports();/return getImports();/" dist/$PROJECT_NAME.js
sed -i "s/async function init(input) {/async function init(input) { return getImports(); } async function _unusedInit(input) {/" dist/$PROJECT_NAME.js
# package json
PACKAGE=$(cat <<- END
{
  "name": "gloam-engine",
  "version": "0.0.3",
  "files": [
    "gloam_engine.d.ts",
    "gloam_engine.js",
    "gloam_engine_bg.wasm",
    "gloam_engine_bg.wasm.d.ts"
  ],
  "module": "gloam_engine.js",
  "types": "gloam_engine.d.ts",
  "sideEffects": false
}
END
)

echo "$PACKAGE" > dist/package.json