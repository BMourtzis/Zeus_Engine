use std::ops::{
    Add, AddAssign, 
    Div, DivAssign, 
    Mul, MulAssign, 
    Sub, SubAssign
};

/// Represents a 2D Vector
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    //Constants
    pub const X: Self = Vector2 { x: 1.0, y: 0.0 };
    pub const Y: Self = Vector2 { x: 0.0, y: 1.0 };

    pub fn new(
        x: f32,
        y: f32,
    ) -> Self {
        Vector2 { x, y }
    }

    pub fn copy(v: Vector2) -> Self {
        Vector2 { x: v.x, y: v.y }
    }

    //Methods
    pub fn negate(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
    }

    pub fn magn(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn manhanttan_length(self) -> f32 {
        self.x + self.y
    }

    pub fn normalize(self) -> Vector2 {
        self / self.magn()
    }

    pub fn dot(
        self,
        rhs: Vector2,
    ) -> f32 {
        self * rhs
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(
        self,
        other: Vector2,
    ) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn add(
        self,
        rhs: &'b Vector2,
    ) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(
        self,
        other: Vector2,
    ) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn sub(
        self,
        rhs: &'b Vector2,
    ) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vector2 {
    type Output = f32;

    fn mul(
        self,
        rhs: Vector2,
    ) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<'a, 'b> Mul<&'b Vector2> for &'a Vector2 {
    type Output = f32;

    fn mul(
        self,
        rhs: &'b Vector2,
    ) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Vector2 {
    type Output = Vector2;

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(
        self,
        rhs: Vector2,
    ) -> Self::Output {
        Vector2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(
        &mut self,
        rhs: f32,
    ) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(
        self,
        rhs: f32,
    ) -> Self {
        let mut div = rhs;

        if div == 0.0 {
            error!("Trying to divide with zero!");
            div = std::f32::NAN;
        }

        Vector2 {
            x: self.x / div,
            y: self.y / div,
        }
    }
}

impl<'a> Div<f32> for &'a Vector2 {
    type Output = Vector2;

    fn div(
        self,
        rhs: f32,
    ) -> Self::Output {
        let mut div = rhs;

        if div == 0.0 {
            error!("Trying to divide with zero!");
            div = std::f32::NAN;
        }

        Vector2 {
            x: self.x / div,
            y: self.y / div,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(
        &mut self,
        rhs: f32,
    ) {
        let mut div = rhs;

        if div == 0.0 {
            error!("Trying to divide with zero!");
            div = std::f32::NAN;
        }

        self.x /= div;
        self.y /= div;
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector2;

    //Constructors
    #[test]
    fn default() {
        let vec = Vector2::default();

        const ZERO: f32 = 0.00;

        assert_eq!(vec.x, ZERO);
        assert_eq!(vec.y, ZERO);
    }

    #[test]
    fn new() {
        let vec = Vector2::new(34.0, -5.0);

        assert_eq!(vec.x, 34.0);
        assert_eq!(vec.y, -5.0);
    }

    //Methods
    #[test]
    fn negate() {
        let mut vec = Vector2::new(1.0, 1.0);

        vec.negate();

        assert_eq!(vec.x, -1.0);
        assert_eq!(vec.y, -1.0);
    }

    #[test]
    fn magnitude() {
        let vec = Vector2::new(3.0, 4.0);

        assert_eq!(vec.magn(), 5.0);
    }

    #[test]
    fn unit_magnitude() {
        let unit_x = Vector2::X;

        assert_eq!(unit_x.magn(), 1.0);
    }

    #[test]
    fn normalize() {
        let vec = Vector2::new(3.0, 4.0);

        assert_eq!(vec.normalize().magn(), 1.0);

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 4.0);
    }

    #[test]
    fn manhanttan_length() {
        let vec = Vector2::new(1.0, 2.0);

        assert_eq!(vec.manhanttan_length(), 3.0);

        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
    }

    // Operations
    #[test]
    fn sum() {
        let vec1 = Vector2::new(1.0, 1.0);
        let vec2 = Vector2::new(2.0, 2.0);

        let sum_vec = vec1 + vec2;

        //Check sum
        assert_eq!(sum_vec.x, 3.0);
        assert_eq!(sum_vec.y, 3.0);

        //Check existing vectors
        assert_eq!(vec1.x, 1.0);
        assert_eq!(vec1.y, 1.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
    }

    #[test]
    fn sum_ref() {
        let vec1 = &Vector2::new(1.0, 1.0);
        let vec2 = &Vector2::new(2.0, 2.0);

        let sum_vec = vec1 + vec2;

        //Check sum
        assert_eq!(sum_vec.x, 3.0);
        assert_eq!(sum_vec.y, 3.0);

        //Check existing vectors
        assert_eq!(vec1.x, 1.0);
        assert_eq!(vec1.y, 1.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
    }

    #[test]
    fn sum_assign() {
        let mut vec1 = Vector2::new(1.0, 1.0);
        let vec2 = Vector2::new(2.0, 2.0);

        vec1 += vec2;

        //Check sum
        assert_eq!(vec1.x, 3.0);
        assert_eq!(vec1.y, 3.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
    }

    #[test]
    fn sub() {
        let vec1 = Vector2::new(2.0, 3.0);
        let vec2 = Vector2::new(3.0, 1.0);

        let sub = vec1 - vec2;

        assert_eq!(sub.x, -1.0);
        assert_eq!(sub.y, 2.0);

        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec2.x, 3.0);
    }

    #[test]
    fn sub_ref() {
        let vec1 = &Vector2::new(2.0, 3.0);
        let vec2 = &Vector2::new(3.0, 1.0);

        let sub = vec1 - vec2;

        assert_eq!(sub.x, -1.0);
        assert_eq!(sub.y, 2.0);

        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec2.x, 3.0);
    }

    #[test]
    fn sub_assign() {
        let mut vec1 = Vector2::new(3.0, 4.0);
        let vec2 = Vector2::new(10.0, 3.0);

        vec1 -= vec2;

        assert_eq!(vec1.x, -7.0);
        assert_eq!(vec1.y, 1.0);

        assert_eq!(vec2.x, 10.0);
    }

    #[test]
    fn dot_product() {
        let vec1 = Vector2::new(2.0, 2.0);
        let vec2 = Vector2::new(1.0, 0.0);

        let dot = vec1 * vec2;

        //Check dot product
        assert_eq!(dot, 2.0);

        //Check existing vectors
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 2.0);

        assert_eq!(vec2.x, 1.0);
        assert_eq!(vec2.y, 0.0);
    }

    #[test]
    fn dot_product_ref() {
        let vec1 = &Vector2::new(2.0, 2.0);
        let vec2 = &Vector2::new(1.0, 0.0);

        let dot = vec1 * vec2;

        //Check dot product
        assert_eq!(dot, 2.0);

        //Check existing vectors
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 2.0);

        assert_eq!(vec2.x, 1.0);
        assert_eq!(vec2.y, 0.0);
    }

    #[test]
    fn mul() {
        let vec = Vector2::new(1.0, 1.0);

        let new_vec = vec * 3.0;

        //Check Vector
        assert_eq!(new_vec.x, 3.0);
        assert_eq!(new_vec.y, 3.0);

        //Check existing
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 1.0);
    }

    #[test]
    fn mul_ref() {
        let vec = &Vector2::new(1.0, 1.0);

        let new_vec = vec * 3.0;

        //Check Vector
        assert_eq!(new_vec.x, 3.0);
        assert_eq!(new_vec.y, 3.0);

        //Check existing
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 1.0);
    }

    #[test]
    fn mul_assign() {
        let mut vec = Vector2::new(2.0, 1.0);

        vec *= 3.0;

        assert_eq!(vec.x, 6.0);
        assert_eq!(vec.y, 3.0);
    }

    #[test]
    fn number_mul() {
        let vec1 = Vector2::new(3.0, 3.0);

        let div = 3.0 * vec1;

        assert_eq!(div.x, 9.0);
        assert_eq!(div.y, 9.0);

        assert_eq!(vec1.x, 3.0);
        assert_eq!(vec1.y, 3.0);
    }

    #[test]
    fn div() {
        let vec = Vector2::new(3.0, 2.0);

        let div = vec / 2.0;

        assert_eq!(div.x, 1.5);
        assert_eq!(div.y, 1.0);

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 2.0);
    }

    #[test]
    fn div_by_zero() {
        let vec = Vector2::new(3.0, 2.0);

        let div = vec / 0.0;

        assert!(f32::is_nan(div.x));
        assert!(f32::is_nan(div.y));

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 2.0);
    }

    #[test]
    fn div_assign() {
        let mut vec = Vector2::new(3.0, 6.0);

        vec /= 3.0;

        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
    }

    #[test]
    fn div_assign_by_zero() {
        let mut vec = Vector2::new(3.0, 6.0);

        vec /= 0.0;

        assert!(f32::is_nan(vec.x));
        assert!(f32::is_nan(vec.y));
    }
}
