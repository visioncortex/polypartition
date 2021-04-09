use std::rc::Rc;

use crate::polypartition::PartitionVertex;

use super::{is_convex, is_inside, is_reflex, normalize, point_f64_approximately};

pub fn update_vertex_reflexity(v: &mut PartitionVertex) {
    let v1 = Rc::clone(&v.previous);
    let v3 = Rc::clone(&v.next);
    v.is_convex = !is_reflex(&v1.p, &v.p, &v3.p);
}

pub fn update_vertex(v: &mut PartitionVertex, vertices: &[PartitionVertex]) {
    let v1 = Rc::clone(&v.previous);
    let v3 = Rc::clone(&v.next);
    v.is_convex = is_convex(&v1.p, &v.p, &v3.p);

    let vec1 = normalize(&(v1.p - v.p));
    let vec3 = normalize(&(v3.p - v.p));
    v.angle = vec1.x * vec3.x + vec1.y * vec3.y;

    if v.is_convex {
        v.is_ear = true;
        for vertex in vertices {
            if point_f64_approximately(vertex.p, v.p) {
              continue;
            }
            if point_f64_approximately(vertex.p, v1.p) {
              continue;
            }
            if point_f64_approximately(vertex.p, v3.p) {
              continue;
            }
            if is_inside(&v1.p, &v.p, &v3.p, &vertex.p) {
              v.is_ear = false;
              break;
            }
        }
    } else {
        v.is_ear = false;
    }
}