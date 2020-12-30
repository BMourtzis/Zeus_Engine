use std::ops::{
    Add, AddAssign, 
    Div, DivAssign, 
    Mul, MulAssign, 
    Sub, SubAssign
};

use crate::math::Matrix3;

/// Represents a 3D Vector
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    //Constants
    pub const X: Self = Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: Self = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z: Self = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn new(
        x: f32,
        y: f32,
        z: f32,
    ) -> Self {
        Vector3 { x, y, z }
    }

    pub fn from_vector(v: &Vector3) -> Self {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    //Methods
    pub fn negate(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
    }

    pub fn magn(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn manhanttan_length(&self) -> f32 {
        self.x + self.y + self.z
    }

    pub fn normalize(&self) -> Vector3 {
        self / self.magn()
    }

    pub fn dot(
        &self,
        rhs: &Vector3,
    ) -> f32 {
        self * rhs
    }

    pub fn cross(
        &self,
        rhs: &Vector3,
    ) -> Vector3 {
        Vector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn triple_product(
        a: &Vector3,
        b: &Vector3,
        c: &Vector3,
    ) -> f32 {
        b.cross(c).dot(a)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(
        self,
        other: Vector3,
    ) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(
        self,
        rhs: &'b Vector3,
    ) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(
        self,
        other: Vector3,
    ) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(
        self,
        rhs: &'b Vector3,
    ) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vector3 {
    type Output = f32;

    fn mul(
        self,
        rhs: Vector3,
    ) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<'a, 'b> Mul<&'b Vector3> for &'a Vector3 {
    type Output = f32;

    fn mul(
        self,
        rhs: &'b Vector3,
    ) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<Matrix3> for Vector3 {
    fn mul_assign(
        &mut self,
        rhs: Matrix3,
    ) {
        let new_x = self.x * rhs[0] + self.y * rhs[3] + self.z * rhs[6];
        let new_y = self.x * rhs[1] + self.y * rhs[4] + self.z * rhs[7];
        let new_z = self.x * rhs[2] + self.y * rhs[5] + self.z * rhs[8];

        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }
}

impl<'a> Mul<f32> for &'a Vector3 {
    type Output = Vector3;

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(
        self,
        rhs: Vector3,
    ) -> Self::Output {
        Vector3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(
        &mut self,
        rhs: f32,
    ) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vector3 {
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

        Vector3 {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
        }
    }
}

impl<'a> Div<f32> for &'a Vector3 {
    type Output = Vector3;

    fn div(
        self,
        rhs: f32,
    ) -> Self::Output {
        let mut div = rhs;

        if div == 0.0 {
            error!("Trying to divide with zero!");
            div = std::f32::NAN;
        }

        Vector3 {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
        }
    }
}

impl DivAssign<f32> for Vector3 {
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
        self.z /= div;
    }
}

impl Eq for Vector3 {}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::math::{Vector3, Matrix3};

    //Constructors
    #[test]
    fn default() {
        let vec = Vector3::default();

        const ZERO: f32 = 0.00;

        assert_eq!(vec.x, ZERO);
        assert_eq!(vec.y, ZERO);
        assert_eq!(vec.z, ZERO);
    }

    #[test]
    fn new() {
        const X: f32 = 34.0;
        const Y: f32 = -5.0;
        const Z: f32 = 135.353_245;

        let vec = Vector3::new(X, Y, Z);

        assert_eq!(vec.x, X);
        assert_eq!(vec.y, Y);
        assert_eq!(vec.z, Z);
    }

    //Methods
    #[test]
    fn negate() {
        let mut vec = Vector3::new(1.0, 1.0, 1.0);

        vec.negate();

        assert_eq!(vec.x, -1.0);
        assert_eq!(vec.y, -1.0);
        assert_eq!(vec.z, -1.0);
    }

    #[test]
    fn magnitude() {
        let vec = Vector3::new(3.0, 4.0, 5.0);

        assert_eq!(vec.magn(), 50.0f32.sqrt());
    }

    #[test]
    fn unit_magnitude() {
        let unit_x = Vector3::X;

        assert_eq!(unit_x.magn(), 1.0);
    }

    #[test]
    fn normalize() {
        let vec = Vector3::new(2.0, 2.0, 2.0);

        assert_eq!(vec.normalize().magn().round(), 1.0);

        assert_eq!(vec.x, 2.0);
    }

    #[test]
    fn manhanttan_length() {
        let vec = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(vec.manhanttan_length(), 6.0);

        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    // Operations
    #[test]
    fn sum() {
        let vec1 = Vector3::new(1.0, 1.0, 1.0);
        let vec2 = Vector3::new(2.0, 2.0, 2.0);

        let sum_vec = vec1 + vec2;

        //Check sum
        assert_eq!(sum_vec.x, 3.0);
        assert_eq!(sum_vec.y, 3.0);
        assert_eq!(sum_vec.z, 3.0);

        //Check existing vectors
        assert_eq!(vec1.x, 1.0);
        assert_eq!(vec1.y, 1.0);
        assert_eq!(vec1.z, 1.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
        assert_eq!(vec2.z, 2.0);
    }

    #[test]
    fn sum_ref() {
        let vec1 = &Vector3::new(1.0, 1.0, 1.0);
        let vec2 = &Vector3::new(2.0, 2.0, 2.0);

        let sum_vec = vec1 + vec2;

        //Check sum
        assert_eq!(sum_vec.x, 3.0);
        assert_eq!(sum_vec.y, 3.0);
        assert_eq!(sum_vec.z, 3.0);

        //Check existing vectors
        assert_eq!(vec1.x, 1.0);
        assert_eq!(vec1.y, 1.0);
        assert_eq!(vec1.z, 1.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
        assert_eq!(vec2.z, 2.0);
    }

    #[test]
    fn sum_assign() {
        let mut vec1 = Vector3::new(1.0, 1.0, 1.0);
        let vec2 = Vector3::new(2.0, 2.0, 2.0);

        vec1 += vec2;

        //Check sum
        assert_eq!(vec1.x, 3.0);
        assert_eq!(vec1.y, 3.0);
        assert_eq!(vec1.z, 3.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
        assert_eq!(vec2.z, 2.0);
    }

    #[test]
    fn sub() {
        let vec1 = Vector3::new(2.0, 3.0, 4.0);
        let vec2 = Vector3::new(3.0, 1.0, 1.0);

        let sub = vec1 - vec2;

        assert_eq!(sub.x, -1.0);
        assert_eq!(sub.y, 2.0);
        assert_eq!(sub.z, 3.0);

        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec2.x, 3.0);
    }

    #[test]
    fn sub_ref() {
        let vec1 = &Vector3::new(2.0, 3.0, 4.0);
        let vec2 = &Vector3::new(3.0, 1.0, 1.0);

        let sub = vec1 - vec2;

        assert_eq!(sub.x, -1.0);
        assert_eq!(sub.y, 2.0);
        assert_eq!(sub.z, 3.0);

        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec2.x, 3.0);
    }

    #[test]
    fn sub_assign() {
        let mut vec1 = Vector3::new(3.0, 4.0, 5.0);
        let vec2 = Vector3::new(10.0, 3.0, 6.0);

        vec1 -= vec2;

        assert_eq!(vec1.x, -7.0);
        assert_eq!(vec1.y, 1.0);
        assert_eq!(vec1.z, -1.0);

        assert_eq!(vec2.x, 10.0);
    }

    #[test]
    fn dot_product() {
        let vec1 = Vector3::new(2.0, 2.0, 2.0);
        let vec2 = Vector3::new(1.0, 0.0, 0.0);

        let dot = vec1 * vec2;

        //Check dot product
        assert_eq!(dot, 2.0);

        //Check existing vectors
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 2.0);
        assert_eq!(vec1.z, 2.0);

        assert_eq!(vec2.x, 1.0);
        assert_eq!(vec2.y, 0.0);
        assert_eq!(vec2.z, 0.0);
    }

    #[test]
    fn dot_product_ref() {
        let vec1 = &Vector3::new(2.0, 2.0, 2.0);
        let vec2 = &Vector3::new(1.0, 0.0, 0.0);

        let dot = vec1 * vec2;

        //Check dot product
        assert_eq!(dot, 2.0);

        //Check existing vectors
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 2.0);
        assert_eq!(vec1.z, 2.0);

        assert_eq!(vec2.x, 1.0);
        assert_eq!(vec2.y, 0.0);
        assert_eq!(vec2.z, 0.0);
    }

    #[test]
    fn mul() {
        let vec = Vector3::new(1.0, 1.0, 1.0);

        let new_vec = vec * 3.0;

        //Check Vector
        assert_eq!(new_vec.x, 3.0);
        assert_eq!(new_vec.y, 3.0);
        assert_eq!(new_vec.z, 3.0);

        //Check existing
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 1.0);
        assert_eq!(vec.z, 1.0);
    }

    #[test]
    fn mul_ref() {
        let vec = &Vector3::new(1.0, 1.0, 1.0);

        let new_vec = vec * 3.0;

        //Check Vector
        assert_eq!(new_vec.x, 3.0);
        assert_eq!(new_vec.y, 3.0);
        assert_eq!(new_vec.z, 3.0);

        //Check existing
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 1.0);
        assert_eq!(vec.z, 1.0);
    }

    #[test]
    fn mul_assign() {
        let mut vec = Vector3::new(2.0, 1.0, 3.0);

        vec *= 3.0;

        assert_eq!(vec.x, 6.0);
        assert_eq!(vec.y, 3.0);
        assert_eq!(vec.z, 9.0);
    }

    #[test]
    fn mul_assign_with_matrix() {
        let mut vec = Vector3::new(3.0, 5.0, 6.0);

        let mut mat = Matrix3::new();

        for i in 0..9 {
            mat[i] = 1.0 + i as f32;
        }

        vec *= mat;

        assert_eq!(vec.x, 65.0);
        assert_eq!(vec.y, 79.0);
        assert_eq!(vec.z, 93.0);
    }

    #[test]
    fn number_mul() {
        let vec1 = Vector3::new(3.0, 3.0, 3.0);

        let div = 3.0 * vec1;

        assert_eq!(div.x, 9.0);
        assert_eq!(div.y, 9.0);
        assert_eq!(div.z, 9.0);

        assert_eq!(vec1.x, 3.0);
        assert_eq!(vec1.y, 3.0);
        assert_eq!(vec1.z, 3.0);
    }

    #[test]
    fn div() {
        let vec = Vector3::new(3.0, 2.0, 44.0);

        let div = vec / 2.0;

        assert_eq!(div.x, 1.5);
        assert_eq!(div.y, 1.0);
        assert_eq!(div.z, 22.0);

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 44.0);
    }

    #[test]
    fn div_by_zero() {
        let vec = Vector3::new(3.0, 2.0, 44.0);

        let div = vec / 0.0;

        assert!(f32::is_nan(div.x));
        assert!(f32::is_nan(div.y));
        assert!(f32::is_nan(div.z));

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 44.0);
    }

    #[test]
    fn div_assign() {
        let mut vec = Vector3::new(3.0, 6.0, 9.0);

        vec /= 3.0;

        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn div_assign_by_zero() {
        let mut vec = Vector3::new(3.0, 6.0, 9.0);

        vec /= 0.0;

        assert!(f32::is_nan(vec.x));
        assert!(f32::is_nan(vec.y));
        assert!(f32::is_nan(vec.z));
    }
}