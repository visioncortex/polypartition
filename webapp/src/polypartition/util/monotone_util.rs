use std::{cell::RefCell, rc::Rc};

use visioncortex::PointF64;

use crate::polypartition::VertexType;

use super::{f64_approximately, is_convex, point_f64_approximately};

#[derive(Clone, Default, PartialEq)]
pub struct MonotoneVertex {
    pub p: PointF64,
    pub previous: usize,
    pub next: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ScanLineEdge {
    pub index: usize,
    pub p1: PointF64,
    pub p2: PointF64,
}

impl ScanLineEdge {
    pub fn is_left_of(&self, other: &ScanLineEdge) -> bool {
        if self.is_same_position_as(other) {
            return self.index < other.index;
        }

        if f64_approximately(other.p1.y, other.p2.y) {
            if f64_approximately(self.p1.y, self.p2.y) {
                return self.p1.y < other.p1.y;
            }
            return is_convex(&self.p1, &self.p2, &other.p1);
        }

        if f64_approximately(self.p1.y, self.p2.y) || self.p1.y < other.p1.y {
            return !is_convex(&other.p1, &other.p2, &self.p1);
        }

        is_convex(&self.p1, &self.p2, &other.p1)
    }

    pub fn is_same_position_as(&self, other: &ScanLineEdge) -> bool {
        point_f64_approximately(self.p1, other.p1) && point_f64_approximately(self.p2, other.p2)
    }
}

impl PartialEq for ScanLineEdge {
    fn eq(&self, other: &Self) -> bool {
        self.is_same_position_as(other)
    }
}

impl Eq for ScanLineEdge {}

impl PartialOrd for ScanLineEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScanLineEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_same_position_as(other) {
            std::cmp::Ordering::Equal
        } else if self.is_left_of(other) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

// Returns true iff p1 is considered to be above p2
pub fn is_above(p1: &PointF64, p2: &PointF64) -> bool {
    p1.y < p2.y || (f64_approximately(p1.y, p2.y) && p1.x < p2.x)
}

#[allow(clippy::too_many_arguments)]
pub fn add_diagonal(vertices: &mut Vec<MonotoneVertex>, num_vertices: &mut usize,
    index1: usize, index2: usize, vertex_types: &mut Vec<VertexType>,
    edge_vec_pointers: &mut Vec<Option<EdgeVecPtr>>, helpers: &mut Vec<usize>) {

    let new_index1 = *num_vertices;
    *num_vertices += 1;
    let new_index2 = *num_vertices;
    *num_vertices += 1;

    vertices[new_index1].p = vertices[index1].p;
    vertices[new_index2].p = vertices[index2].p;

    vertices[new_index2].next = vertices[index2].next;
    vertices[new_index1].next = vertices[index1].next;

    let temp_index = vertices[index2].next;
    vertices[temp_index].previous = new_index2;
    let temp_index = vertices[index1].next;
    vertices[temp_index].previous = new_index1;

    vertices[index1].next = new_index2;
    vertices[new_index2].previous = index1;

    vertices[index2].next = new_index1;
    vertices[new_index1].previous = index2;

    // Update all relevant structures
    vertex_types[new_index1] = vertex_types[index1];
    edge_vec_pointers[new_index1] = edge_vec_pointers[index1].clone();
    helpers[new_index1] = helpers[index1];
    if let Some(edge) = &edge_vec_pointers[new_index1] {
        edge.borrow_mut().index = new_index1;
    }

    vertex_types[new_index2] = vertex_types[index2];
    edge_vec_pointers[new_index2] = edge_vec_pointers[index2].clone();
    helpers[new_index2] = helpers[index2];
    if let Some(edge) = &edge_vec_pointers[new_index2] {
        edge.borrow_mut().index = new_index2;
    }
}

/// A data structure to store ScanLineEdge's
/// Requirement:
/// 1) Change any element given Rc in constant time (supported by Rc<RefCell<...>> in nature)
/// 2) Insert
/// 3) Remove given Rc 
/// 4) Lowerbound search
#[derive(Default)]
pub struct EdgeVec {
    pub vec: Vec<EdgeVecPtr>,
}

pub type EdgeVecPtr = Rc<RefCell<ScanLineEdge>>;

impl EdgeVec {
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_edge_copy(&self, index: usize) -> Option<ScanLineEdge> {
        if index < self.vec.len() {
            Some(self.vec[index].borrow().clone())
        } else {
            None
        }
    }

    pub fn insert(&mut self, edge: ScanLineEdge) -> Option<EdgeVecPtr> {
        if let Err(target_index) = self.find(&edge) {
            let rc = Rc::new(RefCell::new(edge));
            self.vec.insert(target_index, rc);
            Some(self.vec[target_index].clone())
        } else {
            None
        }

    }

    /// Remove the given edge from the vec if it exists
    pub fn remove(&mut self, edge: &ScanLineEdge) {
        let result = self.find(&edge);
        if let Ok(index) = result {
            self.vec.remove(index);
        }
    }

    /// Returns the index of the first edge that is not less than the input edge
    pub fn lower_bound(&self, edge: &ScanLineEdge) -> usize {
        match self.find(edge) {
            Ok(index) => index,
            Err(index) => index,
        }
    }

    /// Binary search for the edge
    pub fn find(&self, edge: &ScanLineEdge) -> Result<usize, usize> {
        self.vec.binary_search_by(|ptr| ptr.borrow().cmp(edge))
    }
}