use visioncortex::{Color, ColorName};
use wasm_bindgen::{JsValue, prelude::*};

use crate::canvas::Canvas;

#[wasm_bindgen]
pub struct DrawingUtil {
    canvas: Canvas,
}

#[wasm_bindgen]
impl DrawingUtil {
    pub fn from_canvas_id(canvas_id: &str) -> Self {
        if let Some(canvas) = Canvas::new_from_id(canvas_id) {
            Self {
                canvas
            }
        } else {
            panic!("Canvas id does not exist.")
        }
    }

    pub fn draw_line(&self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let ctx = self.canvas.get_rendering_context_2d();
        ctx.set_stroke_style(JsValue::from_str(&Color::color(&ColorName::Black).to_color_string()).as_ref());
        ctx.begin_path();
        ctx.move_to(x1, y1);
        ctx.line_to(x2, y2);
        ctx.stroke();
    }
}