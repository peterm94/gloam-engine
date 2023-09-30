#![feature(extern_types)]

use std::cell::RefCell;
use std::rc::Rc;
use collision::dbvt::DynamicBoundingVolumeTree;

use macroquad::prelude::*;
use wasm_bindgen::prelude::*;
use crate::collisions::Shape;
use crate::draw::to_color;

use crate::game::{GameOptions, log};
use crate::scene::{GameObject, Scene, Transform};

mod game;
mod draw;
mod events;
mod scene;
mod collisions;

static mut CURRENT_SCENE: Option<Scene> = None;

static mut GAME_OPTIONS: GameOptions = GameOptions { width: 512, height: 512, scale: 1, background_colour: 0 };
static mut STARTED: bool = false;

#[derive(Default)]
pub struct GameState {
    add_objects: Vec<(usize, GameObject, Rc<RefCell<Transform>>)>,
    add_colliders: Vec<(Shape, Rc<RefCell<usize>>)>,
    del_objects: Vec<usize>,

}

#[wasm_bindgen]
pub fn main2() {
    main();
}

#[macroquad::main("Gloam")]
async fn main() {
    // Wait until the game is started.
    unsafe {
        while !STARTED {
            next_frame().await
        }
    }

    unsafe {
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., GAME_OPTIONS.width as f32, GAME_OPTIONS.height as f32));
        // There is a macroquad bug that makes the camera render upside down.
        // https://github.com/not-fl3/macroquad/issues/171
        log(&format!("{} {}", camera.zoom.x, camera.zoom.y));
        camera.zoom.x = 1.0 / (GAME_OPTIONS.width as f32 / 2.0);
        camera.zoom.y = 1.0 / (GAME_OPTIONS.height as f32 / 2.0);
        // camera.zoom.y *= -1.0;
        set_camera(&camera);
    }

    let bg_colour = to_color(unsafe { GAME_OPTIONS.background_colour });

    loop {
        unsafe {
            if let Some(scene) = &mut CURRENT_SCENE {
                clear_background(bg_colour);
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