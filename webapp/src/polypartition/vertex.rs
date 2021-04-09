use std::rc::Rc;

use visioncortex::PointF64;

#[derive(Default)]
pub struct PartitionVertex {
    pub is_active: bool,
    pub is_convex: bool,
    pub is_ear: bool,

    /// The point at the vertex
    pub p: PointF64,
    pub angle: f64,
    pub previous: Rc<PartitionVertex>,
    pub next: Rc<PartitionVertex>,
}