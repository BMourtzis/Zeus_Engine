use gfx_hal::{
    format::Aspects,
    image::SubresourceRange,
    window::Extent2D
};

pub const VERSION: &str = "0.1.1";

pub const DIMS: Extent2D = Extent2D {
    width: 1024,
    height: 768,
};

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
