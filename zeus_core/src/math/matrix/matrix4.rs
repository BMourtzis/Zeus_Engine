use crate::math::{
    Matrix3,
    Vector3,
    Vector4
};

use std::{
    fmt::{
        self, Display
    },
    ops::{
        Index, IndexMut, Mul, MulAssign
    }
};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    entries: [f32; 16],
}

impl Matrix4 {
    //Constructors
    pub fn zero() -> Self {
        Matrix4 { entries: [0.0; 16] }
    }

    pub fn new() -> Self {
        let mut res = Matrix4::zero();

        res[0] = 1.0;
        res[5] = 1.0;
        res[10] = 1.0;
        res[15] = 1.0;

        res
    }

    pub fn new_traslation(x: f32, y: f32, z: f32) -> Self {
        let mut res = Matrix4::new();

        res[3] = x;
        res[7] = y;
        res[11] = z;

        res
    }

    pub fn new_rotation_x(theta: f32) -> Self {
        let mut res = Matrix4::new();
        let theta = theta.to_radians();

        res[5] = theta.cos();
        res[6] = theta.sin();

        res[9] = -theta.sin();
        res[10] = theta.cos();

        res
    }

    pub fn new_rotation_y(theta: f32) -> Self {
        let mut res = Matrix4::new();
        let theta = theta.to_radians();

        res[0] = theta.cos();
        res[2] = -theta.sin();

        res[8] = theta.sin();
        res[10] = theta.cos();

        res
    }

    pub fn new_rotation_z(theta: f32) -> Self {
        let mut res = Matrix4::new();
        let theta = theta.to_radians();

        res[0] = theta.cos();
        res[1] = theta.sin();

        res[4] = -theta.sin();
        res[5] = theta.cos();

        res
    }

    pub fn new_rotation(axis: Vector3, theta: f32) -> Self {
        let mut res = Matrix4::new();

        let theta = theta.to_radians();
        let axis = axis.normalize();

        res[0] = axis.x.powi(2) * (1.0 - theta.cos()) + theta.cos();
        res[1] = axis.x * axis.y * (1.0 - theta.cos()) + axis.z * theta.sin();
        res[2] = axis.x * axis.z * (1.0 - theta.cos()) - axis.y * theta.sin();

        res[4] = axis.x * axis.y * (1.0 - theta.cos()) - axis.z * theta.sin();
        res[5] = axis.y.powi(2) * (1.0 - theta.cos()) + theta.cos();
        res[6] = axis.y * axis.z * (1.0 - theta.cos()) + axis.x * theta.sin();

        res[8] = axis.x * axis.z * (1.0 - theta.cos()) + axis.y * theta.sin();
        res[9] = axis.y * axis.z * (1.0 - theta.cos()) - axis.x * theta.sin();
        res[10] = axis.z.powi(2) * (1.0 - theta.cos()) + theta.cos();

        res
    }

    pub fn new_scale(x: f32, y: f32, z: f32) -> Self {
        let mut res = Matrix4::new();

        res[0] = x;
        res[5] = y;
        res[10] = z;

        res
    }

    pub fn new_scale_vector(scale: Vector3) -> Self {
        let mut res = Matrix4::new();

        res[0] = scale.x;
        res[5] = scale.y;
        res[10] = scale.z;

        res
    }

    pub fn new_scale_axis(axis: Vector3, scale: f32) -> Self {
        let mut res = Matrix4::new();
        let axis = axis.normalize();

        res[0] = 1.0 + (scale - 1.0) * axis.x.powi(2);
        res[1] = (scale - 1.0) * axis.x * axis.y;
        res[2] = (scale - 1.0) * axis.x * axis.z;

        res[4] = (scale - 1.0) * axis.z * axis.y;
        res[5] = 1.0 + (scale - 1.0) * axis.y.powi(2);
        res[6] = (scale - 1.0) * axis.y * axis.z;

        res[8] = (scale - 1.0) * axis.x * axis.z;
        res[9] = (scale - 1.0) * axis.y * axis.z;
        res[10] = 1.0 + (scale - 1.0) * axis.z.powi(2);

        res
    }

    pub fn from_matrix3(mat: Matrix3) -> Self {
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

    pub fn from_3d_vectors(a: &Vector3, b: &Vector3, c: &Vector3) -> Self {
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

    pub fn from_vector3(vec: &Vector3) -> Self {
        let mut mat = Matrix4::new();

        mat[3] = vec.x;
        mat[7] = vec.y;
        mat[11] = vec.z;

        mat
    }

    pub fn perspective(
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let mut aspect = aspect;
        if aspect <= 0.0 {
            aspect = 1.0;
        }

        //TODO: make sure far > near

        let f = 1.0 / (fov / 2.0).tan();

        Matrix4 {
            entries: [
                f / aspect,
                0.0,
                0.0,
                0.0,

                0.0,
                f,
                0.0,
                0.0,

                0.0,
                0.0,
                -(far + near) / (far - near),
                -(2.0 * far * near) / (far - near),

                0.0,
                0.0,
                -1.0,
                0.0,
            ],
        }
    }

    pub fn look_at(pos: Vector3, target: Vector3, up: Vector3) -> Self {
        let f = (pos - target).normalize();

        let mut r = f.cross(&up).normalize();
        r.negate();

        //NOTE: Not need to normialize as both f and r are normalized
        let u = f.cross(&r);

        Matrix4 {
            entries: [
                r.x, r.y, r.z, 0.0, u.x, u.y, u.z, 0.0, f.x, f.y, f.z, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn new_projection() -> Self {
        let mut res = Matrix4::new();
        res[0] = -1.0;
        res[5] = -1.0;

        res
    }
    
    //Translation
    /// Translates the current matrix
    pub fn translate(&mut self, x: f32, y: f32,z: f32) {
        *self = Matrix4::new_traslation(x, y, z) * *self;
    }

    pub fn translate_by_vector(&mut self, vec: Vector3) {
        *self = Matrix4::new_traslation(vec.x, vec.y, vec.z) * *self;
    }

    //Scale
    /// Scale the matrix along the Cardinal Axis
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        *self = Matrix4::new_scale(x, y, z) * *self;
    }

    pub fn scale_with_vector(&mut self, scale: Vector3)  {
        *self = Matrix4::new_scale_vector(scale) * *self
    }

    pub fn scale_axis(&mut self, axis: Vector3, scale: f32) {
        *self = Matrix4::new_scale_axis(axis, scale) * *self
    }

    //Rotation

    pub fn rotate_x(&mut self, theta: f32) {
        *self = Matrix4::new_rotation_x(theta) * *self;
    }

    pub fn rotate_y(&mut self, theta: f32) {
        *self = Matrix4::new_rotation_y(theta) * *self;
    }

    pub fn rotate_z(&mut self, theta: f32) {
        *self = Matrix4::new_rotation_z(theta) * *self;
    }

    pub fn rotate(&mut self, axis: Vector3, theta: f32) {
        *self = Matrix4::new_rotation(axis, theta) * *self;
    }

    //TODO: add tests
    pub fn orthographic_projection(self, axis: Vector3) -> Self {
        let mut mat = Matrix4::new();

        mat[0] = 1.0 - axis.x.powi(2);
        mat[0] = -axis.x * axis.y;
        mat[0] = -axis.x * axis.z;

        mat[0] = -axis.x * axis.y;
        mat[0] = 1.0 - axis.y.powi(2);
        mat[0] = -axis.y * axis.z;

        mat[0] = -axis.x * axis.z;
        mat[0] = -axis.y * axis.z;
        mat[0] = 1.0 - axis.z.powi(2);

        mat * self
    }

    pub fn orth_proj_xy(self) -> Self {
        let mut mat = Matrix4::new();

        mat[10] = 0.0;

        mat * self
    }

    pub fn orth_proj_xz(self) -> Self {
        let mut mat = Matrix4::new();

        mat[5] = 0.0;

        mat * self
    }

    pub fn orth_proj_yz(self) -> Self {
        let mut mat = Matrix4::new();

        mat[0] = 0.0;

        mat * self
    }

    pub fn perspective_projection(self, dist: f32) -> Self {
        let mut mat = Matrix4::new();

        mat[14] = 1.0 / dist;

        mat * self
    }

    pub fn reflection(self, axis: Vector4) -> Self {
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

    pub fn shear_xy(self, s: f32, t: f32) -> Self {
        let mut mat = Matrix4::zero();

        mat[8] = s;
        mat[9] = t;

        mat * self
    }

    pub fn shear_xz(self, s: f32, t: f32) -> Self {
        let mut mat = Matrix4::zero();

        mat[4] = s;
        mat[6] = t;

        mat * self
    }

    pub fn shear_yz(self, s: f32, t: f32) -> Self {
        let mut mat = Matrix4::zero();

        mat[1] = s;
        mat[2] = t;

        mat * self
    }

    pub fn invert(&self) -> Self {
        let mut result = Matrix4::zero();

        for i in 0..16 {
            result[i] = self[15 - i];
        }

        result
    }

    pub fn transpose(&self) -> Self {
        let mut result = Matrix4::zero();

        for i in 0..4 {
            for j in 0..4 {
                result[i * 4 + j] = self[j * 4 + i];
            }
        }

        result
    }

    //Gets
    pub fn get_diagonal_vector(&self) -> Vector4 {
        Vector4::new(self[0], self[5], self[10], self[15])
    }

    pub fn get(&self, idx: usize,) -> Option<f32> {
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

    fn mul(
        self,
        rhs: f32,
    ) -> Self::Output {
        let mut mat = Matrix4::zero();

        for i in 0..16 {
            mat[i] = self[i] * rhs;
        }

        mat
    }
}

impl MulAssign<f32> for Matrix4 {
    fn mul_assign(
        &mut self,
        rhs: f32,
    ) {
        for i in 0..16 {
            self[i] = self[i] * rhs;
        }
    }
}

impl Mul<Matrix4> for f32 {
    type Output = Matrix4;

    fn mul(
        self,
        rhs: Matrix4,
    ) -> Self::Output {
        rhs * self
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul( self,rhs: Matrix4 ) -> Self::Output {
        let mut mat = Matrix4::zero();

        for i in 0..16 {
            let row = (i / 4) * 4;
            let col = i % 4;

            mat[i] = (self[row] * rhs[col])
                + (self[row + 1] * rhs[col + 4])
                + (self[row + 2] * rhs[col + 8])
                + (self[row + 3] * rhs[col + 12]);
        }

        mat
    }
}

impl Mul<&mut Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: &mut Matrix4) -> Self::Output {
        let mut mat = Matrix4::zero();

        for i in 0..16 {
            let row = (i / 4) * 4;
            let col = i % 4;

            mat[i] = (self[row] * rhs[col])
                + (self[row + 1] * rhs[col + 4])
                + (self[row + 2] * rhs[col + 8])
                + (self[row + 3] * rhs[col + 12]);
        }

        mat
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(
        self,
        rhs: Vector4,
    ) -> Self::Output {
        Vector4::new(
            self[0] * rhs.x + self[1] * rhs.y + self[2] * rhs.z + self[3] * rhs.w,
            self[4] * rhs.x + self[5] * rhs.y + self[6] * rhs.z + self[7] * rhs.w,
            self[8] * rhs.x + self[9] * rhs.y + self[10] * rhs.z + self[11] * rhs.w,
            self[12] * rhs.x + self[13] * rhs.y + self[14] * rhs.z + self[15] * rhs.w,
        )
    }
}

impl Index<usize> for Matrix4 {
    type Output = f32;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        if index > 15 {
            error!("Trying to reach out of bounds index");
            return &std::f32::NAN;
        }

        &self.entries[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4::zero()
    }
}

impl Display for Matrix4 {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "Matrix4: \n {} {} {} {} \n {} {} {} {} \n {} {} {} {} \n {} {} {} {}",
            self[0],
            self[1],
            self[2],
            self[3],
            self[4],
            self[5],
            self[6],
            self[7],
            self[8],
            self[9],
            self[10],
            self[11],
            self[12],
            self[13],
            self[14],
            self[15]
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::math::{Vector3, Vector4, Matrix4};

    #[test]
    fn zero() {
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
                    assert_eq!(mat[i + 4 * j], 1.0);
                } else {
                    assert_eq!(mat[i + 4 * j], 0.0);
                }
            }
        }
    }

    #[test]
    fn new_translation() {
        let mat = Matrix4::new_traslation(3.0, 4.0, 5.0);

        assert_eq!(mat[0], 1.0);
        assert_eq!(mat[5], 1.0);
        assert_eq!(mat[10], 1.0);
        assert_eq!(mat[15], 1.0);

        assert_eq!(mat[3], 3.0);
        assert_eq!(mat[7], 4.0);
        assert_eq!(mat[11], 5.0);
    }

    #[test]
    fn new_rotation_x() {
        let mat = Matrix4::new_rotation_x(90.0);

        assert_eq!(mat[0].round(), 1.0);
        assert_eq!(mat[5].round(), 0.0);
        assert_eq!(mat[10].round(), 0.0);
        assert_eq!(mat[15], 1.0);

        assert_eq!(mat[6], 1.0);
        assert_eq!(mat[9], -1.0);
    }

    #[test]
    fn new_rotation_y() {
        let mat = Matrix4::new_rotation_y(90.0);

        assert_eq!(mat[0].round(), 0.0);
        assert_eq!(mat[5], 1.0);
        assert_eq!(mat[10].round(), 0.0);

        assert_eq!(mat[2], -1.0);
        assert_eq!(mat[8], 1.0);
        assert_eq!(mat[15], 1.0);
    }

    #[test]
    fn new_rotation_z() {
        let mat = Matrix4::new_rotation_z(90.0);

        assert_eq!(mat[0].round(), 0.0);
        assert_eq!(mat[5].round(), 0.0);
        assert_eq!(mat[10], 1.0);

        assert_eq!(mat[1], 1.0);
        assert_eq!(mat[4], -1.0);
        assert_eq!(mat[15], 1.0);
    }

    #[test]
    fn new_rotation() {
        let mat = Matrix4::new_rotation(Vector3::X, 90.0);

        assert_eq!(mat[0].round(), 1.0);
        assert_eq!(mat[5].round(), 0.0);
        assert_eq!(mat[10].round(), 0.0);
        assert_eq!(mat[15].round(), 1.0);

        assert_eq!(mat[6].round(), 1.0);
        assert_eq!(mat[9].round(), -1.0);
        
    }
    
    #[test]
    fn new_scale() {
        let mat = Matrix4::new_scale(3.0, 4.0, 5.0);

        assert_eq!(mat[0], 3.0);
        assert_eq!(mat[5], 4.0);
        assert_eq!(mat[10], 5.0);
        assert_eq!(mat[15], 1.0);
    }

    #[test]
    fn new_scale_vector() {
        let mat = Matrix4::new_scale_vector(Vector3::new(3.0, 4.0, 5.0));

        assert_eq!(mat[0], 3.0);
        assert_eq!(mat[5], 4.0);
        assert_eq!(mat[10], 5.0);
        assert_eq!(mat[15], 1.0);
    }

    #[test]
    fn new_scale_axis() {
        let mat = Matrix4::new_scale_axis(Vector3::X, 3.0);

        assert_eq!(mat[0], 3.0);
        assert_eq!(mat[5], 1.0);
        assert_eq!(mat[10], 1.0);
        assert_eq!(mat[15], 1.0);
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
        let mut mat = Matrix4::new();
        mat.translate(3.0, 4.0, 5.0);

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
        let mut mat = Matrix4::new();
        mat.translate_by_vector(Vector3::new(3.0, 3.0, 3.0));

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
        let mut mat = Matrix4::new();
        mat.scale(2.0, 3.0, 5.0);

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
        let mut mat = Matrix4::new();
        mat.scale_axis(Vector3::X, 2.0);

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
        let mut mat = Matrix4::new();
        mat.scale_axis(Vector3::Y, 2.0);

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
        let mut mat = Matrix4::new();
        mat.scale_axis(Vector3::Z, 2.0);

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
        let mut mat = Matrix4::new();
        mat.rotate_x(90.0);

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
        let mut mat = Matrix4::new();
        mat.rotate_y(90.0);

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
        let mut mat = Matrix4::new();
        mat.rotate_z(90.0);

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
        let mut mat = Matrix4::new();
        mat.rotate(Vector3::X, 90.0);

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
        let mut mat = Matrix4::new();
        mat.rotate(Vector3::Y, 90.0);

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
        let mut mat = Matrix4::new();
        mat.rotate(Vector3::Z, 90.0);

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
    fn rotate_z_increment() {}

    #[test]
    fn transformed_vector() {
        let mut mat = Matrix4::new();
        mat.scale(2.0, 1.0, 1.0);
        mat.rotate_y(90.0);
        mat.translate(5.0, 3.0, 4.0);

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
        let mut mat = Matrix4::new();
        mat.scale(2.0, 2.0, 2.0);
        mat.rotate_x(90.0);

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

        let mut mat = Matrix4::new();
        mat.translate(5.0, 7.0, 10.0);

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