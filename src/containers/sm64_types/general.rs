//! Contains wrapped types that interacts with memory in SM64

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Angle = u16;
