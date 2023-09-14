#![feature(extern_types)]

use macroquad::prelude::*;
use wasm_bindgen::prelude::*;

use crate::scene::Scene;

mod game;
mod draw;
mod events;
mod scene;
#[wasm_bindgen]
pub fn main2() {
    main();
}

#[macroquad::main("Gloam")]
async fn main() {

    loop {
        let delta = get_frame_time();
        Scene::update(delta as f64);
        next_frame().await
    }
}