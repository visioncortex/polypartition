use crate::{polypartition::{PartitionVertex, PartitionVertexPtr}, util::console_log_util};

use super::{is_convex, is_inside, is_reflex, normalize, point_f64_approximately};

pub fn update_vertex_reflexity(v: &mut PartitionVertex) {
    let v1_info = v.get_previous_info().unwrap();
    let v3_info = v.get_next_info().unwrap();
    v.info.is_convex = !is_reflex(&v1_info.p, &v.info.p, &v3_info.p);
}

pub fn update_vertex(v: &PartitionVertexPtr, vertices: &[PartitionVertexPtr]) {
    let v1_info = v.borrow().get_previous_info().unwrap();
    let v3_info = v.borrow().get_next_info().unwrap();
    let p = v.borrow().info.p;
    v.borrow_mut().info.is_convex = is_convex(&v1_info.p, &p, &v3_info.p);

    let vec1 = normalize(&(v1_info.p - p));
    let vec3 = normalize(&(v3_info.p - p));
    //console_log_util(format!("{:?}\n{:?}", v1_info, v3_info));
    v.borrow_mut().info.angle = vec1.x * vec3.x + vec1.y * vec3.y;

    if v.borrow().info.is_convex {
        v.borrow_mut().info.is_ear = true;
        for vertex in vertices {
            if point_f64_approximately(vertex.borrow().info.p, p) {
              continue;
            }
            if point_f64_approximately(vertex.borrow().info.p, v1_info.p) {
              continue;
            }
            if point_f64_approximately(vertex.borrow().info.p, v3_info.p) {
              continue;
            }
            if is_inside(&v1_info.p, &p, &v3_info.p, &vertex.borrow().info.p) {
              v.borrow_mut().info.is_ear = false;
              break;
            }
        }
    } else {
        v.borrow_mut().info.is_ear = false;
    }
}