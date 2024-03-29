use crate::math::Vector4;

use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Point4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Point4 {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> Self {
        Point4 { x, y, z, w }
    }

    pub fn from_vector(vec: &Vector4) -> Self {
        Point4 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: vec.w,
        }
    }

    pub fn distance(
        self,
        rhs: Point4,
    ) -> f32 {
        (self - rhs).magn()
    }
}

impl Sub for Point4 {
    type Output = Vector4;

    fn sub(
        self,
        rhs: Point4,
    ) -> Self::Output {
        Vector4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Default for Point4 {
    fn default() -> Self {
        Point4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
        use crate::math::{point::Point4, vector::Vector4};

        #[test]
        fn new() {
            let p = Point4::new(1.0, 2.0, 3.0, 4.0);

            assert_eq!(p.x, 1.0);
            assert_eq!(p.y, 2.0);
            assert_eq!(p.z, 3.0);
            assert_eq!(p.w, 4.0);
        }

        #[test]
        fn default() {
            let p = Point4::default();

            assert_eq!(p.x, 0.0);
            assert_eq!(p.y, 0.0);
            assert_eq!(p.z, 0.0);
            assert_eq!(p.w, 0.0);
        }

        #[test]
        fn from_vector() {
            let vec = Vector4::new(2.0, 3.0, 4.0, 5.0);

            let p = Point4::from_vector(&vec);

            assert_eq!(p.x, 2.0);
            assert_eq!(p.y, 3.0);
            assert_eq!(p.z, 4.0);
            assert_eq!(p.w, 5.0);

            assert_eq!(vec.x, 2.0);
            assert_eq!(vec.y, 3.0);
            assert_eq!(vec.z, 4.0);
            assert_eq!(vec.w, 5.0);
        }

        //Methods

        #[test]
        fn distance() {
            let p1 = Point4::new(3.0, 4.0, 3.0, 56.0);
            let p2 = Point4::new(0.0, 8.0, 3.0, 56.0);

            let dis = p1.distance(p2);

            assert_eq!(dis, 5.0);
        }

        //Opearators

        #[test]
        fn sub() {
            let p1 = Point4::new(3.0, 4.0, 5.0, 12.0);
            let p2 = Point4::new(0.0, 8.0, -0.1, 2.05);

            let vec = p1 - p2;

            assert_eq!(vec.x, 3.0);
            assert_eq!(vec.y, -4.0);
            assert_eq!(vec.z, 5.1);
            assert_eq!(vec.w, 9.95);
        }
}
