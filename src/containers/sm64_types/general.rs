//! Contains wrapped types that interacts with memory in SM64

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Angle = u16;
