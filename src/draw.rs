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

    // TODO implement sprite sheets and use this
    pub fn draw_texture_ex(tex_id: usize, x: f32, y: f32) {
        let tex = unsafe { TEXTURES.get(tex_id) };
        if let Some(tex) = tex {
            draw_texture_ex(*tex, x, y, WHITE, DrawTextureParams::default());
        }
    }

    pub fn draw_circle_filled(x: f32, y: f32, r: f32, color: u32) {
        draw_circle(x, y, r, Gloam::to_color(color));
    }

    pub fn draw_hexagon(x: f32, y: f32, size: f32, border: f32, vertical: bool, border_color: u32, fill_color: u32) {
        draw_hexagon(x, y, size, border, vertical, Gloam::to_color(border_color), Gloam::to_color(fill_color));
    }

    pub fn draw_circle(x: f32, y: f32, r: f32, thickness: f32, color: u32) {
        draw_circle_lines(x, y, r, thickness, Gloam::to_color(color));
    }

    pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: u32) {
        draw_line(x1, y1, x2, y2, thickness, Gloam::to_color(color));
    }

    pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, thickness: f32, color: u32) {
        draw_rectangle_lines(x, y, w, h, thickness, Gloam::to_color(color));
    }

    pub fn draw_triangle_filled(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: u32) {
        draw_triangle(Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), Gloam::to_color(color));
    }

    pub fn draw_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, thickness: f32, color: u32) {
        draw_triangle_lines(Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), thickness, Gloam::to_color(color));
    }

    // TODO ttf font resource loading
    // TODO draw_text_ex
    pub fn draw_text(text: &str, x: f32, y: f32, size: f32, color: u32) {
        draw_text(text, x, y, size, Gloam::to_color(color));
    }

    fn to_color(value: u32) -> Color {
        let red = (value >> 16) & 0xFF;
        let green = (value >> 8) & 0xFF;
        let blue = value & 0xFF;

        color_u8!(red, green, blue, 255)
    }
}

#[cfg(test)]
mod tests {
    use macroquad::prelude::*;

    use crate::game::Gloam;

    #[test]
    fn test_color_parse() {
        assert_eq!(Gloam::to_color(0xFFFFFF), WHITE);
        assert_eq!(Gloam::to_color(0xFF00FF), MAGENTA);
    }
}