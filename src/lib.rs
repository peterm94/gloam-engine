#![feature(extern_types)]

use macroquad::prelude::*;
use wasm_bindgen::prelude::*;

mod game;

#[wasm_bindgen]
pub fn main2() {
    main();
}

#[macroquad::main("Gloam")]
async fn main() {
    loop {
        let delta = get_frame_time();
        game::Gloam::update(delta as f64);
        // draw_circle(200.0, 200.0, 1.0, RED);
        // draw_poly(300.0, 300.0, 100, 10.0, 0., RED);
        next_frame().await
    }
}
