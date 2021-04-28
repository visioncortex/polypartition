use crate::polypartition::{PartitionVertex, PartitionVertexInfo, Polygon, PolygonInterface, util::update_vertex};

pub fn triangulate_ec_vec(polys: Vec<Polygon>) -> Result<Vec<Polygon>, String> {
    let mut triangles = vec![];
    for poly in polys.iter() {
        triangles.extend(triangulate_ec(poly)?);
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
    let num_vertices = poly.num_points();

    // Just in case (should be guarded above)
    if num_vertices < 3 {
        return Err("Input polygon has less than 3 vertices");
    }
    // Trivial case
    if num_vertices == 3 {
        return Ok(vec![poly.clone()]);
    }

    let mut vertices = vec![PartitionVertex::default(); num_vertices];

    for (i, vertex) in vertices.iter_mut().enumerate() {
        let mut info = PartitionVertexInfo::default();
        // Fill in info
        info.is_active = true;
        info.p = poly.get_point(i);

        vertex.set_info(info);

        // Setting previous/next
        if i == (num_vertices - 1) { // Last
            vertex.next = 0; // Wrap around
        } else {
            vertex.next = i+1; // Just the next
        }
        if i == 0 { // First
            vertex.previous = num_vertices - 1; // Wrap around
        } else {
            vertex.previous= i-1; // Just the previous
        }
    }

    // Update the angles and is_ear the first time
    for i in 0..num_vertices {
        update_vertex(i, &mut vertices);
    }

    for i in 0..(num_vertices-3) {
        // Find optimal ear
        let ear = vertices.iter().enumerate().fold(
            None,
            |optimal_ear, (i, vertex)| {
                if !vertex.info.is_active {
                    return optimal_ear;
                }
                if !vertex.info.is_ear {
                    return optimal_ear;
                }
                if optimal_ear.is_none() {
                    return Some(i);
                }
                let optimal_ear_i = optimal_ear.unwrap();
                let optimal_ear_angle = vertices[optimal_ear_i].info.angle;
                if vertex.info.angle > optimal_ear_angle {
                    Some(i)
                } else {
                    Some(optimal_ear_i)
                }
            }
        );
        if ear.is_none() {
            return Err("No ear found!");
        }
        let ear = ear.unwrap();
        let prev = vertices[ear].previous;
        let next = vertices[ear].next;

        triangles.push(Polygon::triangle(
            vertices[prev].info.p,
            vertices[ear].info.p,
            vertices[next].info.p,
        ));

        vertices[ear].info.is_active = false;
        // Tighten the loose ends
        vertices[prev].next = next;
        vertices[next].previous = prev;

        if i == (num_vertices - 4) {
            break;
        }

        update_vertex(prev, &mut vertices);
        update_vertex(next, &mut vertices);
    }

    for vertex in vertices.iter() {
        if vertex.info.is_active {
            triangles.push(Polygon::triangle(
            vertices[vertex.previous].info.p,
            vertex.info.p,
            vertices[vertex.next].info.p,
            ));
            break;
        }
    }

    Ok(triangles)
}