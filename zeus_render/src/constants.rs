use gfx_hal::{format::Aspects, image::SubresourceRange, window::Extent2D};

pub const VERSION: &str = "0.1.1";

pub const DIMS: Extent2D = Extent2D {
    width: 1024,
    height: 768,
};

pub const COLOR_RANGE: SubresourceRange = SubresourceRange {
    aspects: Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

pub const DEPTH_RANGE: SubresourceRange = SubresourceRange {
    aspects: Aspects::DEPTH,
    levels: 0..1,
    layers: 0..1
};
