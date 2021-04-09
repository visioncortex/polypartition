use visioncortex::{Color, ColorName, PointF64};
use wasm_bindgen::JsValue;

use crate::{canvas::Canvas, polypartition::PolygonProps};

pub struct DrawingUtil {
    canvas: Canvas,
}

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

    pub fn clear(&self) {
        self.canvas.clear();
    }

    pub fn draw_line(&self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let ctx = self.canvas.get_rendering_context_2d();
        ctx.set_stroke_style(JsValue::from_str(&Color::color(&ColorName::Black).to_color_string()).as_ref());
        ctx.begin_path();
        ctx.move_to(x1, y1);
        ctx.line_to(x2, y2);
        ctx.stroke();
    }

    pub fn draw_line_with_points(&self, p1: &PointF64, p2: &PointF64) {
        self.draw_line(p1.x, p1.y, p2.x, p2.y);
    }

    pub fn draw_polygon_with_props(&self, polygon_props: &PolygonProps) {
        let len = polygon_props.points.len();
        for curr in 0..len {
            let p1 = &polygon_props.points[curr];
            let p2 = &polygon_props.points[(curr+1) % len];
            self.draw_line_with_points(p1, p2);
        }
    }
}