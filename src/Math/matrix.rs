use super::vector::Vector3;

#[derive(Debug)]
pub struct Matrix4 {
    entries: [[f64; 4]; 4]
}

impl Matrix4 {
    fn new() -> Matrix4 {
        Matrix4 { entries: [[0.0; 4]; 4]}
    }

    pub fn identity() -> Matrix4{
        let mut result = Matrix4::new();

        for i in 0..4{
            for j in 0..4{
                if i == j{
                    result.entries[i][j] = 1.0;
                }
            }
        }

        result
    }

    pub fn from_vectors(a: Vector3, b: Vector3, c: Vector3) -> Matrix4 {
        let mut result = Matrix4::new();

        result.entries[0][0] = a.x;
        result.entries[0][1] = a.y;
        result.entries[0][2] = a.z;
        result.entries[0][3] = 0.0;

        result.entries[1][0] = b.x;
        result.entries[1][1] = b.y;
        result.entries[1][2] = b.z;
        result.entries[1][3] = 0.0;

        result.entries[2][0] = c.x;
        result.entries[2][1] = c.y;
        result.entries[2][2] = c.z;
        result.entries[2][3] = 0.0;

        result.entries[3][0] = 0.0;
        result.entries[3][1] = 0.0;
        result.entries[3][2] = 0.0;
        result.entries[3][3] = 1.0;

        result
    }

    pub fn transform(&self){

    }

    pub fn invert(&self){

    }

    pub fn tranpose(&self){

    }
}

#[derive(Debug)]
pub struct Matrix3
{
    entries: [[f64; 3]; 3]
}

impl Matrix3 
{
    pub fn new() -> Matrix3
    {
        Matrix3 {
            entries: [[0.0; 3]; 3]
        }
    }
}