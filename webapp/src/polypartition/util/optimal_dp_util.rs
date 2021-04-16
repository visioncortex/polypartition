#[derive(Clone, Default)]
pub struct DPState {
    pub visible: bool,
    pub weight: f64,
    pub best_vertex: Option<usize>,
}

pub struct Diagonal {
    pub index_1: usize,
    pub index_2: usize,
}

impl Diagonal {
    pub fn new(index_1: usize, index_2: usize) -> Self {
        Self {
            index_1,
            index_2,
        }
    }
}