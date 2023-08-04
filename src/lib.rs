#![feature(extern_types)]

use macroquad::prelude::*;
use macroquad::telemetry::log_string;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

mod game;
mod draw;

#[wasm_bindgen]
pub fn main2() {
    main();
}

#[macroquad::main("Gloam")]
async fn main() {
    loop {
        game::Gloam::update(delta as f64);
        next_frame().await
    }
}