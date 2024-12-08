use bitflags::bitflags;

use crate::containers::types::Padding;

use super::{general::*, *};

bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct MarioHatState: u32 {
        const SHOULD_HAVE_HAT = 0b0000_0001;
        const VANISH_CAP = 0b0000_0010;
        const METAL_CAP = 0b0000_0100;
        const WING_CAP = 0b0000_1000;

        const HAT_ON_HEAD = 0x10;
        const HAT_IN_HAND = 0x20;
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Mario {
    _pad0: Padding<0x4>,      // 0x00
    hat_state: MarioHatState, // 0x04
    _pad1: Padding<0x16>,     // 0x08
    water_shell_timer: i16,   // 0x1A TODO: is this right
    _pad2: Padding<0x2>,      // 0x1C
    squished_timer: u16,      // 0x1E
    _pad3: Padding<0x4>,      // 0x20
    yaw_intended: Angle,      // 0x24
    hitstun_timer: i16,       // 0x26 TODO: is this right
    _pad4: Padding<0x4>,      // 0x28
    pitch: Angle,             // 0x2C
    yaw_facing: Angle,        // 0x2E
    roll: Angle,              // 0x30
    pitch_vel: Angle,         // 0x32
    yaw_vel: Angle,           // 0x34
    roll_vel: Angle,          // 0x36
    _pad5: Padding<0x4>,      // 0x38
    yaw_moving: Angle,        // 0x38
    twirl_yaw: Angle,         // 0x3A
    pos: Vec3,                // 0x3C
    spd: Vec3,                // 0x48
    h_spd: f32,               // 0x54
    x_sliding_spd: f32,       // 0x58
    z_sliding_spd: f32,       // 0x5C
    _pad6: Padding<0x10>,     // 0x60
    floor_height: f32,        // 0x70
    floor_yaw: Angle,         // 0x74
    _pad7: Padding<0x3E>,     // 0x76
    unsquishing_timer: i8,    // 0xB4 TODO: is this right
    _pad8: Padding<0x1>,      // 0xB5
    cap_timer: u16,           // 0xB6
    _pad9: Padding<0x4>,      // 0xB8
    peak_height: f32,         // 0xBC
    _pad10: Padding<0x18a>,   // 0xC0
    holp_type: u8,            // 0x24A TODO: enum
    _pad11: Padding<0xD>,     // 0x24B
    holp: Vec3,               // 0x258
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
