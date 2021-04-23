use intrusive_collections::{KeyAdapter, RBTree, RBTreeLink, intrusive_adapter};
use visioncortex::PointF64;

use crate::polypartition::VertexType;

use super::{f64_approximately, is_convex, point_f64_approximately};

#[derive(Clone, Default, PartialEq)]
pub struct MonotoneVertex {
    pub p: PointF64,
    pub previous: usize,
    pub next: usize,
}

#[derive(Clone, Default)]
pub struct ScanLineEdge {
    pub index: usize,
    pub p1: PointF64,
    pub p2: PointF64,
    pub link: RBTreeLink,
}

impl ScanLineEdge {
    pub fn is_left_of(&self, other: &ScanLineEdge) -> bool {
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

intrusive_adapter!(
    pub ScanLineEdgeAdaptor = Box<ScanLineEdge>: ScanLineEdge { link: RBTreeLink }
);

impl<'a> KeyAdapter<'a> for ScanLineEdgeAdaptor {
    type Key = ScanLineEdge;

    fn get_key(&self, value: &'a Self::Key) -> Self::Key {
        value.clone()
    }
}

pub type EdgeTree = RBTree<ScanLineEdgeAdaptor>;
#[derive(Clone, Copy)]
pub enum EdgeTreePtr {
    Node(*mut ScanLineEdge),
    Null
}

pub fn is_below(p1: &PointF64, p2: &PointF64) -> bool {
    p1.y < p2.y || (f64_approximately(p1.y, p2.y) && p1.x < p2.x)
}

#[allow(clippy::too_many_arguments)]
pub fn add_diagonal(vertices: &mut Vec<MonotoneVertex>, num_vertices: &mut usize,
    index1: usize, index2: usize, vertex_types: &mut Vec<VertexType>,
    edge_tree_pointers: &mut Vec<EdgeTreePtr>, helpers: &mut Vec<usize>) {
    
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
    edge_tree_pointers[new_index1] = edge_tree_pointers[index1];
    helpers[new_index1] = helpers[index1];
    if let EdgeTreePtr::Node(edge) = edge_tree_pointers[new_index1] {
        unsafe { (*edge).index = new_index1; }
    }

    vertex_types[new_index2] = vertex_types[index2];
    edge_tree_pointers[new_index2] = edge_tree_pointers[index2];
    helpers[new_index2] = helpers[index2];
    if let EdgeTreePtr::Node(edge) = edge_tree_pointers[new_index2] {
        unsafe { (*edge).index = new_index2; }
    }
}