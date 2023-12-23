//! Contains container types that wraps around stuff in SM64

use stroop_emu_mem::Emulator;
use strum::{Display, EnumCount, EnumIter};
use thiserror::Error;

use crate::map_file::MapFile;

mod types;

/// A base type of something in SM64.
///
/// This is used to determine the base offset of a type through the map file.
///
/// # Naming
/// The name directly corresponds to the name of the type in the map file.
#[allow(non_camel_case_types)]
#[repr(usize)]
#[derive(EnumIter, Display, PartialEq, Eq, EnumCount, Clone, Copy)]
pub enum BaseType {
    gMarioStates,
}

/// Represents the base type of something in SM64.
pub trait SM64Container: Default + ContainerInfo {
    /// Updates internal state by reading from memory.
    fn update_read<E: Emulator>(
        &mut self,
        map_file: &MapFile,
        emulator: E,
    ) -> Result<(), ContainerIOError> {
        // only update if we have a valid offset
        let offset =
            Self::get_base_type_offset(map_file).ok_or(ContainerIOError::NoBaseTypeOffset)?;
        *self = emulator.read(offset)?;
        Ok(())
    }
}

pub trait ContainerInfo {
    fn get_base_type() -> BaseType;

    fn get_base_type_offset(map_file: &MapFile) -> Option<usize> {
        let base_type = Self::get_base_type();
        map_file.get_offset(base_type)
    }
}

/// Represents a pending write to the emulator.
struct PendingWrite<'a, T: ContainerInfo>(&'a T);

impl<'a, T: SM64Container> PendingWrite<'a, T> {
    /// Updates emulator memory by writing to it.
    fn write<E: Emulator>(&self, map_file: &MapFile, emulator: E) -> Result<(), ContainerIOError> {
        // only update if we have a valid offset
        let offset = <T as ContainerInfo>::get_base_type_offset(map_file)
            .ok_or(ContainerIOError::NoBaseTypeOffset)?;
        emulator.write(offset, self.0)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ContainerIOError {
    #[error("failed to write to emulator")]
    EmulatorWriteFail(#[from] Box<dyn std::error::Error>),
    #[error("failed to get offset for base type")]
    NoBaseTypeOffset,
}
