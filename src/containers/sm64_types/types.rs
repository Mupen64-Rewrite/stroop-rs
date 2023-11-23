//! Contains wrapped types that interacts with memory in SM64

use crate::containers::types::Padding;

use super::*;

#[repr(C)]
#[derive(Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Default)]
pub struct Mario {
    _pad0: Padding<0x3C>,
    pub pos: Vec3,
}

impl SM64Container for Mario {}

impl ContainerInfo for Mario {
    fn get_base_type() -> BaseType {
        BaseType::gMarioStates
    }
}
