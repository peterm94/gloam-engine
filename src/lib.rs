#![feature(extern_types)]

use std::cell::RefCell;
use std::rc::Rc;
use macroquad::prelude::*;
use wasm_bindgen::prelude::*;

use crate::game::log;
use crate::scene::{GameObject, Scene, Transform};

mod game;
mod draw;
mod events;
mod scene;

static mut CURRENT_SCENE: Option<Scene> = None;

#[derive(Default)]
pub struct GameState {
    add_objects: Vec<(usize, GameObject, Rc<RefCell<Transform>>)>,
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
                let fps = 1000.0 / delta / 1000.0;
                if fps < 40.0 {
                    log(&format!("{}", 1000. / delta / 1000.));
                }
                scene.update(delta);
            }
        }
        next_frame().await
    }
}