use crate::polypartition::PartitionVertex;

use super::{is_convex, is_inside, is_reflex, normalize, point_f64_approximately};

pub fn update_vertex_reflexity(v: &mut PartitionVertex) {
    let v1_info = v.get_previous_info().unwrap();
    let v3_info = v.get_next_info().unwrap();
    v.info.is_convex = !is_reflex(&v1_info.p, &v.info.p, &v3_info.p);
}

pub fn update_vertex(v: &mut PartitionVertex, vertices: &[PartitionVertex]) {
    let v1_info = v.get_previous_info().unwrap();
    let v3_info = v.get_next_info().unwrap();
    v.info.is_convex = is_convex(&v1_info.p, &v.info.p, &v3_info.p);

    let vec1 = normalize(&(v1_info.p - v.info.p));
    let vec3 = normalize(&(v3_info.p - v.info.p));
    v.info.angle = vec1.x * vec3.x + vec1.y * vec3.y;

    if v.info.is_convex {
        v.info.is_ear = true;
        for vertex in vertices {
            if point_f64_approximately(vertex.info.p, v.info.p) {
              continue;
            }
            if point_f64_approximately(vertex.info.p, v1_info.p) {
              continue;
            }
            if point_f64_approximately(vertex.info.p, v3_info.p) {
              continue;
            }
            if is_inside(&v1_info.p, &v.info.p, &v3_info.p, &vertex.info.p) {
              v.info.is_ear = false;
              break;
            }
        }
    } else {
        v.info.is_ear = false;
    }
}