use std::fs;
use std::path::Path;

use wasm_bindgen_cli_support::Bindgen;

fn main() {
    let project_name = "gloam_engine";

    let wasm_target = Path::new(env!("CRATE_OUT_DIR")).join(format!("{project_name}.wasm"));
    let manifest_dir = Path::new(env!("CRATE_MANIFEST_DIR"));
    let dist_dir = manifest_dir.join("dist");

    fs::create_dir_all(&dist_dir).unwrap();

    Bindgen::new()
        .input_path(wasm_target)
        .debug(true)
        .typescript(true)
        .reference_types(true)
        .web(true).unwrap()
        .generate(&dist_dir)
        .expect("wasm-bindgen failed.");

    // Fix issues with generated files
    let js_path = dist_dir.join(format!("{project_name}.js"));
    let js_contents = fs::read_to_string(&js_path).unwrap();
    let fixed_js = js_contents.replace("import * as __wbg_star0 from 'env';", "")
        .replace("let wasm;", "let wasm; export const set_wasm = (w) => wasm = w;")
        .replace("imports['env'] = __wbg_star0;", "return imports.wbg;")
        .replace("async function __wbg_init(input) {", "async function __wbg_init(input) { return __wbg_get_imports(); } async function _unusedInit(input) {");

    let mq_js_bundle = fs::read_to_string(manifest_dir.join("js").join("mq_js_bundle.js")).unwrap();
    let fixed_js = fixed_js + &mq_js_bundle;
    fs::write(&js_path, fixed_js).unwrap();

    // Output a package.json
    let template = r#"{
  "name": "gloam-engine",
  "version": "VERSION",
  "files": [
    "gloam_engine.d.ts",
    "gloam_engine.js",
    "gloam_engine_bg.wasm",
    "gloam_engine_bg.wasm.d.ts"
  ],
  "module": "gloam_engine.js",
  "types": "gloam_engine.d.ts",
  "sideEffects": false
}"#;

    fs::write(dist_dir.join("package.json"),
              template.replace("VERSION", env!("CARGO_PKG_VERSION"))).unwrap();
}