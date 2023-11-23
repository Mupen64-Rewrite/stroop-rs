//! Contains container types that wraps around stuff in SM64

use super::{emulator::EmulatorMemory, map_file::MapFile};

mod types;

/// Represents the base type of something in SM64.
pub trait SM64Container {
    /// Updates internal state by reading from memory.
    fn update(&mut self, map_file: &MapFile, emulator: &EmulatorMemory);
}
