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
        let non_hole_polygons: Vec<Polygon> = self.input_polygons.iter()
            .filter_map(|polygon|
                if polygon.is_hole() {
                    None
                } else {
                    Some(polygon.clone())
                }
            ).collect();
        self.output_polygons = Some(triangulate_opt_vec(non_hole_polygons)?);
        Ok(())
    }

    pub fn test_monotone(&mut self) -> Result<(), JsValue> {
        self.output_polygons = Some(triangulate_mono_vec(self.input_polygons.clone())?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polypartition_lightning_person() {
        let input_text = "2\n44\n0\n170 75\n179 87\n178 108\n163 125\n163 138\n212 144\n230 99\n230 80\n254 79\n254 98\n235 163\n212 173\n189 172\n189 242\n219 301\n228 358\n254 361\n253 377\n208 377\n208 355\n196 310\n150 266\n108 310\n96 355\n96 377\n51 377\n50 361\n76 358\n85 301\n115 242\n115 172\n92 173\n69 163\n50 98\n50 79\n74 80\n74 99\n92 144\n141 138\n141 125\n126 108\n125 87\n134 75\n152 71\n6\n1\n159 161\n125 191\n153 197\n132 221\n179 196\n150 183";
        let mut tester = Tester::from_input_text(input_text);
        match tester.test_remove_holes() {
            Ok(_) => {
                let correct_dump = "1\n52\n0\n170 75\n179 87\n178 108\n163 125\n163 138\n212 144\n230 99\n230 80\n254 79\n254 98\n235 163\n212 173\n189 172\n179 196\n150 183\n159 161\n125 191\n153 197\n132 221\n179 196\n189 172\n189 242\n219 301\n228 358\n254 361\n253 377\n208 377\n208 355\n196 310\n150 266\n108 310\n96 355\n96 377\n51 377\n50 361\n76 358\n85 301\n115 242\n115 172\n92 173\n69 163\n50 98\n50 79\n74 80\n74 99\n92 144\n141 138\n141 125\n126 108\n125 87\n134 75\n152 71";
                assert!(tester.dump_polygons("out", false) == correct_dump);
            },
            Err(e) => {panic!(e.as_string().unwrap());}
        }
        match tester.test_ear_clipping() {
            Ok(_) => {
                let correct_dump = "50\n3\n0\n179 196\n189 172\n189 242\n3\n0\n125 191\n153 197\n132 221\n3\n0\n132 221\n179 196\n189 242\n3\n0\n230 80\n254 79\n254 98\n3\n0\n230 99\n230 80\n254 98\n3\n0\n230 99\n254 98\n235 163\n3\n0\n212 144\n230 99\n235 163\n3\n0\n212 144\n235 163\n212 173\n3\n0\n212 144\n212 173\n189 172\n3\n0\n163 138\n212 144\n189 172\n3\n0\n50 98\n50 79\n74 80\n3\n0\n50 98\n74 80\n74 99\n3\n0\n69 163\n50 98\n74 99\n3\n0\n69 163\n74 99\n92 144\n3\n0\n92 173\n69 163\n92 144\n3\n0\n115 172\n92 173\n92 144\n3\n0\n115 172\n92 144\n141 138\n3\n0\n189 172\n179 196\n150 183\n3\n0\n189 172\n150 183\n159 161\n3\n0\n163 138\n189 172\n159 161\n3\n0\n253 377\n208 377\n208 355\n3\n0\n96 355\n96 377\n51 377\n3\n0\n228 358\n254 361\n253 377\n3\n0\n228 358\n253 377\n208 355\n3\n0\n219 301\n228 358\n208 355\n3\n0\n219 301\n208 355\n196 310\n3\n0\n189 242\n219 301\n196 310\n3\n0\n189 242\n196 310\n150 266\n3\n0\n132 221\n189 242\n150 266\n3\n0\n51 377\n50 361\n76 358\n3\n0\n96 355\n51 377\n76 358\n3\n0\n96 355\n76 358\n85 301\n3\n0\n108 310\n96 355\n85 301\n3\n0\n108 310\n85 301\n115 242\n3\n0\n150 266\n108 310\n115 242\n3\n0\n132 221\n150 266\n115 242\n3\n0\n132 221\n115 242\n115 172\n3\n0\n125 191\n132 221\n115 172\n3\n0\n159 161\n125 191\n115 172\n3\n0\n159 161\n115 172\n141 138\n3\n0\n163 138\n159 161\n141 138\n3\n0\n163 125\n163 138\n141 138\n3\n0\n163 125\n141 138\n141 125\n3\n0\n178 108\n163 125\n141 125\n3\n0\n178 108\n141 125\n126 108\n3\n0\n179 87\n178 108\n126 108\n3\n0\n179 87\n126 108\n125 87\n3\n0\n170 75\n179 87\n125 87\n3\n0\n152 71\n170 75\n125 87\n3\n0\n152 71\n125 87\n134 75";
                assert!(tester.dump_polygons("out", false) == correct_dump);
            },
            Err(e) => {panic!(e.as_string().unwrap());}
        }
        match tester.test_optimal_dp() {
            Ok(_) => {
                let correct_dump = "42\n3\n0\n170 75\n179 87\n152 71\n3\n0\n179 87\n178 108\n152 71\n3\n0\n178 108\n141 125\n152 71\n3\n0\n178 108\n163 125\n141 125\n3\n0\n141 125\n125 87\n152 71\n3\n0\n163 125\n163 138\n141 125\n3\n0\n141 125\n126 108\n125 87\n3\n0\n125 87\n134 75\n152 71\n3\n0\n163 138\n141 138\n141 125\n3\n0\n163 138\n189 172\n141 138\n3\n0\n163 138\n212 144\n189 172\n3\n0\n189 172\n115 172\n141 138\n3\n0\n212 144\n212 173\n189 172\n3\n0\n189 172\n189 242\n115 172\n3\n0\n115 172\n92 144\n141 138\n3\n0\n212 144\n235 163\n212 173\n3\n0\n189 242\n115 242\n115 172\n3\n0\n115 172\n92 173\n92 144\n3\n0\n212 144\n254 98\n235 163\n3\n0\n189 242\n150 266\n115 242\n3\n0\n92 173\n69 163\n92 144\n3\n0\n212 144\n230 99\n254 98\n3\n0\n189 242\n196 310\n150 266\n3\n0\n150 266\n108 310\n115 242\n3\n0\n69 163\n50 98\n92 144\n3\n0\n230 99\n230 80\n254 98\n3\n0\n189 242\n219 301\n196 310\n3\n0\n108 310\n85 301\n115 242\n3\n0\n50 98\n74 99\n92 144\n3\n0\n230 80\n254 79\n254 98\n3\n0\n219 301\n208 355\n196 310\n3\n0\n108 310\n96 355\n85 301\n3\n0\n50 98\n74 80\n74 99\n3\n0\n219 301\n228 358\n208 355\n3\n0\n96 355\n76 358\n85 301\n3\n0\n50 98\n50 79\n74 80\n3\n0\n228 358\n208 377\n208 355\n3\n0\n96 355\n96 377\n76 358\n3\n0\n228 358\n253 377\n208 377\n3\n0\n96 377\n51 377\n76 358\n3\n0\n228 358\n254 361\n253 377\n3\n0\n51 377\n50 361\n76 358";
                assert!(tester.dump_polygons("out", false) == correct_dump);
            },
            Err(e) => {panic!(e.as_string().unwrap());}
        }
        match tester.test_monotone() {
            Ok(_) => {
                let correct_dump = "50\n3\n0\n134 75\n152 71\n170 75\n3\n0\n134 75\n170 75\n125 87\n3\n0\n125 87\n170 75\n179 87\n3\n0\n125 87\n179 87\n126 108\n3\n0\n126 108\n179 87\n178 108\n3\n0\n126 108\n178 108\n141 125\n3\n0\n141 125\n178 108\n163 125\n3\n0\n141 125\n163 125\n141 138\n3\n0\n141 138\n163 125\n163 138\n3\n0\n141 138\n163 138\n92 144\n3\n0\n92 144\n163 138\n212 144\n3\n0\n230 80\n254 79\n254 98\n3\n0\n230 80\n254 98\n230 99\n3\n0\n212 144\n230 99\n254 98\n3\n0\n212 144\n254 98\n235 163\n3\n0\n159 161\n212 144\n235 163\n3\n0\n189 172\n159 161\n235 163\n3\n0\n159 161\n189 172\n150 183\n3\n0\n179 196\n150 183\n189 172\n3\n0\n115 242\n132 221\n179 196\n3\n0\n179 196\n189 172\n189 242\n3\n0\n115 242\n179 196\n189 242\n3\n0\n150 266\n115 242\n189 242\n3\n0\n115 242\n150 266\n85 301\n3\n0\n85 301\n150 266\n108 310\n3\n0\n96 355\n85 301\n108 310\n3\n0\n85 301\n96 355\n76 358\n3\n0\n51 377\n50 361\n76 358\n3\n0\n51 377\n76 358\n96 355\n3\n0\n51 377\n96 355\n96 377\n3\n0\n212 173\n189 172\n235 163\n3\n0\n150 266\n189 242\n219 301\n3\n0\n150 266\n219 301\n196 310\n3\n0\n208 355\n196 310\n219 301\n3\n0\n208 355\n219 301\n228 358\n3\n0\n208 355\n228 358\n208 377\n3\n0\n228 358\n254 361\n208 377\n3\n0\n208 377\n254 361\n253 377\n3\n0\n50 79\n74 80\n50 98\n3\n0\n50 98\n74 80\n74 99\n3\n0\n92 144\n50 98\n74 99\n3\n0\n159 161\n92 144\n212 144\n3\n0\n50 98\n92 144\n69 163\n3\n0\n92 144\n159 161\n69 163\n3\n0\n115 172\n69 163\n159 161\n3\n0\n115 172\n159 161\n125 191\n3\n0\n132 221\n125 191\n153 197\n3\n0\n132 221\n115 172\n125 191\n3\n0\n115 172\n132 221\n115 242\n3\n0\n92 173\n69 163\n115 172";
                assert!(tester.dump_polygons("out", false) == correct_dump);
            },
            Err(e) => {panic!(e.as_string().unwrap());}
        }
    }

    #[test]
    fn polypartition_hexagon() {
        let input_text = "1\n6\n0\n60 40\n200 40\n220 110\n200 180\n60 180\n40 110";
        let mut tester = Tester::from_input_text(input_text);
        match tester.test_remove_holes() {
            Ok(_) => {
                let correct_dump = "1\n6\n0\n60 40\n200 40\n220 110\n200 180\n60 180\n40 110";
                assert!(tester.dump_polygons("out", false) == correct_dump);
            },
            Err(e) => {panic!(e.as_string().unwrap());}
        }
        match tester.test_ear_clipping() {
            Ok(_) => {
                let correct_dump = "4\n3\n0\n40 110\n60 40\n200 40\n3\n0\n40 110\n200 40\n220 110\n3\n0\n40 110\n220 110\n200 180\n3\n0\n40 110\n200 180\n60 180";
                assert!(tester.dump_polygons("out", false) == correct_dump);
            },
            Err(e) => {panic!(e.as_string().unwrap());}
        }
        match tester.test_optimal_dp() {
            Ok(_) => {
                let correct_dump = "4\n3\n0\n60 40\n60 180\n40 110\n3\n0\n60 40\n200 40\n60 180\n3\n0\n200 40\n200 180\n60 180\n3\n0\n200 40\n220 110\n200 180";
                assert!(tester.dump_polygons("out", false) == correct_dump);
            },
            Err(e) => {panic!(e.as_string().unwrap());}
        }
        match tester.test_monotone() {
            Ok(_) => {
                let correct_dump = "4\n3\n0\n60 40\n200 40\n40 110\n3\n0\n40 110\n200 40\n220 110\n3\n0\n40 110\n220 110\n60 180\n3\n0\n60 180\n220 110\n200 180";
                assert!(tester.dump_polygons("out", false) == correct_dump);
            },
            Err(e) => {panic!(e.as_string().unwrap());}
        }
    }
}