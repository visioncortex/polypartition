#[derive(PartialEq)]
pub enum Orientation {
    Clockwise = -1,
    None = 0, // The polygon has no measurable area
    CounterClockwise = 1,
}

#[derive(Clone, Copy, Debug)]
pub enum VertexType {
    Null = -1,
    Regular = 0,
    Start = 1,
    End = 2,
    Split = 3,
    Merge = 4,
}