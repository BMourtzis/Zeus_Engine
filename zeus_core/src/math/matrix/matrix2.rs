use crate::math::Vector2;

use std::ops::{Index, IndexMut, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Matrix2 {
    entries: [f32; 4],
}

impl Matrix2 {
    pub fn new() -> Matrix2 {
        Matrix2 { entries: [0.0; 4] }
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
        Vector2::new(self[0], self[4])
    }
}

impl Mul<f32> for Matrix2 {
    type Output = Matrix2;

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        let mut mat = Matrix2::new();

        for i in 0..4 {
            mat[i] = self[i] * rhs;
        }

        mat
    }
}

impl MulAssign<f32> for Matrix2 {
    fn mul_assign(
        &mut self,
        rhs: f32,
    ) {
        for i in 0..4 {
            self[i] = self[i] * rhs;
        }
    }
}

impl Mul<Matrix2> for f32 {
    type Output = Matrix2;

    fn mul(
        self,
        rhs: Matrix2,
    ) -> Self::Output {
        rhs * self
    }
}

impl Mul for Matrix2 {
    type Output = Matrix2;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(
        self,
        rhs: Matrix2,
    ) -> Self::Output {
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

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        if index > 3 {
            error!("Trying to reach out of bounds index");
            return &std::f32::NAN;
        }

        &self.entries[index]
    }
}

impl IndexMut<usize> for Matrix2 {
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Default for Matrix2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Matrix2;

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
