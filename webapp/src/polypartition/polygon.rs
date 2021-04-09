use visioncortex::PointF64;

use super::Orientation;

/// Common properties/methods for any polygons.
///
/// To be composed into any structs that represents a polygon
#[derive(Clone, Debug, Default)]
pub struct PolygonProps {
    pub points: Vec<PointF64>,
    pub is_hole: bool,
}

pub trait PolygonInterface {
    fn props(&self) -> &PolygonProps;
    fn props_mut(&mut self) -> &mut PolygonProps;
}

#[derive(Clone, Debug, Default)]
// The most basic and generic polygon class
pub struct Polygon {
    props: PolygonProps,
}

impl PolygonInterface for Polygon {
    fn props(&self) -> &PolygonProps {
        &self.props
    }

    fn props_mut(&mut self) -> &mut PolygonProps {
        &mut self.props
    }
}

impl Polygon {
    pub fn from_points_and_is_hole(points: Vec<PointF64>, is_hole: bool) -> Self {
        Self {
            props: PolygonProps::from_points_and_is_hole(points, is_hole)
        }
    }

    /// Convert self to a triangle with given vertices, keeping is_hole untouched
    #[allow(clippy::wrong_self_convention)]
    pub fn to_triangle(&mut self, p1: PointF64, p2: PointF64, p3: PointF64) {
        self.props.points = vec![p1, p2, p3];
    }

    pub fn is_valid(&self) -> bool {
        self.props.is_valid()
    }
}

impl PolygonProps {
    pub fn from_points_and_is_hole(points: Vec<PointF64>, is_hole: bool) -> Self {
        Self {
            points,
            is_hole
        }
    }

    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    pub fn get_point_safe(&self, i: usize) -> Option<PointF64> {
        if i < self.num_points() {
            Some(self.points[i])
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.points.clear()
    }

    /// Allocate memory. Does not affect is_hole.
    pub fn init(&mut self, num_points: usize) {
        self.points = vec![PointF64::default(); num_points];
    }

    /// Invert the order of vertices
    pub fn invert(&mut self) {
        self.points.reverse()
    }

    pub fn get_orientation(&self) -> Orientation {
        let mut area = 0.0;
        let len = self.num_points();
        for curr in 0..len {
            let next = (curr+1) % len;
            let (curr_pt, next_pt) = (self.points[curr], self.points[next]);
            area += curr_pt.x * next_pt.y - curr_pt.y * next_pt.x;
        }

        if area.is_sign_positive() {
            Orientation::CounterClockwise
        } else if area.is_sign_negative() {
            Orientation::Clockwise
        } else {
            Orientation::None
        }
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        let poly_orientation = self.get_orientation();
        if poly_orientation != Orientation::None && poly_orientation != orientation {
            self.invert();
        }
    }

    /// A valid PolygonProps should have at least 3 points
    pub fn is_valid(&self) -> bool {
        self.num_points() >= 3
    }
}