use crate::math::vector::{
    Vector2,
    Vector3,
    Vector4
};

use std::ops::{
    Index, IndexMut,
    Mul, MulAssign
};
use std::fmt;
use std::fmt::Display;

// region Matrix2
#[derive(Debug, Clone, Copy)]
pub struct Matrix2 {
    entries: [f32; 4]
}

impl Matrix2 {
    pub fn new() -> Matrix2 {
        Matrix2 {
            entries: [0.0; 4]
        }
    }

    pub fn identity() -> Matrix2 {
        let mut mat = Matrix2::new();

        mat[0] = 1.0;
        mat[3] = 1.0;

        mat
    }

    pub fn invert(&self) -> Matrix2 {
        let mut mat = Matrix2::new();

        for i in 0..4 {
            mat[i] = self[3 - i];
        }

        mat
    }

    pub fn transpose(&self) -> Matrix2 {
        let mut mat = Matrix2::new();

        for i in 0..2 {
            for j in 0..2 {
                mat[i * 2 + j] = self[j * 2 + i];
            }
        }

        mat
    }

    pub fn get_diagonal_vector(&self) -> Vector2 {
        Vector2::new(
            self[0],
            self[4]
        )
    }
}

impl Mul<f32> for Matrix2 {
    type Output = Matrix2;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut mat = Matrix2::new();

        for i in 0..4 {
            mat[i] = self[i]*rhs;
        }

        mat
    }
}

impl MulAssign<f32> for Matrix2 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..4 {
            self[i] = self[i]*rhs;
        }
    }
}

impl Mul<Matrix2> for f32 {
    type Output = Matrix2;

    fn mul(self, rhs: Matrix2) -> Self::Output {
        rhs * self
    }
}

impl Mul for Matrix2 {
    type Output = Matrix2;

    fn mul(self, rhs: Matrix2) -> Self::Output {
        let mut mat = Matrix2::new();

        for i in 0..4 {
            let row = (i / 2) * 2;
            let col = i % 2;

            mat[i] = (self[row] * rhs[col]) + (self[row + 1] * rhs[col + 2]);
        }

        mat
    }
}

impl Index<usize> for Matrix2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 3 {
            error!("Trying to reach out of bounds index");
            return &std::f32::NAN;
        }

        &self.entries[index]
    }
}

impl IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Default for Matrix2 {
    fn default() -> Self {
        Self::new()
    }
}

// endregion

// region Matrix3

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    entries: [f32; 9]
}

impl Matrix3 {
    pub fn new() -> Matrix3 {
        Matrix3 {
            entries: [0.0; 9]
        }
    }

    pub fn identity() -> Matrix3 {
        let mut result = Matrix3::new();

        result[0] = 1.0;
        result[4] = 1.0;
        result[8] = 1.0;

        result
    }

    pub fn from_2d_vectors(a: &Vector2, b: &Vector2) -> Matrix3 {
        let mut result = Matrix3::new();

        result[0] = a.x;
        result[1] = a.y;
        result[2] = 0.0;

        result[3] = b.x;
        result[4] = b.y;
        result[5] = 0.0;

        result[6] = 0.0;
        result[7] = 0.0;
        result[8] = 1.0;

        result
    }

    pub fn invert(&self) -> Matrix3 {
        let mut result = Matrix3::new();

        for i in 0..9 {
            result[i] = self[8 - i];
        }

        result
    }

    pub fn transpose(&self) -> Matrix3{
        let mut result = Matrix3::new();

        for i in 0..3 {
            for j in 0..3 {
                result[i * 3 + j] = self[j * 3 + i];
            }
        }

        result
    }

    pub fn get_diagonal_vector(&self) -> Vector3 {
        Vector3::new(self[0], self[4], self[8])
    }
}

impl Mul<f32> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut mat = Matrix3::new();

        for i in 0..9 {
            mat[i] = self[i]*rhs;
        }

        mat
    }
}

impl MulAssign<f32> for Matrix3 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..9 {
            self[i] = self[i]*rhs;
        }
    }
}

impl Mul<Matrix3> for f32 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        rhs * self
    }
}

impl Mul for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        let mut mat = Matrix3::new();

        for i in 0..9 {
            let row = (i / 3) * 3;
            let col = i % 3;

            mat[i] = (self[row] * rhs[col]) + (self[row + 1] * rhs[col + 3]) + (self[row + 2] * rhs[col + 6]);
        }

        mat
    }
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(
            self[0]*rhs.x + self[1]*rhs.y + self[2]*rhs.z,
            self[4]*rhs.x + self[5]*rhs.y + self[6]*rhs.z,
            self[8]*rhs.x + self[9]*rhs.y + self[10]*rhs.z,
        )
    }
}

impl Index<usize> for Matrix3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 8 {
            error!("Trying to reach out of bounds index");
            return &std::f32::NAN;
        }

        &self.entries[index]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Default for Matrix3 {
    fn default() -> Self
    {
        Matrix3::new()
    }
}

// endregion

// region Matrix4

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    entries: [f32; 16]
}

impl Matrix4 {
    pub fn zero() -> Matrix4 {
        Matrix4 { entries: [0.0; 16]}
    }

    pub fn new() -> Matrix4 {
        let mut result = Matrix4::zero();

        result[0] = 1.0;
        result[5] = 1.0;
        result[10] = 1.0;
        result[15] = 1.0;

        result
    }

    pub fn from_matrix3(mat: Matrix3) -> Matrix4 {
        let mut new_mat = Matrix4::new();

        new_mat[0] = mat[0];
        new_mat[1] = mat[1];
        new_mat[2] = mat[2];

        new_mat[4] = mat[3];
        new_mat[5] = mat[4];
        new_mat[6] = mat[5];

        new_mat[8] = mat[6];
        new_mat[9] = mat[7];
        new_mat[10] = mat[8];

        new_mat
    }

    pub fn from_3d_vectors(a: &Vector3, b: &Vector3, c: &Vector3) -> Matrix4 {
        let mut result = Matrix4::zero();

        result[0] = a.x;
        result[1] = a.y;
        result[2] = a.z;
        result[3] = 0.0;

        result[4] = b.x;
        result[5] = b.y;
        result[6] = b.z;
        result[7] = 0.0;

        result[8] = c.x;
        result[9] = c.y;
        result[10] = c.z;
        result[11] = 0.0;

        result[12] = 0.0;
        result[13] = 0.0;
        result[14] = 0.0;
        result[15] = 1.0;

        result
    }

    pub fn from_vector3(vec: &Vector3) -> Matrix4 {
        let mut mat = Matrix4::new();

        mat[3] = vec.x;
        mat[7] = vec.y;
        mat[11] = vec.z;

        mat
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
        let mut aspect = aspect;
        if aspect <= 0.0 {
            aspect = 1.0;
        }

        //TODO: make sure far > near

        let f = 1.0/(fov/2.0).tan();

        Matrix4 {
            entries: [
                f/aspect, 0.0, 0.0, 0.0,
                0.0, f, 0.0, 0.0,
                0.0, 0.0, -(far + near)/(far - near), -(2.0 * far * near)/(far-near),
                0.0, 0.0, -1.0, 0.0
            ]
        }
    }

    pub fn look_at(pos: Vector3, target: Vector3, up: Vector3) -> Matrix4 {
        let f = (pos - target).normalize();

        let mut r = f.cross(&up).normalize();
        r.negate();

        //NOTE: Not need to normialize as both f and r are normalized
        let u = f.cross(&r);

        Matrix4 {
            entries: [
                r.x, r.y, r.z, 0.0,
                u.x, u.y, u.z, 0.0,
                f.x, f.y, f.z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    //Methods

    /// Translates the current matrix
    pub fn translate(self, x: f32, y: f32, z: f32) -> Matrix4 {
        let mut mat = Matrix4::new();
        
        mat[3] = x;
        mat[7] = y;
        mat[11] = z;

        mat * self
    }

    pub fn translate_by_vector(self, vec: Vector3) -> Matrix4 {
        let mut mat = Matrix4::new();
        
        mat[3] = vec.x;
        mat[7] = vec.y;
        mat[11] = vec.z;

        mat * self
    }

    /// Scale the matrix along the Cardinal Axis
    pub fn scale(self, x: f32, y: f32, z: f32) -> Matrix4 {
        let mut mat = Matrix4::new();

        mat[0] = x;
        mat[5] = y;
        mat[10] = z;

        mat * self
    }

    pub fn scale_with_vector(self, scale: Vector3) -> Matrix4 {
        let mut mat = Matrix4::new();

        mat[0] = scale.x;
        mat[5] = scale.x;
        mat[10] = scale.x;

        mat * self
    }

    pub fn scale_axis(self, axis: Vector3, scale: f32) -> Matrix4 {
        let mut mat = Matrix4::new();
        let axis = axis.normalize();

        mat[0] = 1.0 + (scale - 1.0) * axis.x.powi(2);
        mat[1] = (scale - 1.0) * axis.x * axis.y;
        mat[2] = (scale - 1.0) * axis.x * axis.z;

        mat[4] = (scale - 1.0) * axis.z * axis.y;
        mat[5] = 1.0 + (scale - 1.0) * axis.y.powi(2);
        mat[6] = (scale - 1.0) * axis.y * axis.z;

        mat[8] = (scale - 1.0) * axis.x * axis.z;
        mat[9] = (scale - 1.0) * axis.y * axis.z;
        mat[10] = 1.0 + (scale - 1.0) * axis.z.powi(2);

        mat * self
    }

    pub fn rotate_x(self, theta: f32) -> Matrix4 {
        let mut mat = Matrix4::new();
        let theta = theta.to_radians();

        mat[5] = theta.cos();
        mat[6] = theta.sin();

        mat[9] = -theta.sin();
        mat[10] = theta.cos();

        mat * self
    }

    pub fn rotate_y(self, theta: f32) -> Matrix4 {
        let mut mat = Matrix4::new();
        let theta = theta.to_radians();

        mat[0] = theta.cos();
        mat[2] = -theta.sin();

        mat[8] = theta.sin();
        mat[10] = theta.cos();

        mat * self
    }

    pub fn rotate_z(self, theta: f32) -> Matrix4 {
        let mut mat = Matrix4::new();
        let theta = theta.to_radians();

        mat[0] = theta.cos();
        mat[1] = theta.sin();

        mat[4] = -theta.sin();
        mat[5] = theta.cos();

        mat * self
    }

    pub fn rotate(self, axis: Vector3, theta: f32) -> Matrix4 {
        let mut mat = Matrix4::new();

        let theta = theta.to_radians();
        let axis = axis.normalize();

        mat[0] = axis.x.powi(2) * (1.0 - theta.cos()) + theta.cos();
        mat[1] = axis.x * axis.y * (1.0 - theta.cos()) + axis.z * theta.sin();
        mat[2] = axis.x * axis.z * (1.0 - theta.cos()) - axis.y * theta.sin();

        mat[4] = axis.x * axis.y * (1.0 - theta.cos()) - axis.z * theta.sin();
        mat[5] = axis.y.powi(2) * (1.0 - theta.cos()) + theta.cos();
        mat[6] = axis.y * axis.z * (1.0 - theta.cos()) + axis.x * theta.sin();

        mat[8] = axis.x * axis.z * (1.0 - theta.cos()) + axis.y * theta.sin();
        mat[9] = axis.y * axis.z * (1.0 - theta.cos()) - axis.x * theta.sin();
        mat[10] = axis.z.powi(2) * (1.0 - theta.cos()) + theta.cos();

        mat * self
    }

    //TODO: add tests
    pub fn orthographic_projection(self, axis: Vector3) -> Matrix4 {
        let mut mat = Matrix4::new();

        mat[0] = 1.0 - axis.x.powi(2);
        mat[0] = -axis.x * axis.y;
        mat[0] = -axis.x * axis.z;

        mat[0] = -axis.x * axis.y;
        mat[0] = 1.0 - axis.y.powi(2);
        mat[0] = - axis.y * axis.z;

        mat[0] = -axis.x * axis.z;
        mat[0] = -axis.y * axis.z;
        mat[0] = 1.0 - axis.z.powi(2);

        mat * self
    }

    pub fn orth_proj_xy(self) -> Matrix4 {
        let mut mat = Matrix4::new();

        mat[10] = 0.0;

        mat * self
    }

    pub fn orth_proj_xz(self) -> Matrix4 {
        let mut mat = Matrix4::new();

        mat[5] = 0.0;

        mat * self
    }

    pub fn orth_proj_yz(self) -> Matrix4 {
        let mut mat = Matrix4::new();

        mat[0] = 0.0;

        mat * self
    }

    pub fn perspective_projection(self, dist: f32) -> Matrix4 {
        let mut mat = Matrix4::new();

        mat[14] = 1.0/dist;

        mat * self
    }

    pub fn reflection(self, axis: Vector4) -> Matrix4 {
        let mut mat = Matrix4::new();

        let axis = axis.normalize();

        mat[0] = 1.0 - 2.0 * axis.x.powi(2);
        mat[1] = -2.0 * axis.x * axis.y;
        mat[2] = -2.0 * axis.x * axis.z;

        mat[4] = -2.0 * axis.x * axis.y;
        mat[5] = 1.0 - 2.0 * axis.y.powi(2);
        mat[6] = -2.0 * axis.y * axis.z;

        mat[8] = -2.0 * axis.x * axis.z;
        mat[9] = -2.0 * axis.y * axis.z;
        mat[10] = 1.0 - 2.0 * axis.z.powi(2);

        mat * self
    }

    pub fn shear_xy(self, s: f32, t: f32) -> Matrix4 {
        let mut mat = Matrix4::zero();

        mat[8] = s;
        mat[9] = t;

        mat * self
    }

    pub fn shear_xz(self, s: f32, t: f32) -> Matrix4 {
        let mut mat = Matrix4::zero();

        mat[4] = s;
        mat[6] = t;

        mat * self
    }

    pub fn shear_yz(self, s: f32, t: f32) -> Matrix4 {
        let mut mat = Matrix4::zero();

        mat[1] = s;
        mat[2] = t;

        mat * self
    }

    pub fn invert(&self) -> Matrix4 {
        let mut result = Matrix4::zero();

        for i in 0..16 {
            result[i] = self[15 - i];
        }

        result
    }

    pub fn transpose(&self) -> Matrix4{
        let mut result = Matrix4::zero();

        for i in 0..4 {
            for j in 0..4 {
                result[i*4 + j] = self[j*4 +i];
            }
        }

        result
    }

    //Gets
    pub fn get_diagonal_vector(&self) -> Vector4 {
        Vector4::new(self[0], self[5], self[10], self[15])
    }

    pub fn get(&self, idx: usize) -> Option<f32> {
        if idx > 15 {
            None
        } else {
            Some(self[idx])
        }
    }

    // pub fn determinant(self) -> f32 {

    // }
}

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut mat = Matrix4::zero();

        for i in 0..16 {
            mat[i] = self[i]*rhs;
        }

        mat
    }
}

impl MulAssign<f32> for Matrix4 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..16 {
            self[i] = self[i]*rhs;
        }
    }
}

impl Mul<Matrix4> for f32 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        rhs * self
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut mat = Matrix4::zero();

        for i in 0..16 {
            let row = (i / 4) * 4;
            let col = i % 4;

            mat[i] = (self[row] * rhs[col]) + (self[row + 1] * rhs[col + 4]) + (self[row + 2] * rhs[col + 8]) + (self[row + 3] * rhs[col + 12]);
        }

        mat
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self[0]*rhs.x + self[1]*rhs.y + self[2]*rhs.z + self[3]*rhs.w,
            self[4]*rhs.x + self[5]*rhs.y + self[6]*rhs.z + self[7]*rhs.w,
            self[8]*rhs.x + self[9]*rhs.y + self[10]*rhs.z + self[11]*rhs.w,
            self[12]*rhs.x + self[13]*rhs.y + self[14]*rhs.z + self[15]*rhs.w,
        )
    }
}

impl Index<usize> for Matrix4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 15 {
            error!("Trying to reach out of bounds index");
            return &std::f32::NAN;
        }
        
        &self.entries[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4::zero()
    }
}

impl Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix4: \n {} {} {} {} \n {} {} {} {} \n {} {} {} {} \n {} {} {} {}", 
        self[0], self[1], self[2], self[3],
        self[4], self[5], self[6], self[7],
        self[8], self[9], self[10], self[11], 
        self[12], self[13], self[14], self[15])
    }
}

// endregion

// region Tests

#[cfg(test)]
mod tests {
    mod matrix2 {
        use crate::math::matrix::Matrix2;

        #[test]
        fn new() {
            let mat = Matrix2::new();

            for i in 0..4 {
                assert_eq!(mat[i], 0.0);
            }
        }

        #[test]
        fn identity() {
            let mat = Matrix2::identity();

            for i in 0..2 {
                for j in 0..2 {
                    if i == j {
                        assert_eq!(mat[i + 2 * j], 1.0);
                    } else {
                        assert_eq!(mat[i + 2 * j], 0.0);
                    }
                }
            }
        }

        //Methods
        #[test]
        fn transpose() {
            let mut orig = Matrix2::new();

            for i in 0..4 {
                orig[i] = 1.0 + i as f32;
            }

            let mat = orig.transpose();
            
            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[1], 3.0);
            assert_eq!(mat[2], 2.0);
            assert_eq!(mat[3], 4.0);

            assert_eq!(orig[0], 1.0);
            assert_eq!(orig[1], 2.0);
            assert_eq!(orig[2], 3.0);
            assert_eq!(orig[3], 4.0);

            


        }

        #[test]
        fn invert() {
            let mut mat = Matrix2::new();

            for i in 0..4 {
                mat[i] = (i + 1) as f32;
            }

            let inv_mat = mat.invert();

            for i in 0..4 {
                assert_eq!(inv_mat[i], 4.0 - (i as f32));
            }

            for i in 0..4 {
                assert_eq!(mat[i], 1.0 + i as f32);
            }
        }


        //Operattors
        #[test]
        fn mul_scalar() {
            let mut mat = Matrix2::new();

            for i in 0..4 {
                mat[i] = 1.0 + i as f32;
            }

            let mul_mat = mat * 2.0;

            for i in 0..4 {
                assert_eq!(mul_mat[i], ((i + 1) * 2) as f32);
            }

            for i in 0..4 {
                assert_eq!(mat[i], 1.0 + i as f32);
            }
        }

        #[test]
        fn mul_by_scalar() {
            let mut mat = Matrix2::new();

            for i in 0..4 {
                mat[i] = 1.0 + i as f32;
            }

            let mul_mat = 2.0 * mat;

            for i in 0..4 {
                assert_eq!(mul_mat[i], ((i + 1) * 2) as f32);
            }

            for i in 0..4 {
                assert_eq!(mat[i], 1.0 + i as f32);
            }
        }

        #[test]
        fn mul_assign_scalar() {
            let mut mat = Matrix2::new();

            for i in 0..4 {
                mat[i] = 1.0 + i as f32;
            }

            mat *= 2.0;

            for i in 0..4 {
                assert_eq!(mat[i], ((i + 1) * 2) as f32);
            }
        }

        #[test]
        fn mul() {
            let mut mat = Matrix2::new();

            for i in 0..4 {
                mat[i] = 1.0 + i as f32;
            }

            let tran_mat = mat.transpose();

            let mul = mat * tran_mat;

            assert_eq!(mul[0], 5.0);
            assert_eq!(mul[1], 11.0);
            assert_eq!(mul[2], 11.0);
            assert_eq!(mul[3], 25.0);
        }

        #[test]
        fn access_high_index() {
            let mat = Matrix2::new();

            let i = mat[4];

            assert!(f32::is_nan(i));
        }
    }

    mod matrix3 {
        use crate::math::matrix::Matrix3;
        use crate::math::vector::Vector2;

        #[test]
        fn new() {
            let mat = Matrix3::new();

            for i in 0..9 {
                assert_eq!(mat[i], 0.0);
            }
        }

        #[test]
        fn identity() {
            let mat = Matrix3::identity();

            for i in 0..3 {
                for j in 0..3 {
                    if i == j {
                        assert_eq!(mat[i +3 * j], 1.0);
                    } else {
                        assert_eq!(mat[i +3 * j], 0.0);
                    }
                }
            }
        }

        #[test]
        fn from_3d_vectors() {
            let vec1 = &Vector2::new(1.0, 2.0,);
            let vec2 = &Vector2::new(3.0, 4.0);

            let mat = Matrix3::from_2d_vectors(vec1, vec2);

            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[1], 2.0);
            assert_eq!(mat[2], 0.0);

            assert_eq!(mat[3], 3.0);
            assert_eq!(mat[4], 4.0);
            assert_eq!(mat[5], 0.0);

            assert_eq!(mat[6], 0.0);
            assert_eq!(mat[7], 0.0);
            assert_eq!(mat[8], 1.0);
        }

        //Methods
        #[test]
        fn transpose() {
            let vec1 = &Vector2::new(1.0, 2.0);
            let vec2 = &Vector2::new(3.0, 4.0);

            let mat = Matrix3::from_2d_vectors(vec1, vec2).transpose();
            
            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[1], 3.0);
            assert_eq!(mat[2], 0.0);

            assert_eq!(mat[3], 2.0);
            assert_eq!(mat[4], 4.0);
            assert_eq!(mat[5], 0.0);

            assert_eq!(mat[6], 0.0);
            assert_eq!(mat[7], 0.0);
            assert_eq!(mat[8], 1.0);
        }

        #[test]
        fn invert() {
            let mut mat = Matrix3::new();

            for i in 0..9 {
                mat[i] = (i + 1) as f32;
            }

            let inv_mat = mat.invert();

            for i in 0..9 {
                assert_eq!(inv_mat[i], 9.0 - (i as f32));
            }

            for i in 0..9 {
                assert_eq!(mat[i], (i + 1) as f32);
            }
        }


        //Operattors
        #[test]
        fn mul_scalar() {
            let mut mat = Matrix3::new();

            for i in 0..9 {
                mat[i] = (i + 1) as f32;
            }

            let mul_mat = mat * 2.0;

            for i in 0..9 {
                assert_eq!(mul_mat[i], ((i + 1) * 2) as f32);
            }

            for i in 0..9 {
                assert_eq!(mat[i], (i + 1) as f32);
            }
        }

        #[test]
        fn mul_by_scalar() {
            let mut mat = Matrix3::new();

            for i in 0..9 {
                mat[i] = (i + 1) as f32;
            }

            let mul_mat = 2.0 * mat;

            for i in 0..9 {
                assert_eq!(mul_mat[i], ((i + 1) * 2) as f32);
            }

            for i in 0..9 {
                assert_eq!(mat[i], (i + 1) as f32);
            }
        }

        #[test]
        fn mul_assign_scalar() {
            let mut mat = Matrix3::new();

            for i in 0..9 {
                mat[i] = (i + 1) as f32;
            }

            mat *= 2.0;

            for i in 0..9 {
                assert_eq!(mat[i], ((i + 1) * 2) as f32);
            }
        }

        #[test]
        fn mul() {
            let mut mat = Matrix3::new();

            for i in 0..9 {
                mat[i] = (i + 1) as f32;
            }

            let tran_mat = mat.transpose();

            let mul = mat * tran_mat;

            assert_eq!(mul[0], 14.0);
            assert_eq!(mul[1], 32.0);
            assert_eq!(mul[2], 50.0);

            assert_eq!(mul[3], 32.0);
            assert_eq!(mul[4], 77.0);
            assert_eq!(mul[5], 122.0);

            assert_eq!(mul[6], 50.0);
            assert_eq!(mul[7], 122.0);
            assert_eq!(mul[8], 194.0);
        }

        #[test]
        fn access_high_index() {
            let mat = Matrix3::new();

            let i = mat[9];

            assert!(f32::is_nan(i));
        }
    }

    mod matrix4 {
        use crate::math::matrix::Matrix4;
        use crate::math::vector::{
            Vector3,
            Vector4
        };

        #[test]
        fn new() {
            let mat = Matrix4::zero();

            for i in 0..16 {
                assert_eq!(mat[i], 0.0);
            }
        }

        #[test]
        fn identity() {
            let mat = Matrix4::new();

            for i in 0..4 {
                for j in 0..4 {
                    if i == j {
                        assert_eq!(mat[i +4*j], 1.0);
                    } else {
                        assert_eq!(mat[i +4*j], 0.0);
                    }
                }
            }
        }

        #[test]
        fn from_3d_vectors() {
            let vec1 = &Vector3::new(1.0, 2.0, 3.0);
            let vec2 = &Vector3::new(4.0, 5.0, 6.0);
            let vec3 = &Vector3::new(7.0, 8.0, 9.0);

            let mat = Matrix4::from_3d_vectors(vec1, vec2, vec3);

            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[1], 2.0);
            assert_eq!(mat[2], 3.0);
            assert_eq!(mat[3], 0.0);

            assert_eq!(mat[4], 4.0);
            assert_eq!(mat[5], 5.0);
            assert_eq!(mat[6], 6.0);
            assert_eq!(mat[7], 0.0);

            assert_eq!(mat[8], 7.0);
            assert_eq!(mat[9], 8.0);
            assert_eq!(mat[10], 9.0);
            assert_eq!(mat[11], 0.0);

            assert_eq!(mat[12], 0.0);
            assert_eq!(mat[13], 0.0);
            assert_eq!(mat[14], 0.0);
            assert_eq!(mat[15], 1.0);
        }

        //Methods
        #[test]
        fn transpose() {
            let vec1 = &Vector3::new(1.0, 2.0, 3.0);
            let vec2 = &Vector3::new(4.0, 5.0, 6.0);
            let vec3 = &Vector3::new(7.0, 8.0, 9.0);

            let mat = Matrix4::from_3d_vectors(vec1, vec2, vec3).transpose();
            
            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[1], 4.0);
            assert_eq!(mat[2], 7.0);
            assert_eq!(mat[3], 0.0);

            assert_eq!(mat[4], 2.0);
            assert_eq!(mat[5], 5.0);
            assert_eq!(mat[6], 8.0);
            assert_eq!(mat[7], 0.0);

            assert_eq!(mat[8], 3.0);
            assert_eq!(mat[9], 6.0);
            assert_eq!(mat[10], 9.0);
            assert_eq!(mat[11], 0.0);

            assert_eq!(mat[12], 0.0);
            assert_eq!(mat[13], 0.0);
            assert_eq!(mat[14], 0.0);
            assert_eq!(mat[15], 1.0);
        }

        #[test]
        fn invert() {
            let mut mat = Matrix4::new();

            for i in 0..16 {
                mat[i] = (i + 1) as f32;
            }

            let inv_mat = mat.invert();

            for i in 0..16 {
                assert_eq!(inv_mat[i], 16.0 - (i as f32));
            }

            for i in 0..16 {
                assert_eq!(mat[i], (i + 1) as f32);
            }
        }

        //Translation
        #[test]
        fn translate() {
            let mut mat = Matrix4::identity();
            mat = mat.translate(3.0, 4.0, 5.0);

            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[5], 1.0);
            assert_eq!(mat[10], 1.0);
            assert_eq!(mat[15], 1.0);
            
            assert_eq!(mat[3], 3.0);
            assert_eq!(mat[7], 4.0);
            assert_eq!(mat[11], 5.0);
        }

        #[test]
        fn translate_by_vector() {
            let mut mat = Matrix4::identity();
            mat = mat.translate_by_vector(Vector3::new(3.0, 3.0, 3.0));

            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[5], 1.0);
            assert_eq!(mat[10], 1.0);
            assert_eq!(mat[15], 1.0);
            
            assert_eq!(mat[3], 3.0);
            assert_eq!(mat[7], 3.0);
            assert_eq!(mat[11], 3.0);
        }

        //Scale
        #[test]
        fn scale() {
            let mut mat = Matrix4::identity();

            mat = mat.scale(2.0, 3.0, 5.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let scaled_vec = mat * vec;

            assert_eq!(mat[0], 2.0);
            assert_eq!(mat[5], 3.0);
            assert_eq!(mat[10], 5.0);
            assert_eq!(mat[15], 1.0);

            assert_eq!(vec.x, 5.0);
            assert_eq!(vec.y, 6.0);
            assert_eq!(vec.z, 3.0);
            assert_eq!(vec.w, 1.0);

            assert_eq!(scaled_vec.x, 10.0);
            assert_eq!(scaled_vec.y, 18.0);
            assert_eq!(scaled_vec.z, 15.0);
            assert_eq!(scaled_vec.w, 1.0);
        }

        #[test]
        fn scale_arbitraty_x() {
            let mut mat = Matrix4::identity();

            mat = mat.scale_axis(Vector3::X, 2.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let scaled_vec = mat * vec;

            assert_eq!(mat[0], 2.0);
            assert_eq!(mat[5], 1.0);
            assert_eq!(mat[10], 1.0);
            assert_eq!(mat[15], 1.0);

            assert_eq!(vec.x, 5.0);
            assert_eq!(vec.y, 6.0);
            assert_eq!(vec.z, 3.0);
            assert_eq!(vec.w, 1.0);

            assert_eq!(scaled_vec.x, 10.0);
            assert_eq!(scaled_vec.y, 6.0);
            assert_eq!(scaled_vec.z, 3.0);
            assert_eq!(scaled_vec.w, 1.0);
        }

        #[test]
        fn scale_arbitraty_y() {
            let mut mat = Matrix4::identity();

            mat = mat.scale_axis(Vector3::Y, 2.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let scaled_vec = mat * vec;

            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[5], 2.0);
            assert_eq!(mat[10], 1.0);
            assert_eq!(mat[15], 1.0);

            assert_eq!(vec.x, 5.0);
            assert_eq!(vec.y, 6.0);
            assert_eq!(vec.z, 3.0);
            assert_eq!(vec.w, 1.0);

            assert_eq!(scaled_vec.x, 5.0);
            assert_eq!(scaled_vec.y, 12.0);
            assert_eq!(scaled_vec.z, 3.0);
            assert_eq!(scaled_vec.w, 1.0);
        }

        #[test]
        fn scale_arbitraty_z() {
            let mut mat = Matrix4::identity();

            mat = mat.scale_axis(Vector3::Z, 2.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let scaled_vec = mat * vec;

            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[5], 1.0);
            assert_eq!(mat[10], 2.0);
            assert_eq!(mat[15], 1.0);

            assert_eq!(vec.x, 5.0);
            assert_eq!(vec.y, 6.0);
            assert_eq!(vec.z, 3.0);
            assert_eq!(vec.w, 1.0);

            assert_eq!(scaled_vec.x, 5.0);
            assert_eq!(scaled_vec.y, 6.0);
            assert_eq!(scaled_vec.z, 6.0);
            assert_eq!(scaled_vec.w, 1.0);
        }

        //Rotation
        #[test]
        fn rotate_x() {
            let mut mat = Matrix4::identity();

            mat = mat.rotate_x(90.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let rotated_vec = mat * vec;

            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[5].round(), 0.0);
            assert_eq!(mat[6], 1.0);
            assert_eq!(mat[9], -1.0);
            assert_eq!(mat[10].round(), 0.0);
            assert_eq!(mat[15], 1.0);

            assert_eq!(rotated_vec.x, 5.0);
            assert_eq!(rotated_vec.y.round(), 3.0);
            assert_eq!(rotated_vec.z.round(), -6.0);
            assert_eq!(rotated_vec.w, 1.0);
        }

        #[test]
        fn rotate_y() {
            let mut mat = Matrix4::identity();

            mat = mat.rotate_y(90.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let rotated_vec = mat * vec;

            assert_eq!(mat[0].round(), 0.0);
            assert_eq!(mat[2], -1.0);
            assert_eq!(mat[5], 1.0);
            assert_eq!(mat[8], 1.0);
            assert_eq!(mat[10].round(), 0.0);
            assert_eq!(mat[15], 1.0);

            assert_eq!(rotated_vec.x.round(), -3.0);
            assert_eq!(rotated_vec.y, 6.0);
            assert_eq!(rotated_vec.z, 5.0);
            assert_eq!(rotated_vec.w, 1.0);
        }

        #[test]
        fn rotate_z() {
            let mut mat = Matrix4::identity();

            mat = mat.rotate_z(90.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let rotated_vec = mat * vec;

            assert_eq!(mat[0].round(), 0.0);
            assert_eq!(mat[1], 1.0);
            assert_eq!(mat[4], -1.0);
            assert_eq!(mat[5].round(), 0.0);
            assert_eq!(mat[10], 1.0);
            assert_eq!(mat[15], 1.0);

            assert_eq!(rotated_vec.x, 6.0);
            assert_eq!(rotated_vec.y.round(), -5.0);
            assert_eq!(rotated_vec.z, 3.0);
            assert_eq!(rotated_vec.w, 1.0);
        }

        #[test]
        fn rotate_axis_x() {
            let mut mat = Matrix4::identity();

            mat = mat.rotate(Vector3::X, 90.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let rotated_vec = mat * vec;

            assert_eq!(mat[0].round(), 1.0);
            assert_eq!(mat[5].round(), 0.0);
            assert_eq!(mat[6].round(), 1.0);
            assert_eq!(mat[9].round(), -1.0);
            assert_eq!(mat[10].round(), 0.0);
            assert_eq!(mat[15].round(), 1.0);

            assert_eq!(rotated_vec.x.round(), 5.0);
            assert_eq!(rotated_vec.y.round(), 3.0);
            assert_eq!(rotated_vec.z.round(), -6.0);
            assert_eq!(rotated_vec.w.round(), 1.0);
        }

        #[test]
        fn rotate_axis_y() {
            let mut mat = Matrix4::identity();

            mat = mat.rotate(Vector3::Y, 90.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let rotated_vec = mat * vec;

            assert_eq!(mat[0].round(), 0.0);
            assert_eq!(mat[2].round(), -1.0);
            assert_eq!(mat[5].round(), 1.0);
            assert_eq!(mat[8].round(), 1.0);
            assert_eq!(mat[10].round(), 0.0);
            assert_eq!(mat[15].round(), 1.0);

            assert_eq!(rotated_vec.x.round(), -3.0);
            assert_eq!(rotated_vec.y.round(), 6.0);
            assert_eq!(rotated_vec.z.round(), 5.0);
            assert_eq!(rotated_vec.w.round(), 1.0);
        }

        #[test]
        fn rotate_axis_z() {
            let mut mat = Matrix4::identity();

            mat = mat.rotate(Vector3::Z, 90.0);

            let vec = Vector4::new(5.0, 6.0, 3.0, 1.0);

            let rotated_vec = mat * vec;

            assert_eq!(mat[0].round(), 0.0);
            assert_eq!(mat[1].round(), 1.0);
            assert_eq!(mat[4].round(), -1.0);
            assert_eq!(mat[5].round(), 0.0);
            assert_eq!(mat[10].round(), 1.0);
            assert_eq!(mat[15].round(), 1.0);

            assert_eq!(rotated_vec.x.round(), 6.0);
            assert_eq!(rotated_vec.y.round(), -5.0);
            assert_eq!(rotated_vec.z.round(), 3.0);
            assert_eq!(rotated_vec.w.round(), 1.0);
        }

        #[test]
        fn rotate_z_increment() {
            
        }

        #[test]
        fn transformed_vector() {
            let mut mat = Matrix4::identity();
            mat = mat.scale(2.0, 1.0, 1.0).rotate_y(90.0).translate(5.0, 3.0, 4.0);

            let vec = Vector4::new(4.0, 4.0, 4.0, 1.0);

            let tras_vec = mat * vec;

            assert_eq!(mat[0].round(), 0.0);
            assert_eq!(mat[2], -1.0);
            assert_eq!(mat[3].round(), 5.0);

            assert_eq!(mat[5], 1.0);
            assert_eq!(mat[7], 3.0);

            assert_eq!(mat[8], 2.0);
            assert_eq!(mat[10].round(), 0.0);
            assert_eq!(mat[11], 4.0);

            assert_eq!(mat[15], 1.0);

            assert_eq!(tras_vec.x.round(), 1.0);
            assert_eq!(tras_vec.y, 7.0);
            assert_eq!(tras_vec.z, 12.0);
            assert_eq!(tras_vec.w, 1.0);

        }

        #[test]
        fn simple_trans_vector() {
            let mat = Matrix4::identity()
                .scale(2.0, 2.0, 2.0)
                .rotate_x(90.0);

            let vec = Vector4::new(4.0, 6.0, 3.0, 1.0);

            let trans_vec = mat * vec;

            assert_eq!(mat[0], 2.0);

            assert_eq!(mat[5].round(), 0.0);
            assert_eq!(mat[6], 2.0);

            assert_eq!(mat[9], -2.0);
            assert_eq!(mat[10].round(), 0.0);

            assert_eq!(mat[15], 1.0);

            assert_eq!(trans_vec.x, 8.0);
            assert_eq!(trans_vec.y.round(), 6.0);
            assert_eq!(trans_vec.z, -12.0);
            assert_eq!(trans_vec.w, 1.0);
        }
        
        //Operators
        #[test]
        fn mul_scalar() {
            let mut mat = Matrix4::new();

            for i in 0..16 {
                mat[i] = (i + 1) as f32;
            }

            let mul_mat = mat * 2.0;

            for i in 0..16 {
                assert_eq!(mul_mat[i], ((i + 1) * 2) as f32);
            }

            for i in 0..16 {
                assert_eq!(mat[i], (i + 1) as f32);
            }
        }

        #[test]
        fn mul_by_scalar() {
            let mut mat = Matrix4::new();

            for i in 0..16 {
                mat[i] = (i + 1) as f32;
            }

            let mul_mat = 2.0 * mat;

            for i in 0..16 {
                assert_eq!(mul_mat[i], ((i + 1) * 2) as f32);
            }

            for i in 0..16 {
                assert_eq!(mat[i], (i + 1) as f32);
            }
        }

        #[test]
        fn mul_assign_scalar() {
            let mut mat = Matrix4::new();

            for i in 0..16 {
                mat[i] = (i + 1) as f32;
            }

            mat *= 2.0;

            for i in 0..16 {
                assert_eq!(mat[i], ((i + 1) * 2) as f32);
            }
        }

        #[test]
        fn mul() {
            let mut mat = Matrix4::new();

            for i in 0..16 {
                mat[i] = (i + 1) as f32;
            }

            let tran_mat = mat.transpose();

            let mul = mat * tran_mat;

            assert_eq!(mul[0], 30.0);
            assert_eq!(mul[1], 70.0);
            assert_eq!(mul[2], 110.0);
            assert_eq!(mul[3], 150.0);

            assert_eq!(mul[4], 70.0);
            assert_eq!(mul[5], 174.0);
            assert_eq!(mul[6], 278.0);
            assert_eq!(mul[7], 382.0);

            assert_eq!(mul[8], 110.0);
            assert_eq!(mul[9], 278.0);
            assert_eq!(mul[10], 446.0);
            assert_eq!(mul[11], 614.0);

            assert_eq!(mul[12], 150.0);
            assert_eq!(mul[13], 382.0);
            assert_eq!(mul[14], 614.0);
            assert_eq!(mul[15], 846.0);
        }

        #[test]
        fn mul_with_vector() {
            let vec = Vector4::default();

            let mut mat = Matrix4::identity();
            mat = mat.translate(5.0, 7.0, 10.0);

            let trans_vec = mat * vec;

            assert_eq!(vec.x, 1.0);
            assert_eq!(vec.y, 1.0);
            assert_eq!(vec.z, 1.0);
            assert_eq!(vec.w, 1.0);

            assert_eq!(mat[0], 1.0);
            assert_eq!(mat[5], 1.0);
            assert_eq!(mat[10], 1.0);
            assert_eq!(mat[15], 1.0);

            assert_eq!(mat[3], 5.0);
            assert_eq!(mat[7], 7.0);
            assert_eq!(mat[11], 10.0);

            assert_eq!(trans_vec.x, 6.0);
            assert_eq!(trans_vec.y, 8.0);
            assert_eq!(trans_vec.z, 11.0);
            assert_eq!(trans_vec.w, 1.0);
        }

        #[test]
        fn access_high_index() {
            let mat = Matrix4::new();

            let i = mat[16];

            assert!(f32::is_nan(i));
        }
    }
}

// endregion