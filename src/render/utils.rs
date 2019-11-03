use gfx_hal::{
    Backend
};
use super::{
    buffer::BufferState,
    desc::DescSet
};

pub struct Dimensions<T> {
    width: T,
    height: T
}

enum Color {
    Red,
    Green,
    Blue,
    Alpha
}

pub struct Uniform<B: Backend> {
    buffer: Option<BufferState<B>>,
    desc: Option<DescSet<B>>
}