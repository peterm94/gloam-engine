use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn shell<I, S>(args: I) -> Output where I: IntoIterator<Item=S>, S: AsRef<OsStr> {
    let mut shell = if cfg!(target_os = "windows") {
        let mut sh = Command::new("cmd");
        sh.arg("/C");
        sh
    } else {
        let mut sh = Command::new("sh");
        sh.arg("-c");
        sh
    };

    shell.args(args).output().expect("Failed to execute process.")
}

fn main() {
    let project_name = "gloam_engine";

    let wasm_target = Path::new(env!("CRATE_OUT_DIR")).join(format!("{project_name}.wasm"));
    let manifest_dir = Path::new(env!("CRATE_MANIFEST_DIR"));
    let dist_dir = manifest_dir.join("dist");

    std::fs::create_dir_all(&dist_dir).unwrap();

    let result = shell(["wasm-bindgen",
        wasm_target.to_str().unwrap(),
        "--out-dir", dist_dir.to_str().unwrap(),
        "--target", "web"]);

    // Fix issues with generated files
    let js_path = dist_dir.join(format!("{project_name}.js"));
    let js_contents = fs::read_to_string(&js_path).unwrap();
    let fixed_js = js_contents.replace("import * as __wbg_star0 from 'env';", "")
        .replace("let wasm;", "let wasm; export const set_wasm = (w) => wasm = w;")
        .replace("imports['env'] = __wbg_star0;", "return imports.wbg;")
        .replace("async function init(input) {", "async function init(input) { return getImports(); } async function _unusedInit(input) {");

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
    "mq_js_bundle.js",
    "gloam_engine_bg.wasm",
    "gloam_engine_bg.wasm.d.ts"
  ],
  "module": "gloam_engine.js",
  "types": "gloam_engine.d.ts",
  "sideEffects": false
}"#;

    fs::write(dist_dir.join("package.json"),
              template.replace("VERSION", env!("CARGO_PKG_VERSION"))).unwrap();

    unsafe { println!("{}", String::from_utf8_unchecked(result.stderr)); }
}