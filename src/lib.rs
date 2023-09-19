#![feature(extern_types)]

use macroquad::prelude::*;
use wasm_bindgen::prelude::*;

use crate::scene::SceneWrapper;

mod game;
mod draw;
mod events;
mod scene;

static mut CURRENT_SCENE: Option<SceneWrapper> = None;

#[wasm_bindgen]
pub fn main2() {
    main();
}

#[macroquad::main("Gloam")]
async fn main() {
    loop {
        unsafe {
            if let Some(scene) = &CURRENT_SCENE {
                let delta = get_frame_time();
                scene.update(delta);
            }
        }
        next_frame().await
    }
}