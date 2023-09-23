#![feature(extern_types)]

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use macroquad::prelude::*;
use wasm_bindgen::prelude::*;

use crate::game::log;
use crate::scene::{GameObject, Scene};

mod game;
mod draw;
mod events;
mod scene;
static mut CURRENT_SCENE: Option<Scene> = None;

#[derive(Default)]
pub struct GameState {
    add_objects: Vec<(usize, GameObject)>,
    del_objects: Vec<usize>,

}

#[wasm_bindgen]
pub fn main2() {
    main();
}

#[macroquad::main("Gloam")]
async fn main() {
    loop {
        unsafe {
            if let Some(scene) = &mut CURRENT_SCENE {
                let delta = get_frame_time();
                log(&format!("{}", 1000. / delta / 1000.));
                scene.update(delta);
            }
        }
        next_frame().await
    }
}