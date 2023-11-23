//! Contains wrapped types that interacts with memory in SM64

use crate::containers::{map_file::*, types::Padding};

use super::*;

#[repr(C)]
#[derive(Default)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
#[derive(Default)]
struct Mario {
    _pad0: Padding<0x3C>,
    pos: Vec3,
}

impl Mario {
    fn get_pos(&self) -> &Vec3 {
        &self.pos
    }

    fn set_pos(&self, pos: Vec3) {
        todo!()
    }
}

impl SM64Container for Mario {
    fn update(&mut self, map_file: &MapFile, emulator: &EmulatorMemory) {
        *self = emulator.read(map_file.get_offset(BaseType::gMarioStates));
    }
}
