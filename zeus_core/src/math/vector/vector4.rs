use std::ops::{
    Add, AddAssign, 
    Div, DivAssign, 
    Mul, MulAssign, 
    Sub, SubAssign
};

use crate::math::{Matrix4, Vector3};

/// Represents a 4D Vector
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    //Constants
    pub const X: Self = Vector4 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const Y: Self = Vector4 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    };
    pub const Z: Self = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 0.0,
    };
    pub const W: Self = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> Self {
        Vector4 { x, y, z, w }
    }

    pub fn zero() -> Self {
        Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn from_vector(vec: &Vector4) -> Vector4 {
        Vector4 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: vec.w,
        }
    }

    pub fn from_vector3(vec: &Vector3) -> Vector4 {
        Vector4 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: 1.0,
        }
    }

    //Methods
    pub fn negate(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
        self.w *= -1.0;
    }

    pub fn magn(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn manhanttan_length(&self) -> f32 {
        self.x + self.y + self.z + self.w
    }

    pub fn normalize(&self) -> Vector4 {
        self / self.magn()
    }

    pub fn dot(
        &self,
        rhs: &Vector4,
    ) -> f32 {
        self * rhs
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(
        self,
        other: Vector4,
    ) -> Self::Output {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<'a, 'b> Add<&'b Vector4> for &'a Vector4 {
    type Output = Vector4;

    fn add(
        self,
        rhs: &'b Vector4,
    ) -> Self::Output {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign for Vector4 {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w
    }
}

impl Sub for Vector4 {
    type Output = Self;

    fn sub(
        self,
        other: Vector4,
    ) -> Self::Output {
        Vector4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<'a, 'b> Sub<&'b Vector4> for &'a Vector4 {
    type Output = Vector4;

    fn sub(
        self,
        rhs: &'b Vector4,
    ) -> Self::Output {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign for Vector4 {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w
    }
}

impl Mul for Vector4 {
    type Output = f32;

    fn mul(
        self,
        rhs: Vector4,
    ) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl MulAssign<Matrix4> for Vector4 {
    fn mul_assign(
        &mut self,
        rhs: Matrix4,
    ) {
        let new_x = self.x * rhs[0] + self.y * rhs[1] + self.z * rhs[2] + self.w * rhs[3];
        let new_y = self.x * rhs[4] + self.y * rhs[5] + self.z * rhs[6] + self.w * rhs[7];
        let new_z = self.x * rhs[8] + self.y * rhs[9] + self.z * rhs[10] + self.w * rhs[11];
        let new_w = self.x * rhs[12] + self.y * rhs[13] + self.z * rhs[14] + self.w * rhs[15];

        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
        self.w = new_w;
    }
}

impl<'a, 'b> Mul<&'b Vector4> for &'a Vector4 {
    type Output = f32;

    fn mul(
        self,
        rhs: &'b Vector4,
    ) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(
        self,
        rhs: f32,
    ) -> Self {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Vector4 {
    type Output = Vector4;

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(
        self,
        rhs: Vector4,
    ) -> Self::Output {
        Vector4 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
            w: rhs.w * self,
        }
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(
        &mut self,
        rhs: f32,
    ) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Div<f32> for Vector4 {
    type Output = Self;

    fn div(
        self,
        rhs: f32,
    ) -> Self::Output {
        let mut div = rhs;

        if div == 0.0 {
            error!("Trying to divide with zero!");
            div = std::f32::NAN;
        }

        Vector4 {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
            w: self.w / div,
        }
    }
}

impl<'a> Div<f32> for &'a Vector4 {
    type Output = Vector4;

    fn div(
        self,
        rhs: f32,
    ) -> Self::Output {
        let mut div = rhs;

        if div == 0.0 {
            error!("Trying to divide with zero!");
            div = std::f32::NAN;
        }

        Vector4 {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
            w: self.w / div,
        }
    }
}

impl DivAssign<f32> for Vector4 {
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
        self.w /= div;
    }
}

impl Default for Vector4 {
    fn default() -> Self {
        Vector4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::math::{Vector3, Vector4};

    //Constructors
    #[test]
    fn default() {
        let vec = Vector4::default();

        const ONE: f32 = 1.0;

        assert_eq!(vec.x, ONE);
        assert_eq!(vec.y, ONE);
        assert_eq!(vec.z, ONE);
        assert_eq!(vec.w, ONE);
    }

    #[test]
    fn new() {
        let vec = Vector4::new(34.0, -5.0, 135.353_245, 0.0);

        assert_eq!(vec.x, 34.0);
        assert_eq!(vec.y, -5.0);
        assert_eq!(vec.z, 135.353_245);
        assert_eq!(vec.w, 0.0);
    }

    #[test]
    fn zero() {
        let vec = Vector4::zero();

        const ZERO: f32 = 0.0;

        assert_eq!(vec.x, ZERO);
        assert_eq!(vec.y, ZERO);
        assert_eq!(vec.z, ZERO);
        assert_eq!(vec.w, ZERO);
    }

    #[test]
    fn from_vector3() {
        let vec3 = Vector3::new(3.0, 4.0, 5.0);

        let vec4 = Vector4::from_vector3(&vec3);

        assert_eq!(vec4.x, 3.0);
        assert_eq!(vec4.y, 4.0);
        assert_eq!(vec4.z, 5.0);
        assert_eq!(vec4.w, 1.0);

        assert_eq!(vec3.x, 3.0);
        assert_eq!(vec3.y, 4.0);
        assert_eq!(vec3.z, 5.0);
    }

    //Methods
    #[test]
    fn negate() {
        let mut vec = Vector4::new(1.0, 1.0, 1.0, 1.0);

        vec.negate();

        assert_eq!(vec.x, -1.0);
        assert_eq!(vec.y, -1.0);
        assert_eq!(vec.z, -1.0);
        assert_eq!(vec.w, -1.0);
    }

    #[test]
    fn magnitude() {
        let vec = Vector4::new(3.0, 4.0, 0.0, 0.0);

        assert_eq!(vec.magn(), 5.0);
    }

    #[test]
    fn unit_magnitude() {
        let unit_x = Vector4::X;

        assert_eq!(unit_x.magn(), 1.0);
    }

    #[test]
    fn normalize() {
        let vec = Vector4::new(2.0, 2.0, 2.0, 2.0);

        assert_eq!(vec.normalize().magn(), 1.0);

        assert_eq!(vec.x, 2.0);
    }

    #[test]
    fn manhanttan_length() {
        let vec = Vector4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(vec.manhanttan_length(), 10.0);

        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
        assert_eq!(vec.w, 4.0);
    }

    // Operations
    #[test]
    fn sum() {
        let vec1 = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let vec2 = Vector4::new(2.0, 2.0, 2.0, 2.0);

        let sum_vec = vec1 + vec2;

        //Check sum
        assert_eq!(sum_vec.x, 3.0);
        assert_eq!(sum_vec.y, 3.0);
        assert_eq!(sum_vec.z, 3.0);
        assert_eq!(sum_vec.w, 3.0);

        //Check existing vectors
        assert_eq!(vec1.x, 1.0);
        assert_eq!(vec1.y, 1.0);
        assert_eq!(vec1.z, 1.0);
        assert_eq!(vec1.w, 1.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
        assert_eq!(vec2.z, 2.0);
        assert_eq!(vec2.w, 2.0);
    }

    #[test]
    fn sum_ref() {
        let vec1 = &Vector4::new(1.0, 1.0, 1.0, 1.0);
        let vec2 = &Vector4::new(2.0, 2.0, 2.0, 2.0);

        let sum_vec = vec1 + vec2;

        //Check sum
        assert_eq!(sum_vec.x, 3.0);
        assert_eq!(sum_vec.y, 3.0);
        assert_eq!(sum_vec.z, 3.0);
        assert_eq!(sum_vec.w, 3.0);

        //Check existing vectors
        assert_eq!(vec1.x, 1.0);
        assert_eq!(vec1.y, 1.0);
        assert_eq!(vec1.z, 1.0);
        assert_eq!(vec1.w, 1.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
        assert_eq!(vec2.z, 2.0);
        assert_eq!(vec2.w, 2.0);
    }

    #[test]
    fn sum_assign() {
        let mut vec1 = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let vec2 = Vector4::new(2.0, 2.0, 2.0, 2.0);

        vec1 += vec2;

        //Check sum
        assert_eq!(vec1.x, 3.0);
        assert_eq!(vec1.y, 3.0);
        assert_eq!(vec1.z, 3.0);
        assert_eq!(vec1.w, 3.0);

        assert_eq!(vec2.x, 2.0);
        assert_eq!(vec2.y, 2.0);
        assert_eq!(vec2.z, 2.0);
        assert_eq!(vec2.w, 2.0);
    }

    #[test]
    fn sub() {
        let vec1 = Vector4::new(2.0, 3.0, 4.0, 5.0);
        let vec2 = Vector4::new(3.0, 1.0, 1.0, 1.0);

        let sub = vec1 - vec2;

        assert_eq!(sub.x, -1.0);
        assert_eq!(sub.y, 2.0);
        assert_eq!(sub.z, 3.0);
        assert_eq!(sub.w, 4.0);

        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec2.x, 3.0);
    }

    #[test]
    fn sub_ref() {
        let vec1 = &Vector4::new(2.0, 3.0, 4.0, 5.0);
        let vec2 = &Vector4::new(3.0, 1.0, 1.0, 1.0);

        let sub = vec1 - vec2;

        assert_eq!(sub.x, -1.0);
        assert_eq!(sub.y, 2.0);
        assert_eq!(sub.z, 3.0);
        assert_eq!(sub.w, 4.0);

        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec2.x, 3.0);
    }

    #[test]
    fn sub_assign() {
        let mut vec1 = Vector4::new(3.0, 4.0, 5.0, 6.0);
        let vec2 = Vector4::new(10.0, 3.0, 6.0, 4.0);

        vec1 -= vec2;

        assert_eq!(vec1.x, -7.0);
        assert_eq!(vec1.y, 1.0);
        assert_eq!(vec1.z, -1.0);
        assert_eq!(vec1.w, 2.0);

        assert_eq!(vec2.x, 10.0);
    }

    #[test]
    fn dot_product() {
        let vec1 = Vector4::new(2.0, 2.0, 2.0, 2.0);
        let vec2 = Vector4::new(1.0, 0.0, 0.0, 0.0);

        let dot = vec1 * vec2;

        //Check dot product
        assert_eq!(dot, 2.0);

        //Check existing vectors
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 2.0);
        assert_eq!(vec1.z, 2.0);
        assert_eq!(vec1.w, 2.0);

        assert_eq!(vec2.x, 1.0);
        assert_eq!(vec2.y, 0.0);
        assert_eq!(vec2.z, 0.0);
        assert_eq!(vec2.w, 0.0);
    }

    #[test]
    fn dot_product_ref() {
        let vec1 = &Vector4::new(2.0, 2.0, 2.0, 2.0);
        let vec2 = &Vector4::new(1.0, 0.0, 0.0, 0.0);

        let dot = vec1 * vec2;

        //Check dot product
        assert_eq!(dot, 2.0);

        //Check existing vectors
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 2.0);
        assert_eq!(vec1.z, 2.0);
        assert_eq!(vec1.w, 2.0);

        assert_eq!(vec2.x, 1.0);
        assert_eq!(vec2.y, 0.0);
        assert_eq!(vec2.z, 0.0);
        assert_eq!(vec2.w, 0.0);
    }

    #[test]
    fn mul() {
        let vec = Vector4::new(1.0, 1.0, 1.0, 1.0);

        let new_vec = vec * 3.0;

        //Check Vector
        assert_eq!(new_vec.x, 3.0);
        assert_eq!(new_vec.y, 3.0);
        assert_eq!(new_vec.z, 3.0);
        assert_eq!(new_vec.w, 3.0);

        //Check existing
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 1.0);
        assert_eq!(vec.z, 1.0);
        assert_eq!(vec.w, 1.0);
    }

    #[test]
    fn mul_ref() {
        let vec = &Vector4::new(1.0, 1.0, 1.0, 1.0);

        let new_vec = vec * 3.0;

        //Check Vector
        assert_eq!(new_vec.x, 3.0);
        assert_eq!(new_vec.y, 3.0);
        assert_eq!(new_vec.z, 3.0);
        assert_eq!(new_vec.w, 3.0);

        //Check existing
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 1.0);
        assert_eq!(vec.z, 1.0);
        assert_eq!(vec.w, 1.0);
    }

    #[test]
    fn mul_assign() {
        let mut vec = Vector4::new(2.0, 1.0, 3.0, 4.0);

        vec *= 3.0;

        assert_eq!(vec.x, 6.0);
        assert_eq!(vec.y, 3.0);
        assert_eq!(vec.z, 9.0);
        assert_eq!(vec.w, 12.0);
    }

    #[test]
    fn number_mul() {
        let vec1 = Vector4::new(3.0, 3.0, 3.0, 3.0);

        let div = 3.0 * vec1;

        assert_eq!(div.x, 9.0);
        assert_eq!(div.y, 9.0);
        assert_eq!(div.z, 9.0);
        assert_eq!(div.w, 9.0);

        assert_eq!(vec1.x, 3.0);
        assert_eq!(vec1.y, 3.0);
        assert_eq!(vec1.z, 3.0);
        assert_eq!(vec1.w, 3.0);
    }

    #[test]
    fn div() {
        let vec = Vector4::new(3.0, 2.0, 44.0, 5.0);

        let div = vec / 2.0;

        assert_eq!(div.x, 1.5);
        assert_eq!(div.y, 1.0);
        assert_eq!(div.z, 22.0);
        assert_eq!(div.w, 2.5);

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 44.0);
        assert_eq!(vec.w, 5.0);
    }

    #[test]
    fn div_by_zero() {
        let vec = Vector4::new(3.0, 2.0, 44.0, 5.0);

        let div = vec / 0.0;

        assert!(f32::is_nan(div.x));
        assert!(f32::is_nan(div.y));
        assert!(f32::is_nan(div.z));
        assert!(f32::is_nan(div.w));

        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 44.0);
        assert_eq!(vec.w, 5.0);
    }

    #[test]
    fn div_assign() {
        let mut vec = Vector4::new(3.0, 6.0, 9.0, 12.0);

        vec /= 3.0;

        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
        assert_eq!(vec.w, 4.0);
    }

    #[test]
    fn div_assign_by_zero() {
        let mut vec = Vector4::new(3.0, 6.0, 9.0, 12.0);

        vec /= 0.0;

        assert!(f32::is_nan(vec.x));
        assert!(f32::is_nan(vec.y));
        assert!(f32::is_nan(vec.z));
        assert!(f32::is_nan(vec.w));
    }
}