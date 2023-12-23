//! Contains wrapped types that interacts with memory in SM64

use crate::containers::types::Padding;

use super::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Mario {
    _pad0: Padding<0x3C>,
    pos: Vec3,
}

impl Mario {
    pub fn get_pos(&self) -> &Vec3 {
        &self.pos
    }

    #[must_use]
    pub fn set_pos(&mut self, pos: Vec3) -> PendingWrite<Self> {
        self.pos = pos;
        PendingWrite(self)
    }
}

impl SM64Container for Mario {}

impl ContainerInfo for Mario {
    fn get_base_type() -> BaseType {
        BaseType::gMarioStates
    }
}
