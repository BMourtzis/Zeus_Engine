use crate::math::{Vector2, Vector3};

use std::ops::{Index, IndexMut, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    entries: [f32; 9],
}

impl Matrix3 {
    pub fn new() -> Matrix3 {
        Matrix3 { entries: [0.0; 9] }
    }

    pub fn identity() -> Matrix3 {
        let mut result = Matrix3::new();

        result[0] = 1.0;
        result[4] = 1.0;
        result[8] = 1.0;

        result
    }

    pub fn from_2d_vectors(
        a: Vector2,
        b: Vector2,
    ) -> Matrix3 {
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

    pub fn transpose(&self) -> Matrix3 {
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

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        let mut mat = Matrix3::new();

        for i in 0..9 {
            mat[i] = self[i] * rhs;
        }

        mat
    }
}

impl MulAssign<f32> for Matrix3 {
    fn mul_assign(
        &mut self,
        rhs: f32,
    ) {
        for i in 0..9 {
            self[i] = self[i] * rhs;
        }
    }
}

impl Mul<Matrix3> for f32 {
    type Output = Matrix3;

    fn mul(
        self,
        rhs: Matrix3,
    ) -> Self::Output {
        rhs * self
    }
}

impl Mul for Matrix3 {
    type Output = Matrix3;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(
        self,
        rhs: Matrix3,
    ) -> Self::Output {
        let mut mat = Matrix3::new();

        for i in 0..9 {
            let row = (i / 3) * 3;
            let col = i % 3;

            mat[i] = (self[row] * rhs[col])
                + (self[row + 1] * rhs[col + 3])
                + (self[row + 2] * rhs[col + 6]);
        }

        mat
    }
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(
        self,
        rhs: Vector3,
    ) -> Self::Output {
        Vector3::new(
            self[0] * rhs.x + self[1] * rhs.y + self[2] * rhs.z,
            self[4] * rhs.x + self[5] * rhs.y + self[6] * rhs.z,
            self[8] * rhs.x + self[9] * rhs.y + self[10] * rhs.z,
        )
    }
}

impl Index<usize> for Matrix3 {
    type Output = f32;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        if index > 8 {
            error!("Trying to reach out of bounds index");
            return &std::f32::NAN;
        }

        &self.entries[index]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Matrix3::new()
    }
}


#[cfg(test)]
mod tests {
    use crate::math::{Vector2, Matrix3};

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
                    assert_eq!(mat[i + 3 * j], 1.0);
                } else {
                    assert_eq!(mat[i + 3 * j], 0.0);
                }
            }
        }
    }

    #[test]
    fn from_3d_vectors() {
        let vec1 = Vector2::new(1.0, 2.0);
        let vec2 = Vector2::new(3.0, 4.0);

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
        let vec1 = Vector2::new(1.0, 2.0);
        let vec2 = Vector2::new(3.0, 4.0);

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
