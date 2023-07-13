use macroquad::prelude::*;
use wasm_bindgen::prelude::*;

use crate::game::{Gloam, TEXTURES};

#[wasm_bindgen]
impl Gloam {
    pub fn draw_texture(tex_id: usize, x: f32, y: f32) {
        let tex = unsafe { TEXTURES.get(tex_id) };
        if let Some(tex) = tex {
            draw_texture(*tex, x, y, WHITE);
        }
    }

    // TODO provide colours
    pub fn draw_circle_filled(x: f32, y: f32, r: f32, _color: u32) {
        draw_circle(x, y, r, Color::from_rgba(0, 0, 0, 255));
    }

    pub fn draw_hexagon(x: f32, y: f32, size: f32, border: f32, vertical: bool, _border_color: u32, _fill_color: u32) {
        draw_hexagon(x, y, size, border, vertical, BLACK, WHITE);
    }

    pub fn draw_circle(x: f32, y: f32, r: f32, thickness: f32, _color: u32) {
        draw_circle_lines(x, y, r, thickness, WHITE);
    }

    pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, _color: u32) {
        draw_line(x1, y1, x2, y2, thickness, WHITE);
    }

    pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, thickness: f32, _color: u32) {
        draw_rectangle_lines(x, y, w, h, thickness, WHITE);
    }

    pub fn draw_triangle_filled(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, _color: u32) {
        draw_triangle(Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), WHITE);
    }

    pub fn draw_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, thickness: f32, _color: u32) {
        draw_triangle_lines(Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), thickness, WHITE);
    }
}