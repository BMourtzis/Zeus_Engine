use gfx_hal::{
    window::Extent2D,
    image::SubresourceRange,
    format::Aspects
};

pub const DIMS: Extent2D = Extent2D { 
    width: 1024, 
    height: 768 
};

pub const COLOR_RANGE: SubresourceRange = SubresourceRange {
    aspects: Aspects::COLOR,
    levels: 0..1,
    layers: 0..1
};