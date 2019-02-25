mod math;

use math::matrix;

fn main() {
    let matrix = matrix::Matrix4::identity();
    println!("{:?}", matrix);
}
