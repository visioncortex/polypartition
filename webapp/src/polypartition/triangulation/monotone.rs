use intrusive_collections::Bound;

use crate::polypartition::{Polygon, PolygonInterface};
use crate::polypartition::util::{add_diagonal, EdgeTree, EdgeTreePtr, f64_approximately, is_below, is_convex, MonotoneVertex, ScanLineEdge, ScanLineEdgeAdaptor};
use crate::polypartition::enums::VertexType;

use visioncortex::PointF64;

pub fn monotone_partition(inpolys: Vec<Polygon>) -> Result<Vec<Polygon>, &'static str> {
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
    let mut vertex_types = vec![VertexType::Regular; max_num_vertices];
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

    let mut helpers = vec![0; max_num_vertices];

    let mut edge_tree = EdgeTree::new(ScanLineEdgeAdaptor::new());
    let mut edge_tree_pointers = vec![EdgeTreePtr::Null; max_num_vertices];

    for &v_index in priority.iter() {
        let v = &vertices[v_index].clone();
        let mut v_index2 = v_index;
        let mut v2 = v;

        // Depending on the vertex type, do the appropriate action
        match vertex_types[v_index] {
            VertexType::Start => {
                let mut new_edge = ScanLineEdge::default();
                new_edge.p1 = v.p;
                new_edge.p2 = vertices[v.next].p;
                new_edge.index = v_index;
                let cursor = edge_tree.insert(Box::new(new_edge));
                let ptr: *mut ScanLineEdge = cursor.as_cursor().clone_pointer().as_deref_mut().unwrap();
                edge_tree_pointers[v_index] = EdgeTreePtr::Node(ptr);
                helpers[v_index] = v_index;
            },
            VertexType::End => {
                if let EdgeTreePtr::Null = edge_tree_pointers[v.previous] {
                    return Err("Pointer is NULL.");
                }
                
                if let VertexType::Merge = vertex_types[helpers[v.previous]] {
                    add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[v.previous],
                        &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                }

                if let EdgeTreePtr::Node(ptr) = edge_tree_pointers[v.previous] {
                    let mut cursor = unsafe { edge_tree.cursor_mut_from_ptr(ptr) };
                    cursor.remove();
                }
                edge_tree_pointers[v.previous] = EdgeTreePtr::Null;
            },
            VertexType::Split => {
                let mut new_edge = ScanLineEdge::default();
                new_edge.p1 = v.p;
                new_edge.p2 = v.p;
                let mut edge_iter = edge_tree.lower_bound(Bound::Included(&new_edge));
                edge_iter.move_prev();
                if edge_iter.is_null() {
                    return Err("edge_iter is the first in EdgeTree.");
                }

                add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[edge_iter.get().unwrap().index],
                    &mut vertex_types, &mut edge_tree_pointers, &mut helpers);

                v_index2 = new_num_vertices - 2;
                v2 = &vertices[v_index2];
                helpers[edge_iter.get().unwrap().index] = v_index;

                let mut new_edge = ScanLineEdge::default();
                new_edge.p1 = v2.p;
                new_edge.p2 = vertices[v2.next].p;
                new_edge.index = v_index2;
                let cursor = edge_tree.insert(Box::new(new_edge));
                let ptr: *mut ScanLineEdge = cursor.as_cursor().clone_pointer().as_deref_mut().unwrap();
                edge_tree_pointers[v_index2] = EdgeTreePtr::Node(ptr);
                helpers[v_index2] = v_index2;
            },
            VertexType::Merge => {
                if let EdgeTreePtr::Null = edge_tree_pointers[v.previous] {
                    return Err("Pointer is NULL.");
                }

                if let VertexType::Merge = vertex_types[helpers[v.previous]] {
                    add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[v.previous],
                        &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                    v_index2 = new_num_vertices - 2;
                    v2 = &vertices[v_index2];
                }

                if let EdgeTreePtr::Node(ptr) = edge_tree_pointers[v.previous] {
                    let mut cursor = unsafe { edge_tree.cursor_mut_from_ptr(ptr) };
                    cursor.remove();
                }
                edge_tree_pointers[v.previous] = EdgeTreePtr::Null;

                let mut new_edge = ScanLineEdge::default();
                new_edge.p1 = v.p;
                new_edge.p2 = v.p;
                let mut edge_iter = edge_tree.lower_bound(Bound::Included(&new_edge));
                edge_iter.move_prev();
                if edge_iter.is_null() {
                    return Err("edge_iter is the first in EdgeTree.");
                }

                if let VertexType::Merge = vertex_types[helpers[edge_iter.get().unwrap().index]] {
                    add_diagonal(&mut vertices, &mut new_num_vertices, v_index2, helpers[edge_iter.get().unwrap().index],
                        &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                }

                helpers[edge_iter.get().unwrap().index] = v_index2;
            },
            VertexType::Regular => {
                if is_below(&v.p, &vertices[v.previous].p) {
                    if let EdgeTreePtr::Null = edge_tree_pointers[v.previous] {
                        return Err("Pointer is NULL.");
                    }

                    if let VertexType::Merge = vertex_types[helpers[v.previous]] {
                        add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[v.previous],
                            &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                        v_index2 = new_num_vertices - 2;
                        v2 = &vertices[v_index2];
                    }

                    if let EdgeTreePtr::Node(ptr) = edge_tree_pointers[v.previous] {
                        let mut cursor = unsafe { edge_tree.cursor_mut_from_ptr(ptr) };
                        cursor.remove();
                    }
                    edge_tree_pointers[v.previous] = EdgeTreePtr::Null;

                    let mut new_edge = ScanLineEdge::default();
                    new_edge.p1 = v2.p;
                    new_edge.p2 = vertices[v2.next].p;
                    new_edge.index = v_index2;
                    let cursor = edge_tree.insert(Box::new(new_edge));
                    let ptr: *mut ScanLineEdge = cursor.as_cursor().clone_pointer().as_deref_mut().unwrap();
                    edge_tree_pointers[v_index2] = EdgeTreePtr::Node(ptr);
                    helpers[v_index2] = v_index;
                } else {
                    let mut new_edge = ScanLineEdge::default();
                    new_edge.p1 = v.p;
                    new_edge.p2 = v.p;
                    let mut edge_iter = edge_tree.lower_bound(Bound::Included(&new_edge));
                    edge_iter.move_prev();
                    if edge_iter.is_null() {
                        return Err("edge_iter is the first in EdgeTree.");
                    }
                    if let VertexType::Merge = vertex_types[helpers[edge_iter.get().unwrap().index]] {
                        add_diagonal(&mut vertices, &mut new_num_vertices, v_index, helpers[edge_iter.get().unwrap().index],
                            &mut vertex_types, &mut edge_tree_pointers, &mut helpers);
                    }
                    helpers[edge_iter.get().unwrap().index] = v_index;
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