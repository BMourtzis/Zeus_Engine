use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    //Inits
    pub fn new() -> Vector3 
    {
        Vector3 { 
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn from_vector(v: &Vector3) -> Vector3
    {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z
        }
    }

    pub fn from_scalar(x:f64, y:f64, z:f64) -> Vector3
    {
        Vector3 { x, y, z}
    }

    //Additions
    pub fn sum(mut self, v: &Vector3) {
        self.x += v.x;
        self.y += v.z;
        self.z += v.z;
    }

    pub fn sum_scalar(mut self, f: f64) {
        self.x += f;
        self.y += f;
        self.z += f;
    }

    pub fn sum_scaled_vectos(mut self, v: &Vector3, s: f64) {
        self.x += v.x * s;
        self.y += v.z * s;
        self.z += v.z * s;
    }

    pub fn sum_vectors(mut self, a: &Vector3, b: &Vector3) {
        self.x += a.x + b.x;
        self.y += a.y + b.y;
        self.z += a.z + b.z;
    }

    //Subtractions
    pub fn sub(mut self, v: &Vector3) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }

    pub fn sub_scalar(mut self, s: f64) {
        self.x -= s;
        self.y -= s;
        self.z -= s;
    }

    pub fn sub_vectors(mut self, a: &Vector3, b: &Vector3) {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        self.z = a.z + b.z;
    }

    //Mutlipliers
    pub fn mul(mut self, v: &Vector3) {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
    }

    pub fn mul_scalar(mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }

    pub fn mul_vectors(mut self, a: &Vector3, b: &Vector3) {
        self.x = a.x * b.x;
        self.y = a.y * b.y;
        self.z = a.z * b.z;
    }

    //Dividers
    pub fn div(mut self, v: &Vector3) {
        self.x /= v.x;
        self.y /= v.y;
        self.z /= v.z;
    }

    pub fn div_scalar(mut self, s: f64) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }

    //Others
    // pub fn dot() -> f64 {}

    pub fn cross(&self) {}

    // pub length(&self) -> f64 {}

    // pub manhanttan_length(&self) -> f64 {}
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Default for Vector3 {
    fn default() -> Self
    {
        Vector3::new()
    }
}

#[derive(Debug)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {

}