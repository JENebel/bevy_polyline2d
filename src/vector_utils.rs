use bevy::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Left,
    Right,
    Straight,
}

/// Returns the orientation of the "turn" in p2, in points p1, p2, p3.
pub fn orientation_test(p1: Vec2, p2: Vec2, p3: Vec2) -> Orientation {
    let v1 = p2 - p1;
    let v2 = p3 - p2;
    let det = v1.perp_dot(v2);
    if det > f32::EPSILON {
        Orientation::Left
    } else if det < -f32::EPSILON {
        Orientation::Right
    } else {
        Orientation::Straight
    }
}

/// Returns the intersection point of two lines defined by a point and a direction vector.
/// Returns None if the lines are parallel or coincident.
pub fn intersection_point(p1: Vec2, d1: Vec2, p2: Vec2, d2: Vec2) -> Option<Vec2> {
    let det = d1.perp_dot(d2);

    if det.abs() < 1e-10 {
        return None; // Lines are parallel or coincident
    }

    let delta = Vec2::new(
        p2.x - p1.x,
        p2.y - p1.y
    );

    let t = delta.perp_dot(d2) / det;

    let intersection = Vec2::new(
        p1.x + t * d1.x,
        p1.y + t * d1.y
    );

    Some(intersection)
}

pub fn calc_left_side_segment(p1: Vec2, p2: Vec2, width: f32) -> (Vec2, Vec2) {
    let vec = p2 - p1;
    let perp = vec.perp().normalize();
    let start = p1 + perp * width;
    let end = p2 + perp * width;
    (start, end)
}

pub fn calc_right_side_segment(p1: Vec2, p2: Vec2, width: f32) -> (Vec2, Vec2) {
    let reverse_side = calc_left_side_segment(p2, p1, width);
    (reverse_side.1, reverse_side.0)
}
pub fn project_point_onto_line(p: Vec2, p1: Vec2, p2: Vec2) -> Vec2 {
    let v = p2 - p1;
    let w = p - p1;
    let proj_factor = w.dot(v) / v.dot(v);
    let p_proj = p1 + v * proj_factor;
    p_proj
}

#[test]
fn test_inter1() {
    let p1 = Vec2::new(0., 0.);
    let d1 = Vec2::new(1., 0.);
    let p2 = Vec2::new(2., 2.);
    let d2 = Vec2::new(0., -1.);

    let res = intersection_point(p1, d1, p2, d2).unwrap();
    assert_eq!(res, Vec2::new(2., 0.));
}

#[test]
fn test_inter2() {
    let p1 = Vec2::new(0., 0.);
    let d1 = Vec2::new(1., 0.);
    let p2 = Vec2::new(2., 2.);
    let d2 = Vec2::new(-1., 0.);

    let res = intersection_point(p1, d1, p2, d2);
    assert_eq!(res, None);
}

#[test]
fn test_inter3() {
    let p1 = Vec2::new(0., 0.);
    let d1 = Vec2::new(10., 0.);
    let p2 = Vec2::new(-1., 2.);
    let d2 = Vec2::new(1., -1.);

    let res = intersection_point(p1, d1, p2, d2).unwrap();
    assert_eq!(res, Vec2::new(1., 0.));
}