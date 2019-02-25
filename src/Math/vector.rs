#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    fn new(x:f64, y:f64, z:f64) -> Vector3
    {
        Vector3 { x, y, z}
    }
}

#[derive(Debug)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {

}