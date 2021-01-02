use gfx_hal::{
    format::{
        Aspects, Format
    },
    image::SubresourceRange,
    window::Extent2D
};

use super::model::Vertex;

use crate::zeus_core::math::{
    Vector2, Vector3, Vector4
};

pub const VERSION: &str = "0.1.2";

pub const DIMS: Extent2D = Extent2D {
    width: 1024,
    height: 768,
};

#[allow(dead_code)]
pub const COLOR_RANGE: SubresourceRange = SubresourceRange {
    aspects: Aspects::COLOR,
    level_start: 0,
    level_count: Some(1),
    layer_start: 0,
    layer_count: Some(1)
};

pub const DEPTH_RANGE: SubresourceRange = SubresourceRange {
    aspects: Aspects::DEPTH,
    level_start: 0,
    level_count: Some(1),
    layer_start: 0,
    layer_count: Some(1)
};

#[allow(dead_code)]
pub const VERTICES: [Vertex; 8] = [
    Vertex {
        a_pos: Vector3 { x: 0.5, y: -0.33, z: 2.5 },
        a_color: Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0 },
        a_uv: Vector2 { x: 0.0, y: 1.0 },
    },
    Vertex {
        a_pos: Vector3 { x: -0.5, y: -0.33, z: 2.5 },
        a_color: Vector4 {x: 0.0, y: 1.0, z: 0.0, w: 1.0 },
        a_uv: Vector2 { x: 1.0, y: 1.0 },
    },
    Vertex {
        a_pos: Vector3 { x: -0.5, y: 0.33, z: 2.5 },
        a_color: Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 1.0 },
        a_uv: Vector2 { x: 1.0, y: 0.0 },
    },
    Vertex {
        a_pos: Vector3 { x: 0.5, y: 0.33, z: 2.5 },
        a_color: Vector4 {x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
        a_uv: Vector2 { x: 0.0, y: 0.0 },
    },

    Vertex {
        a_pos: Vector3 { x: 1.5, y: -0.33, z: 3.5 },
        a_color: Vector4 {x: 1.0, y: 0.0, z: 0.0, w: 1.0 },
        a_uv: Vector2 { x: 0.0, y: 1.0 },
    },
    Vertex {
        a_pos: Vector3 { x: 0.5, y: -0.33, z: 3.5 },
        a_color: Vector4 {x: 0.0, y: 1.0, z: 0.0, w: 1.0 },
        a_uv: Vector2 { x: 1.0, y: 1.0 },
    },
    Vertex {
        a_pos: Vector3 { x: 0.5, y: 0.33, z: 3.5 },
        a_color: Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 1.0 },
        a_uv: Vector2 { x: 1.0, y: 0.0 },
    },
    Vertex {
        a_pos: Vector3 { x: 1.5, y: 0.33, z: 3.5 },
        a_color: Vector4 {x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
        a_uv: Vector2 { x: 0.0, y: 0.0 },
    },
];

#[allow(dead_code)]
pub const INDICES: [u32; 12] = [
    0, 1, 2, 2, 3, 0, //obj 1
    4, 5, 6, 6, 7, 4 //obj 2
];

pub const FILE_EXT: &str = r"[.]([a-zA-Z]*)$";
pub const IMAGE_FORMAT:Format = Format::Rgba8Srgb;
pub const DEPTH_IMAGE_FORMAT:Format = Format::D32SfloatS8Uint;