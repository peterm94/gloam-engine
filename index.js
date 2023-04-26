// import init, { set_wasm } from "./gloam_engine.js";
// import startApp from "./app.js";
// async function impl_run() {
//     let wbg = await init();
//     miniquad_add_plugin({
//         register_plugin: (a) => (a.wbg = wbg),
//         on_init: () => {
//             set_wasm(wasm_exports)
//             startApp()
//         },
//         version: "0.0.1",
//         name: "wbg",
//     });
//     load("./gloam_engine_bg.wasm");
// }
window.run = function() {
    document.getElementById("run-container").remove();
    document.getElementById("glcanvas").removeAttribute("hidden");
    document.getElementById("glcanvas").focus();
    // impl_run();
}
run();