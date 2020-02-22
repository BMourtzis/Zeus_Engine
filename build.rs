use std::path::Path;
use std::{env, fs};

// const SETTINGS_FILE: &str = "Settings.toml";
// const LOG4RS_FILE: &str = "log4rs.toml";
const QUAD_VERT: &str = "data/quad.vert";
const QUAD_FRAG: &str = "data/quad.frag";
const LOGO: &str = "data/logo.png";
const ERROR: &str = "data/error.png";


fn main() {
    let target_dir_path = env::var("OUT_DIR").unwrap();

    let data_path = Path::new(&target_dir_path).join("../../../data");

    if !data_path.exists() {
        fs::create_dir(data_path).unwrap();
    }

    copy(&target_dir_path, QUAD_VERT);
    copy(&target_dir_path, QUAD_FRAG);
    copy(&target_dir_path, LOGO);
    copy(&target_dir_path, ERROR);

}

fn copy<S: AsRef<std::ffi::OsStr> + ?Sized, P: Copy + AsRef<Path>>(target_dir_path: &S, file_name: P) {
    fs::copy(file_name, Path::new(&target_dir_path).join("../../..").join(file_name)).unwrap();
}