use crate::math::Vector3;

use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
    ) -> Self {
        Point3 { x, y, z }
    }

    pub fn from_vector(vec: &Vector3) -> Self {
        Point3 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }

    pub fn distance(
        self,
        rhs: Point3,
    ) -> f32 {
        (self - rhs).magn()
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(
        self,
        rhs: Point3,
    ) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Default for Point3 {
    fn default() -> Self {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{point::Point3, vector::Vector3};

    #[test]
    fn new() {
        let p = Point3::new(1.0, 2.0, 3.0);

        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn default() {
        let p = Point3::default();

        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    fn from_vector() {
        let vec = Vector3::new(2.0, 3.0, 4.0);

        let p = Point3::from_vector(&vec);

        assert_eq!(p.x, 2.0);
        assert_eq!(p.y, 3.0);
        assert_eq!(p.z, 4.0);

        assert_eq!(vec.x, 2.0);
        assert_eq!(vec.y, 3.0);
        assert_eq!(vec.z, 4.0);
    }

    //Methods

    #[test]
    fn distance() {
        let p1 = Point3::new(3.0, 4.0, 3.0);
        let p2 = Point3::new(0.0, 8.0, 3.0);

        let dis = p1.distance(p2);

        assert_eq!(dis, 5.0);
    }

    //Opearators

    #[test]
    fn sub() {
        let p1 = Point3::new(3.0, 4.0, 5.0);
        let p2 = Point3::new(0.0, 8.0, -0.1);

        let vec = p1 - p2;

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, -4.0);
        assert_eq!(vec.z, 5.1);
    }
}