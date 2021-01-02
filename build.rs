use std::path::Path;
use std::{env, fs};

fn main() {
    let target_dir_path = env::var("OUT_DIR").unwrap();

    create_data_paths(&target_dir_path);

    //shaders
    copy(&target_dir_path, "data/shaders/quad.vert");
    copy(&target_dir_path, "data/shaders/quad.frag");

    //textures
    copy(&target_dir_path, "data/textures/logo.png");
    copy(&target_dir_path, "data/textures/statue.jpg");
    copy(&target_dir_path, "data/textures/error.png");
    copy(&target_dir_path, "data/textures/viking_room.png");

    //models
    copy(&target_dir_path, "data/models/viking_room.mtl");
    copy(&target_dir_path, "data/models/viking_room.obj");
}

fn create_data_paths(target_dir_path: &str) {
    let data_path = Path::new(&target_dir_path).join("../../../data");

    if !data_path.exists() {
        fs::create_dir(data_path).unwrap();
    }

    let shaders_path = Path::new(&target_dir_path).join("../../../data/shaders");

    if !shaders_path.exists() {
        fs::create_dir(shaders_path).unwrap();
    }

    let textures_path = Path::new(&target_dir_path).join("../../../data/textures");

    if !textures_path.exists() {
        fs::create_dir(textures_path).unwrap();
    }

    let models_path = Path::new(&target_dir_path).join("../../../data/models");

    if !models_path.exists() {
        fs::create_dir(models_path).unwrap();
    }
}

fn copy<S: AsRef<std::ffi::OsStr> + ?Sized, P: Copy + AsRef<Path>>(
    target_dir_path: &S,
    file_name: P,
) {
    fs::copy(
        file_name,
        Path::new(&target_dir_path).join("../../..").join(file_name),
    )
    .unwrap();
}
