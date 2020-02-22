extern crate env_logger;
extern crate zeus_render;
extern crate zeus_core;

use env_logger::{
    Builder,
    Target
};

use log::LevelFilter;

fn main() {
    build_logger();

    zeus_render::render();
}

fn build_logger() {
    Builder::new()
        .target(Target::Stdout)
        // .filter_level(LevelFilter::Debug)
        .filter_module("gfx_backend_vulkan", LevelFilter::Warn)
        .filter_module("winit", LevelFilter::Warn)
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .init();
}
