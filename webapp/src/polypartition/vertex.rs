use visioncortex::PointF64;

#[derive(Clone, Debug, Default)]
pub struct PartitionVertex {
    pub info: PartitionVertexInfo,
    // Indices of the corresponding vertex node in the Vec
    pub previous: usize,
    pub next: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartitionVertexInfo {
    pub is_active: bool,
    pub is_convex: bool,
    pub is_ear: bool,

    /// The point at the vertex
    pub p: PointF64,
    pub angle: f64,
}

impl PartitionVertex {
    /// Get the information stored in this PartitionVertex
    pub fn get_info(&self) -> PartitionVertexInfo {
        self.info.clone()
    }

    pub fn set_info(&mut self, info: PartitionVertexInfo) {
        self.info = info;
    }
}