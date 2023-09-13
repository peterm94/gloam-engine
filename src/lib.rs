#![feature(extern_types)]

use macroquad::prelude::*;
use wasm_bindgen::prelude::*;
use crate::scene2::{Graph, SCENE_GRAPH};

mod game;
mod draw;
mod events;
mod scene;
mod scene2;

#[wasm_bindgen]
pub fn main2() {
    main();
}

#[macroquad::main("Gloam")]
async fn main() {

    loop {
        let delta = get_frame_time();
        game::Gloam::update(delta as f64);
        next_frame().await
    }
}