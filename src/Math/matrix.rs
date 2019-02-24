#[derive(Debug)]
pub struct Matrix4 
{
    entries: [[f64; 4]; 4]
}

impl Matrix4 {
    pub fn new() -> Matrix4
    {
        Matrix4 {
            entries: [[0.0; 4]; 4]
        }
    }
}

#[derive(Debug)]
pub struct Matrix3
{
    entries: [[f64; 3]; 3]
}

impl Matrix3 {
    pub fn new() -> Matrix3
    {
        Matrix3 {
            entries: [[0.0; 3]; 3]
        }
    }
}