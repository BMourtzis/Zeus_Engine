use std::path::Path;
use std::{env, fs};

// const SETTINGS_FILE: &str = "Settings.toml";
// const LOG4RS_FILE: &str = "log4rs.toml";---

fn main() {
    let target_dir_path = env::var("OUT_DIR").unwrap();

    let data_path = Path::new(&target_dir_path).join("../../../data");

    if !data_path.exists() {
        fs::create_dir(data_path).unwrap();
    }

    copy(&target_dir_path, "data/shaders/quad.vert");
    copy(&target_dir_path, "data/shaders/quad.frag");
    copy(&target_dir_path, "data/textures/logo.png");
    copy(&target_dir_path, "data/textures/statue.jpg");
    copy(&target_dir_path, "data/textures/error.png");
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
