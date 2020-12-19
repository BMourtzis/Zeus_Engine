use crate::math::Vector2;

use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub fn new(
        x: f32,
        y: f32,
    ) -> Self {
        Point2 { x, y }
    }

    pub fn from_vector(vec: Vector2) -> Self {
        Point2 { x: vec.x, y: vec.y }
    }

    pub fn distance(
        self,
        rhs: Point2,
    ) -> f32 {
        (self - rhs).magn()
    }
}

impl Sub for Point2 {
    type Output = Vector2;

    fn sub(
        self,
        rhs: Point2,
    ) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Default for Point2 {
    fn default() -> Self {
        Point2 { x: 0.0, y: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{point::Point2, vector::Vector2};

    #[test]
    fn new() {
        let p = Point2::new(1.0, 2.0);

        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn default() {
        let p = Point2::default();

        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn from_vector() {
        let vec = Vector2::new(2.0, 3.0);

        let p = Point2::from_vector(vec);

        assert_eq!(p.x, 2.0);
        assert_eq!(p.y, 3.0);

        assert_eq!(vec.x, 2.0);
        assert_eq!(vec.y, 3.0);
    }

    //Methods

    #[test]
    fn distance() {
        let p1 = Point2::new(3.0, 4.0);
        let p2 = Point2::new(0.0, 8.0);

        let dis = p1.distance(p2);

        assert_eq!(dis, 5.0);
    }

    //Opearators

    #[test]
    fn sub() {
        let p1 = Point2::new(3.0, 4.0);
        let p2 = Point2::new(0.0, 8.0);

        let vec = p1 - p2;

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, -4.0);
    }
}