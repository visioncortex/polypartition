use visioncortex::PointF64;

pub fn is_convex(p1: &PointF64, p2: &PointF64, p3: &PointF64) -> bool {
    let tmp = (p3.y - p1.y) * (p2.x - p1.x) - (p3.x - p1.x) * (p2.y - p1.y);
    tmp.is_sign_positive()
}

pub fn is_reflex(p1: &PointF64, p2: &PointF64, p3: &PointF64) -> bool {
    let tmp = (p3.y - p1.y) * (p2.x - p1.x) - (p3.x - p1.x) * (p2.y - p1.y);
    tmp.is_sign_negative()
}

pub fn is_inside(p1: &PointF64, p2: &PointF64, p3: &PointF64, p: &PointF64) -> bool {
    !(is_convex(p1, p, p2) || is_convex(p2, p, p3) || is_convex(p3, p, p1))
}

pub fn is_in_cone(p1: &PointF64, p2: &PointF64, p3: &PointF64, p: &PointF64) -> bool {
    if is_convex(p1, p2, p3) {
        if !is_convex(p1, p2, p) || !is_convex(p2, p3, p) {
            return false;
        }
        true
    } else {
        if is_convex(p1, p2, p) || is_convex(p2, p3, p) {
            return true;
        }
        false
    }
}

pub fn normalize(p: &PointF64) -> PointF64 {
    let norm = p.norm();
    if norm != 0.0 {
        PointF64::new(p.x / norm, p.y / norm)
    } else {
        PointF64::new(0.0, 0.0)
    }
}

pub fn distance(p1: &PointF64, p2: &PointF64) -> f64 {
    (*p1 - *p2).norm()
}

pub fn intersects(p11: &PointF64, p12: &PointF64, p21: &PointF64, p22: &PointF64) -> bool {
    let (p11, p12, p21, p22) = (*p11, *p12, *p21, *p22);

    if p11 == p21 || p11 == p22 || p12 == p21 || p12 == p22 {
        return false;
    }
    
    let v1ort = PointF64::new(p12.y - p11.y, p11.x - p12.x);
    let v2ort = PointF64::new(p22.y - p21.y, p21.x - p22.x);

    let v = p21 - p11;
    let dot21 = v.dot(v1ort);

    let v = p22 - p11;
    let dot22 = v.dot(v1ort);

    let v = p11 - p21;
    let dot11 = v.dot(v2ort);

    let v = p12 - p21;
    let dot12 = v.dot(v2ort);

    !((dot11 * dot12).is_sign_positive() || (dot21 * dot22).is_sign_positive())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point_f64_approximately(p1: PointF64, p2: PointF64) -> bool {
        let epsilon = 1e-7;
        let diff = p1 - p2;
        diff.x < epsilon && diff.y < epsilon
    }

    #[test]
    fn util_general_point() {
        let p = PointF64::new(1.0, 0.0);
        assert!(normalize(&p) == p);
        let p = PointF64::new(1.0, 1.0);
        assert!(point_f64_approximately(normalize(&p), PointF64::new(1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt())));
        let p = PointF64::new(3.0, 3.0);
        assert!(point_f64_approximately(normalize(&p), PointF64::new(1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt())));
    }

    #[test]
    fn util_general_is_inside() {
        let p1 = &PointF64::new(-1.0, -1.0);
        let p2 = &PointF64::new(1.0, -1.0);
        let p3 = &PointF64::new(0.0, 1.0);
        let p = &PointF64::new(0.0, 0.5);

        assert!(is_inside(p1, p2, p3, p));

        let p1 = &PointF64::new(-1.0, -1.0);
        let p2 = &PointF64::new(1.0, -1.0);
        let p3 = &PointF64::new(0.0, 1.0);
        let p = &PointF64::new(0.0, 9.0);

        assert!(!is_inside(p1, p2, p3, p));
    }

    #[test]
    fn util_general_intersects() {
        let p11 = &PointF64::new(-1.0, 0.0);
        let p12 = &PointF64::new(1.0, 0.0);
        let p21 = &PointF64::new(0.0, -1.0);
        let p22 = &PointF64::new(0.0, 1.0);

        assert!(intersects(p11, p12, p21, p22));

        let p11 = &PointF64::new(-1.0, 0.0);
        let p12 = &PointF64::new(1.0, 0.0);
        let p21 = &PointF64::new(-1.0, -1.0);
        let p22 = &PointF64::new(1.0, -1.0);

        assert!(!intersects(p11, p12, p21, p22));
    }
}