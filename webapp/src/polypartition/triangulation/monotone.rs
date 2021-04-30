use crate::polypartition::{EdgeVec, Polygon, PolygonInterface};
use crate::polypartition::util::{add_diagonal, f64_approximately, is_below, is_convex, MonotoneVertex, ScanLineEdge};
use crate::polypartition::enums::VertexType;

use visioncortex::PointF64;

#[allow(clippy::clippy::missing_safety_doc)]
pub unsafe fn triangulate_mono_vec(polys: Vec<Polygon>) -> Result<Vec<Polygon>, String> {
    let polys = monotone_partition(polys)?;
    let mut triangles = vec![];
    for poly in polys.iter() {
        triangles.extend(triangulate_mono(poly)?);
    }
    Ok(triangles)
}

#[allow(clippy::clippy::missing_safety_doc)]
pub unsafe fn triangulate_mono(poly: &Polygon) -> Result<Vec<Polygon>, &str> {
    if !poly.is_valid() {
        return Err("Input polygon is invalid.");
    }

    let num_points = poly.num_points();

    // Trivial case
    if num_points == 3 {
        return Ok(vec![poly.clone()]);
    }
    
    let points = &poly.props().points;

    let mut top_index = 0;
    let mut bottom_index = 0;
    // Find the top-most and bottom-most points
    for i in 1..num_points {
        if is_below(&points[i], &points[bottom_index]) {
            bottom_index = i;
        }
        if is_below(&points[top_index], &points[i]) {
            top_index = i;
        }
    }

    crate::util::console_log_util(format!("Top: {:?} Bottom: {:?}", points[top_index], points[bottom_index]));

    // Check if the polygon is really monotone
    {
        let mut i;

        i = top_index;
        while i != bottom_index {
            let i2 = (i+1) % num_points;
            if !is_below(&points[i2], &points[i]) {
                return Err("Input polygon is not monotone.");
            }
            i = i2;
        }

        i = bottom_index;
        while i != top_index {
            let i2 = (i+1) % num_points;
            if !is_below(&points[i], &points[i2]) {
                return Err("Input polygon is not monotone.");
            }
            i = i2;
        }
    }

    let mut vertex_types = vec![0_i8; num_points];
    let mut priority = vec![0_usize; num_points];

    // Merge left and right vertex chains
    priority[0] = top_index;
    vertex_types[top_index] = 0;
    let mut left_index = (top_index+1) % num_points;
    let mut right_index = if top_index == 0 {num_points-1} else {top_index-1};
    for p in priority.iter_mut().skip(1).take(num_points - 2) {
        if left_index == bottom_index {
            *p = right_index;
            right_index = if right_index==0 {num_points-1} else {right_index-1};
            vertex_types[*p] = -1;
        } else if right_index == bottom_index {
            *p = left_index;
            left_index = (left_index+1) % num_points;
            vertex_types[*p] = 1;
        } else if is_below(&points[left_index], &points[right_index]) {
            *p = right_index;
            right_index = if right_index==0 {num_points-1} else {right_index-1};
            vertex_types[*p]  = -1;
        } else {
            *p = left_index;
            left_index = (left_index+1) % num_points;
            vertex_types[*p] = 1;
        }
    }
    priority[num_points-1] = bottom_index;
    vertex_types[bottom_index] = 0;

    let mut stack = vec![0_usize; num_points];
    let mut stack_ptr = 2;

    stack[0] = priority[0];
    stack[1] = priority[1];

    let mut triangles = vec![];

    // For each vertex from top to bottom trim as many triangles as possible
    for i in 2..(num_points-1) {
        let v_index = priority[i];
        if vertex_types[v_index] != vertex_types[stack[stack_ptr - 1]] {
            for j in 0..(stack_ptr-1) {
                if vertex_types[v_index] == 1 {
                    triangles.push(Polygon::triangle(
                        points[stack[j+1]], points[stack[j]], points[v_index]
                    ));
                } else {
                    triangles.push(Polygon::triangle(
                        points[stack[j]], points[stack[j+1]], points[v_index]
                    ));
                }
            }
            stack[0] = priority[i-1];
            stack[1] = priority[i];
            stack_ptr = 2;
        } else {
            stack_ptr -= 1;
            while stack_ptr > 0 {
                if vertex_types[v_index] == 1 {
                    if is_convex(&points[v_index], &points[stack[stack_ptr - 1]], &points[stack[stack_ptr]]) {
                        triangles.push(Polygon::triangle(
                            points[v_index], points[stack[stack_ptr-1]], points[stack[stack_ptr]]
                        ));
                        stack_ptr -= 1;
                    } else {
                        break;
                    }
                } else if is_convex(&points[v_index], &points[stack[stack_ptr]], &points[stack[stack_ptr - 1]]) {
                    triangles.push(Polygon::triangle(
                        points[v_index], points[stack[stack_ptr]], points[stack[stack_ptr-1]]
                    ));
                    stack_ptr -= 1;
                } else { 
                    break;
                }
            }
            stack[stack_ptr + 1] = v_index;
            stack_ptr += 2;
        }
    }
    let v_index = priority[num_points-1];
    for j in 0..(stack_ptr-1) {
        if vertex_types[stack[j+1]] == 1 {
            triangles.push(Polygon::triangle(
                points[stack[j]], points[stack[j+1]], points[v_index]
            ));
        } else {
            triangles.push(Polygon::triangle(
                points[stack[j+1]], points[stack[j]], points[v_index]
            ));
        }
    }

    Ok(triangles)
}

#[allow(clippy::clippy::missing_safety_doc)]
pub unsafe fn monotone_partition(inpolys: Vec<Polygon>) -> Result<Vec<Polygon>, &'static str> {
    if inpolys.iter().any(|poly| !poly.is_valid()) {
        return Err("Some input polygon is invalid!");
    }

    let num_vertices = inpolys.iter().fold(0, |acc, poly| acc + poly.num_points());

    let max_num_vertices = num_vertices * 3;
    let mut vertices = vec![MonotoneVertex::default(); max_num_vertices];
    let mut new_num_vertices = num_vertices;

    let mut poly_start_index = 0;
    for poly in inpolys.iter() {
        let num_points = poly.num_points();
        let poly_end_index = poly_start_index + num_points - 1;
        for i in 0..num_points {
            vertices[i + poly_start_index].p = poly.get_point(i);
            if i == 0 {
                vertices[i + poly_start_index].previous = poly_end_index;
            } else {
                vertices[i + poly_start_index].previous = i + poly_start_index - 1;
            }
            if i == num_points - 1 {
                vertices[i + poly_start_index].next = poly_start_index;
            } else {
                vertices[i + poly_start_index].next = i + poly_start_index + 1;
            }
        }
        poly_start_index = poly_end_index + 1;
    }

    let mut priority: Vec<usize> = (0..num_vertices).collect();
    priority.sort_by(|&index1, &index2| {
        let p1 = vertices[index1].p;
        let p2 = vertices[index2].p;
        let result;
        // Primary key is y, secondary key is x
        if !f64_approximately(p1.y, p2.y) {
            result = p1.y.partial_cmp(&p2.y).unwrap();
        } else {
            result = p1.x.partial_cmp(&p2.x).unwrap();
        }
        // Sort in falling order
        if let std::cmp::Ordering::Less = result {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    // Determine vertex types
    let mut vertex_types = vec![VertexType::Null; max_num_vertices];
    for i in 0..num_vertices {
        let v = &vertices[i];
        let v_prev = &vertices[v.previous];
        let v_next = &vertices[v.next];

        if is_below(&v_prev.p, &v.p) && is_below(&v_next.p, &v.p) {
            if is_convex(&v_next.p, &v_prev.p, &v.p) {
                vertex_types[i] = VertexType::Start;
            } else {
                vertex_types[i] = VertexType::Split;
            }
        } else if is_below(&v.p, &v_prev.p) && is_below(&v.p, &v_next.p) {
            if is_convex(&v_next.p, &v_prev.p, &v.p) {
                vertex_types[i] = VertexType::End;
            } else {
                vertex_types[i] = VertexType::Merge;
            }
        } else {
            vertex_types[i] = VertexType::Regular;
        }
    }

    // Correct up to this point

    let mut helpers = vec![0; max_num_vertices];

    let mut edge_tree = EdgeVec::default();
    let mut edge_tree_pointers = vec![None; max_num_vertices];

    for &v_index in priority.iter() {
        let v = &vertices[v_index].clone();
        let mut v_index2 = v_index;
        let mut v2 = v;

        // Depending on the vertex type, do the appropriate action
        match vertex_types[v_index] {
            VertexType::Null => {panic!()},
            VertexType::Start => {
                let mut new_edge = ScanLineEdge::default();
                new_edge.p1 = v.p;
                new_edge.p2 = vertices[v.next].p;
                new_edge.index = v_index;
                let ptr = edge_tree.insert(new_edge);
                edge_tree_pointers[v_index] = ptr;
                helpers[v_index] = v_index;
            },
            VertexType::End => {
                if edge_tree_pointers[v.previous].is_none() {
                    return Err("Pointer is NULL.");
                }
                
                if let VertexType::Merge = vertex_types[helpers[v.previous]] {
                    add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[v.previous],
                        &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                }

                if let Some(ptr) = &edge_tree_pointers[v.previous] {
                    edge_tree.remove(&ptr.borrow());
                }
                edge_tree_pointers[v.previous] = None;
            },
            VertexType::Split => {
                let mut new_edge = ScanLineEdge::default();
                new_edge.p1 = v.p;
                new_edge.p2 = v.p;
                let mut edge_pos_index = edge_tree.lower_bound(&new_edge);
                if edge_pos_index == 0 {
                    return Err("edge_iter is the first in EdgeTree.");
                }
                edge_pos_index -= 1;

                let index = edge_tree.get_edge_copy(edge_pos_index).unwrap().index;

                add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[index],
                    &mut vertex_types, &mut edge_tree_pointers, &mut helpers);

                v_index2 = new_num_vertices - 2;
                v2 = &vertices[v_index2];
                helpers[index] = v_index;

                let mut new_edge = ScanLineEdge::default();
                new_edge.p1 = v2.p;
                new_edge.p2 = vertices[v2.next].p;
                new_edge.index = v_index2;
                let ptr = edge_tree.insert(new_edge);
                edge_tree_pointers[v_index2] = ptr;
                helpers[v_index2] = v_index2;
            },
            VertexType::Merge => {
                if edge_tree_pointers[v.previous].is_none() {
                    return Err("Pointer is NULL.");
                }

                #[allow(unused_assignments)]
                if let VertexType::Merge = vertex_types[helpers[v.previous]] {
                    add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[v.previous],
                        &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                    v_index2 = new_num_vertices - 2;
                    v2 = &vertices[v_index2]; // False alarm here?
                }

                if let Some(ptr) = &edge_tree_pointers[v.previous] {
                    edge_tree.remove(&ptr.borrow());
                }
                edge_tree_pointers[v.previous] = None;

                let mut new_edge = ScanLineEdge::default();
                new_edge.p1 = v.p;
                new_edge.p2 = v.p;
                let mut edge_pos_index = edge_tree.lower_bound(&new_edge);
                if edge_pos_index == 0 {
                    return Err("edge_iter is the first in EdgeTree.");
                }
                edge_pos_index -= 1;

                let index = edge_tree.get_edge_copy(edge_pos_index).unwrap().index;

                if let VertexType::Merge = vertex_types[helpers[index]] {
                    add_diagonal(&mut vertices, &mut new_num_vertices, v_index2, helpers[index],
                        &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                }

                helpers[index] = v_index2;
            },
            VertexType::Regular => {
                if is_below(&v.p, &vertices[v.previous].p) {
                    if edge_tree_pointers[v.previous].is_none() {
                        return Err("Pointer is NULL.");
                    }

                    if let VertexType::Merge = vertex_types[helpers[v.previous]] {
                        add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[v.previous],
                            &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                        v_index2 = new_num_vertices - 2;
                        v2 = &vertices[v_index2];
                    }

                    if let Some(ptr) = &edge_tree_pointers[v.previous] {
                        edge_tree.remove(&ptr.borrow());
                    }

                    let mut new_edge = ScanLineEdge::default();
                    new_edge.p1 = v2.p;
                    new_edge.p2 = vertices[v2.next].p;
                    new_edge.index = v_index2;
                    let ptr = edge_tree.insert(new_edge);
                    edge_tree_pointers[v_index2] = ptr;
                    helpers[v_index2] = v_index;
                } else {
                    let mut new_edge = ScanLineEdge::default();
                    new_edge.p1 = v.p;
                    new_edge.p2 = v.p;
                    let mut edge_pos_index = edge_tree.lower_bound(&new_edge);
                    if edge_pos_index == 0 {
                        return Err("edge_iter is the first in EdgeTree.");
                    }
                    edge_pos_index -= 1;

                    let index = edge_tree.get_edge_copy(edge_pos_index).unwrap().index;

                    if let VertexType::Merge = vertex_types[helpers[index]] {
                        add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[index],
                            &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                    }
                    helpers[index] = v_index;
                }
            },
        }
    }

    let mut monotone_polys = vec![];

    let mut used = vec![false; new_num_vertices];
    let mut size;
    for i in 0..new_num_vertices {
        if used[i] {
            continue;
        }
        let v = &vertices[i];
        let mut v_next = &vertices[v.next];
        size = 1;
        while v_next != v {
            v_next = &vertices[v_next.next];
            size += 1;
        }

        let mut points = vec![PointF64::default(); size];
        let v = &vertices[i];
        points[0] = v.p;
        let mut v_next = &vertices[v.next];
        size = 1;
        used[i] = true;
        used[v.next] = true;
        while v_next != v {
            points[size] = v_next.p;
            used[v_next.next] = true;
            v_next = &vertices[v_next.next];
            size += 1;
        }

        monotone_polys.push(Polygon::from_points_and_is_hole(points, false));
    }
    Ok(monotone_polys)
}