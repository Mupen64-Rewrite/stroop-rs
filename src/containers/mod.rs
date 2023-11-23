//! Module for giving interfaces to reading / writing in SM64.

use self::{
    emulator::EmulatorMemory,
    map_file::MapFile,
    sm64_types::{types::*, SM64Container},
};

mod emulator;
mod map_file;
mod sm64_types;
mod types;

/// Represents the state of SM64 in an emulator with a specific map file that represents the offsets of types in SM64.
struct SM64State {
    emulator: EmulatorMemory,
    map_file: MapFile,
    pub mario: Mario,
}

impl SM64State {
    pub fn new(emulator: EmulatorMemory, map_file: MapFile) -> Self {
        Self {
            emulator,
            map_file,
            mario: Mario::default(),
        }
    }

    /// Updates the internal state of SM64 by reading from memory.
    pub fn update(&mut self) {
        self.mario.read_from_memory(&self.map_file, &self.emulator);
    }

    /// Writes the internal state of SM64 to memory.
    pub fn write(&mut self) {
        self.mario.write_to_memory(&self.map_file, &self.emulator);
    }
}
