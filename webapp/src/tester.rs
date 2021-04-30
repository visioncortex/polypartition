use visioncortex::PointF64;
use wasm_bindgen::prelude::*;

use crate::{draw::{DrawingUtil}, polypartition::{Polygon, PolygonInterface, remove_holes, triangulate_ec_vec, triangulate_mono_vec, triangulate_opt_vec}, util::console_log_util};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Tester {
    input_polygons: Vec<Polygon>,
    output_polygons: Option<Vec<Polygon>>,
}

#[wasm_bindgen]
impl Tester {
    pub fn from_input_text(text: &str) -> Self {
        let mut input_polygons = vec![];
        let lines: Vec<&str> = text.split('\n').collect();
        let mut i = 1;
        while i < lines.len()-1 {
            let num_vertices: usize = lines[i].parse().unwrap(); i += 1;
            let is_hole = lines[i] == "1"; i += 1;
            let mut points = vec![];
            let point_until = i+num_vertices;
            while i < point_until {
                let coords: Vec<f64> = lines[i].split(' ').map(|num_text| num_text.parse::<f64>().unwrap()).collect();
                points.push(PointF64::new(coords[0], coords[1]));
                i += 1;
            }
            let input_polygon = Polygon::from_points_and_is_hole(points, is_hole);
            if !input_polygon.is_valid() {
                panic!("One of the input polygons is invalid!");
            }
            input_polygons.push(
                input_polygon
            );
        }

        Self {
            input_polygons,
            output_polygons: None,
        }
    }

    pub fn print(&self, in_or_out: &str) {
        let empty = vec![];
        console_log_util(&format!("{:?}:\n {:?}",
        in_or_out,
        if in_or_out == "in" {
            &self.input_polygons
        } else if let Some(polys) = &self.output_polygons {
            polys
        } else {
            &empty
        }));
    }

    pub fn dump_polygons(&self, in_or_out: &str, decimal: bool) -> String {
        let polygons =
            if in_or_out == "in" {
                &self.input_polygons
            } else if let Some(output) = &self.output_polygons {
                output
            } else {
                panic!("No output!");
            };
        let mut dump = vec![polygons.len().to_string()];
        for polygon in polygons.iter() {
            dump.push(polygon.props().dump(decimal));
        }

        dump.join("\n")
    }

    pub fn draw_polygons(&self, canvas_id: &str, in_or_out: &str) {
        let polygons =
            if in_or_out == "in" {
                &self.input_polygons
            } else if let Some(output) = &self.output_polygons {
                output
            } else {
                panic!("No output!");
            };
        let drawing_util = DrawingUtil::from_canvas_id(canvas_id);
        drawing_util.clear();
        for polygon in polygons.iter() {
            drawing_util.draw_polygon_with_props(polygon.props());
        }
    }

    pub fn test_remove_holes(&mut self) -> Result<(), JsValue> {
        self.output_polygons = Some(remove_holes(&self.input_polygons)?);
        Ok(())
    }
    
    pub fn test_ear_clipping(&mut self) -> Result<(), JsValue> {
        let polygons_removed_holes = remove_holes(&self.input_polygons)?;
        self.output_polygons = Some(triangulate_ec_vec(polygons_removed_holes)?);
        Ok(())
    }

    pub fn test_optimal_dp(&mut self) -> Result<(), JsValue> {
        self.output_polygons = Some(triangulate_opt_vec(self.input_polygons.clone())?);
        Ok(())
    }

    pub fn test_monotone(&mut self) -> Result<(), JsValue> {
        self.output_polygons = Some(triangulate_mono_vec(self.input_polygons.clone())?);
        Ok(())
    }
}