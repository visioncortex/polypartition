use super::{Polygon, PolygonInterface, intersects, is_in_cone, normalize};

pub fn remove_holes(inpolys: &[Polygon]) -> Result<Vec<Polygon>, &str> {
    // Check for the trivial case of no holes
    if inpolys.iter().find(|polygon| polygon.is_hole()).is_none() {
        return Ok(inpolys.to_vec());
    }

    let mut polys = inpolys.to_vec();
    // Repeatedly merge hole polygon with a suitable non-hole polygon
    // until no hole polygons are present.
    // Each time look for the hole polygon with the largest x
    while let (Some(hole_polygon), holepoint_index, hole_polygon_index) = polys.iter().enumerate().fold(
        (None, 0, 0), // (Option<&Polygon>, holepoint_index, hole_polygon_index)
        |(acc_polygon, acc_holepoint_index, acc_hole_polygon_index), (polygon_index, polygon)| {
            if !polygon.is_hole() {
                return (acc_polygon, acc_holepoint_index, acc_hole_polygon_index);
            }
            // Find the point of largest x
            let holepoint_index = polygon.props().points.iter()
                .enumerate()
                .skip(1)
                .fold( // effectively fold_first
                    (0, polygon.get_point(0)),
                    |(p1_index, p1), (p2_index, p2)| {
                    if p2.x > p1.x { (p2_index, *p2) } else { (p1_index, p1) }
                }).0;
            // Update
            if let Some(acc_polygon) = acc_polygon {
                // Compare the x of current hole polygon with history
                let curr_x = polygon.get_point(holepoint_index).x;
                let acc_polygon: &Polygon = acc_polygon;
                let hist_x = acc_polygon.get_point(acc_holepoint_index).x;
                if curr_x > hist_x {
                    (Some(polygon), holepoint_index, polygon_index)
                } else {
                    (Some(acc_polygon), acc_holepoint_index, acc_hole_polygon_index)
                }
            } else { // No history
                (Some(polygon), holepoint_index, polygon_index)
            }
        }) {
        // At this point, hole_polygon stores the hole polygon we're looking at in this iteration
        // and holepoint_index stores the index of the holepoint with largest x (across all hole polygons)
        let holepoint = hole_polygon.get_point(holepoint_index);

        // Now find the suitable non-hole polygon and its "polypoint"
        let non_hole_polygons = polys.iter().filter(|poly| !poly.is_hole());
        let (best_polygon, polypoint_index, best_polygon_index) = non_hole_polygons.into_iter().enumerate()
            .fold(
                (None, 0, 0), // To simulate pointfound. (Option<&polygon>, polypoint_index, best_polygon_index)
                |(mut acc_polygon, mut acc_polypoint_index, mut acc_best_polygon_index), (polygon_index, polygon)| {
                    // Loop through all points to check if this polygon is suitable
                    let points = &polygon.props().points;
                    let num_points = points.len();
                    for i in 0..num_points {
                        if points[i].x <= holepoint.x {
                            continue;
                        }
                        // Basic check
                        let prev = points[(i + num_points - 1) % num_points];
                        let curr = points[i];
                        let next = points[(i + 1) % num_points];
                        if !is_in_cone(&prev, &curr, &next, &holepoint) {
                            continue;
                        }
                        let polypoint = points[i];
                        // Check optimality
                        if let Some(acc_polygon) = acc_polygon {
                            let acc_polygon: &Polygon = acc_polygon;
                            let best_polypoint = acc_polygon.get_point(acc_polypoint_index);
                            let v1 = normalize(&(polypoint - holepoint));
                            let v2 = normalize(&(best_polypoint - holepoint));
                            if v2.x > v1.x {
                                continue;
                            }
                        }
                        // Check visibility
                        let all_visible = polys.iter().all(|poly| {
                            if poly.is_hole() {
                                return true;
                            }
                            let points = &poly.props().points;
                            let num_points = points.len();
                            for i2 in 0..num_points {
                                let curr = points[i2];
                                let next = points[(i2 + 1) % num_points];
                                if intersects(&holepoint, &polypoint, &curr, &next) {
                                    return false;
                                }
                            }
                            true
                        });
                        if all_visible {
                            acc_polygon = Some(polygon);
                            acc_polypoint_index = i;
                            acc_best_polygon_index = polygon_index;
                        }
                    }

                    (acc_polygon, acc_polypoint_index, acc_best_polygon_index)
                }
            );
            
        let best_polygon = if let Some(polygon) = best_polygon {
            polygon
        } else {
            return Err("No visible polypoint found. Cannot merge hole polygon with a non-hole polygon.");
        };

        // Construct the new polygon (merging hole_polygon and best_polygon)
        let mut newpoly_points = Vec::with_capacity(
            hole_polygon.props().num_points() + best_polygon.props().num_points() + 2
        );
        // Insert the points in non-hole polygon up until the polypoint
        for i in 0..=polypoint_index {
            newpoly_points.push(best_polygon.get_point(i));
        }
        // Insert all points in hole polygon in a cyclic manner STARTING from holepoint
        let hole_num_points = hole_polygon.props().num_points();
        for i in 0..=hole_polygon.props().num_points() {
            newpoly_points.push(hole_polygon.get_point((i + holepoint_index) % hole_num_points));
        }
        // Insert the rest of the points in non-hole polygon
        for i in polypoint_index..best_polygon.props().num_points() {
            newpoly_points.push(best_polygon.get_point(i));
        }
        let newpoly = Polygon::from_points_and_is_hole(newpoly_points, false);

        polys.remove(hole_polygon_index);
        polys.remove(best_polygon_index);
        polys.push(newpoly);
    }

    Ok(polys)
}