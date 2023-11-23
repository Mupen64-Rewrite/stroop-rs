//! Contains container types that wraps around stuff in SM64

use strum::{Display, EnumIter};

use super::{emulator::EmulatorMemory, map_file::MapFile};

pub mod types;

/// A base type of something in SM64.
///
/// This is used to determine the base offset of a type through the map file.
///
/// # Naming
/// The name directly corresponds to the name of the type in the map file.
#[allow(non_camel_case_types)]
#[repr(usize)]
#[derive(EnumIter, Display)]
pub enum BaseType {
    gMarioStates,
}

/// Represents the base type of something in SM64.
pub trait SM64Container: Default + ContainerInfo {
    /// Updates internal state by reading from memory.
    fn read_from_memory(&mut self, map_file: &MapFile, emulator: &EmulatorMemory) {
        let offset = Self::get_base_type_offset(map_file);
        *self = emulator.read(offset);
    }

    /// Writes internal state to emulator memory.
    fn write_to_memory(&self, map_file: &MapFile, emulator: &EmulatorMemory) {
        let offset = Self::get_base_type_offset(map_file);
        emulator.write(offset, self);
    }
}

pub trait ContainerInfo {
    fn get_base_type() -> BaseType;

    fn get_base_type_offset(map_file: &MapFile) -> usize {
        let base_type = Self::get_base_type();
        map_file.get_offset(base_type)
    }
}
