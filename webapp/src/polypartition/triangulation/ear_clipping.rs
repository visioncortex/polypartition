use std::{cell::RefCell, rc::Rc};
use crate::polypartition::{PartitionVertex, PartitionVertexInfo, PartitionVertexPtr, Polygon, PolygonInterface, util::update_vertex};

pub fn triangulate_ec_vec(polys: Vec<Polygon>) -> Result<Vec<Polygon>, String> {
    let mut triangles = vec![];
    for poly in polys.iter() {
        let result = triangulate_ec(poly);
        match result {
            Ok(pieces) => triangles.extend(pieces),
            Err(e) => return Err(e.into())
        }
    }
    Ok(triangles)
}

/// Takes an arbitrary polygon.
///
/// Returns a vec of triangles.
pub fn triangulate_ec(poly: &Polygon) -> Result<Vec<Polygon>, &str> {
    if !poly.is_valid() {
        return Err("Input polygon is invalid.");
    }

    let mut triangles = vec![];
    let num_vertices = poly.get_num_points();

    // Just in case (should be guarded above)
    if num_vertices < 3 {
        return Err("Input polygon has less than 3 vertices");
    }
    // Trivial case
    if num_vertices == 3 {
        return Ok(vec![poly.clone()]);
    }

    let mut vertices = vec![];
    for _ in 0..num_vertices {
        vertices.push(Rc::new(RefCell::new(PartitionVertex::default())));
    }

    for i in 0..num_vertices {
        let mut info = PartitionVertexInfo::default();
        // Fill in info
        info.id = i;
        info.is_active = true;
        info.p = poly.get_point(i);

        vertices[i].borrow_mut().set_info(info);

        // Setting previous/next
        if i == (num_vertices - 1) { // Last
            vertices[i].borrow_mut().set_next(&vertices[0]); // Wrap around
        } else {
            vertices[i].borrow_mut().set_next(&vertices[i+1]); // Just the next
        }
        if i == 0 { // First
            vertices[i].borrow_mut().set_previous(&vertices[num_vertices - 1]); // Wrap around
        } else {
            vertices[i].borrow_mut().set_previous(&vertices[i-1]); // Just the previous
        }
    }

    // Update the angles and is_ear the first time
    for vertex in vertices.iter() {
        update_vertex(vertex, &vertices);
    }

    for i in 0..(num_vertices-3) {
        // Find optimal ear
        let mut debug_string = vec![];
        let ear = vertices.iter().fold(
            None,
            |ear, vertex| {
                debug_string.push(format!("{}\n{:?}",
                    ear.is_some(),
                    vertex.borrow().info,
                ));
                if !vertex.borrow().info.is_active {
                    return ear;
                }
                if !vertex.borrow().info.is_ear {
                    return ear;
                }
                if let Some(optimal_ear) = ear {
                    let optimal_ear: &PartitionVertexPtr = optimal_ear;
                    if vertex.borrow().info.angle > optimal_ear.borrow().info.angle {
                        Some(vertex)
                    } else {
                        ear
                    }
                } else {
                    Some(vertex)
                }
            }
        );
        if ear.is_none() {
            return Err("No ear found!");
        }
        let ear = ear.unwrap();

        triangles.push(Polygon::triangle(
            ear.borrow().get_previous_info().unwrap().p,
            ear.borrow().info.p,
            ear.borrow().get_next_info().unwrap().p
        ));

        ear.borrow_mut().info.is_active = false;
        // Tighten the loose ends
        let prev = Rc::clone(ear.borrow().previous.as_ref().unwrap());
        let next = Rc::clone(ear.borrow().next.as_ref().unwrap());
        prev.borrow_mut().set_next(&next);
        next.borrow_mut().set_previous(&prev);

        if i == (num_vertices - 4) {
            break;
        }

        update_vertex(&prev, &vertices);
        update_vertex(&next, &vertices);
    }

    for vertex in vertices.iter() {
        if vertex.borrow().info.is_active {
            triangles.push(Polygon::triangle(
                vertex.borrow().get_previous_info().unwrap().p,
                vertex.borrow().info.p,
                vertex.borrow().get_next_info().unwrap().p
            ));
            break;
        }
    }

    Ok(triangles)
}