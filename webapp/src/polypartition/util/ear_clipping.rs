use crate::polypartition::PartitionVertex;

use super::{is_convex, is_inside, is_reflex, normalize, point_f64_approximately};

pub fn update_vertex_reflexity(v: usize, vertices: &mut Vec<PartitionVertex>) {
    let v1 = vertices[v].previous;
    let v3 = vertices[v].next;
    let v1_info = &vertices[v1].info;
    let v3_info = &vertices[v3].info;
    vertices[v].info.is_convex = !is_reflex(&v1_info.p, &vertices[v].info.p, &v3_info.p);
}

pub fn update_vertex(v: usize, vertices: &mut Vec<PartitionVertex>) {
    let v1 = vertices[v].previous;
    let v3 = vertices[v].next;
    let v1_info = &vertices[v1].get_info();
    let v3_info = &vertices[v3].get_info();
    let v_info = vertices[v].get_info();
    vertices[v].info.is_convex = is_convex(&v1_info.p, &v_info.p, &v3_info.p);

    let vec1 = normalize(&(v1_info.p - v_info.p));
    let vec3 = normalize(&(v3_info.p - v_info.p));
    //console_log_util(format!("{:?}\n{:?}", v1_info, v3_info));
    vertices[v].info.angle = vec1.x * vec3.x + vec1.y * vec3.y;

    if vertices[v].info.is_convex {
        vertices[v].info.is_ear = true;
        for vertex in vertices.iter() {
            let vertex_info = vertex.get_info();
            if point_f64_approximately(vertex_info.p, v_info.p) {
              continue;
            }
            if point_f64_approximately(vertex_info.p, v1_info.p) {
              continue;
            }
            if point_f64_approximately(vertex_info.p, v3_info.p) {
              continue;
            }
            if is_inside(&v1_info.p, &v_info.p, &v3_info.p, &vertex_info.p) {
              vertices[v].info.is_ear = false;
              break;
            }
        }
    } else {
        vertices[v].info.is_ear = false;
    }
}