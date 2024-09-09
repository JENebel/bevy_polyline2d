use bevy::math::{Vec2, Vec3};

pub trait VectorExtensions {
    fn determinant(&self, other: Self) -> f32;
    fn ortho_normal(&self) -> Self;
}

impl VectorExtensions for Vec3 {
    fn determinant(&self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    fn ortho_normal(&self) -> Self {
        let norm = self.normalize();
        Vec3::new(-norm.y, norm.x, self.z)
    }
}

impl VectorExtensions for Vec2 {
    fn determinant(&self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    fn ortho_normal(&self) -> Self {
        let norm = self.normalize();
        Vec2::new(-norm.y, norm.x)
    }
}

pub fn intersection_point(p1: Vec2, d1: Vec2, p2: Vec2, d2: Vec2) -> Option<Vec2> {
    let det = d1.determinant(d2);

    if det.abs() < 1e-10 {
        return None; // Lines are parallel or coincident
    }

    let delta = Vec2::new(
        p2.x - p1.x,
        p2.y - p1.y
    );

    let t = delta.determinant(d2) / det;

    let intersection = Vec2::new(
        p1.x + t * d1.x,
        p1.y + t * d1.y
    );

    Some(intersection)
}

pub fn intersection_point_legacy(p1: Vec3, v1: Vec3, p2: Vec3, v2: Vec3) -> Vec3 {
    let a1 = v1.y - p1.y;
    let b1 = p1.x - v1.x;
    let c1 = a1 * p1.x + b1 * p1.y;

    let a2 = v2.y - p2.y;
    let b2 = p2.x - v2.x;
    let c2 = a2 * p2.x + b2 * p2.y;

    let determinant = a1 * b2 - a2 * b1;

    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;
    return Vec3::from((x, y, 0.));
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