use macroquad::prelude::*;
use wasm_bindgen::prelude::*;

use crate::game::{Gloam, TEXTURES};

#[wasm_bindgen]
impl Gloam {
    pub fn draw_texture(tex_id: usize, x: f32, y: f32) {
        let tex = unsafe { TEXTURES.get(tex_id) };
        if let Some(tex) = tex {
            draw_texture(tex, x, y, WHITE);
        }
    }

    pub fn draw_texture_part(tex_id: usize, x: f32, y: f32, rx: f32, ry: f32, rw: f32, rh: f32) {
        let tex = unsafe { TEXTURES.get(tex_id) };

        if let Some(tex) = tex {
            draw_texture_ex(tex, x, y, WHITE, DrawTextureParams {
                source: Some(Rect { x: rx, y: ry, w: rw, h: rh }), ..Default::default() });
        }
    }

    pub fn draw_circle_filled(x: f32, y: f32, r: f32, color: u32) {
        draw_circle(x, y, r, to_color(color));
    }

    pub fn draw_hexagon(x: f32, y: f32, size: f32, border: f32, vertical: bool, border_color: u32, fill_color: u32) {
        draw_hexagon(x, y, size, border, vertical, to_color(border_color), to_color(fill_color));
    }

    pub fn draw_circle(x: f32, y: f32, r: f32, thickness: f32, color: u32) {
        draw_circle_lines(x, y, r, thickness, to_color(color));
    }

    pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: u32) {
        draw_line(x1, y1, x2, y2, thickness, to_color(color));
    }

    pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, thickness: f32, color: u32) {
        draw_rectangle_lines(x, y, w, h, thickness, to_color(color));
    }

    pub fn draw_rectangle_filled(x: f32, y: f32, w: f32, h: f32, color: u32) {
        draw_rectangle(x, y, w, h, to_color(color));
    }

    pub fn draw_triangle_filled(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: u32) {
        draw_triangle(Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), to_color(color));
    }

    pub fn draw_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, thickness: f32, color: u32) {
        draw_triangle_lines(Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), thickness, to_color(color));
    }

    // TODO ttf font resource loading
    // TODO draw_text_ex
    pub fn draw_text(text: &str, x: f32, y: f32, size: f32, color: u32, multi_line: Option<bool>) {
        if let Some(false) = multi_line {
            draw_text(text, x, y, size, to_color(color));
            return;
        }
        let color = to_color(color);
        let line_height = measure_text("Ay", None, size as _, 1.).height;
        text.split('\n').enumerate().for_each(|(i, line)| {
            draw_text(line, x, y + (line_height * i as f32), size, color);
        })
    }

    pub fn measure_text(text: &str, size: u16, multi_line: Option<bool>) -> TextDims {
        let dims = measure_text(text, None, size, 1.0);
        if let Some(false) = multi_line {
            return dims.into();
        }
        let mut width = 0.0;
        let height = measure_text("Ay", None, size as _, 1.).height;
        let offset_y = dims.offset_y;

        // This is a pixel short without this, not sure how it scales to lots of lines.
        let mut lines = 0.1;
        text.split('\n').for_each(|line| {
            let line_dims = measure_text(line, None, size as _, 1.0);
            width = f32::max(width, line_dims.width);
            lines += 1.0;
        });

        TextDims { width, height: height * lines, offset_y }
    }
}

pub fn to_color(value: u32) -> Color {
    let red = (value >> 16) & 0xFF;
    let green = (value >> 8) & 0xFF;
    let blue = value & 0xFF;

    color_u8!(red, green, blue, 255)
}

#[wasm_bindgen]
pub struct TextDims {
    pub width: f32,
    pub height: f32,
    pub offset_y: f32,
}

impl Into<TextDims> for TextDimensions {
    fn into(self) -> TextDims {
        TextDims { width: self.width, height: self.height, offset_y: self.offset_y }
    }
}

#[cfg(test)]
mod tests {
    use macroquad::prelude::*;

    #[test]
    fn test_color_parse() {
        assert_eq!(to_color(0xFFFFFF), WHITE);
        assert_eq!(to_color(0xFF00FF), MAGENTA);
    }
}