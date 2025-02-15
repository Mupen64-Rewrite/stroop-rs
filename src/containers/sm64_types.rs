//! Contains container types that wraps around stuff in SM64

use stroop_emu_mem::Emulator;
use strum::{Display, EnumCount, EnumIter};
use thiserror::Error;

use crate::map_file::MapFile;

pub mod general;
pub mod specific;

pub use general::*;
pub use specific::*;

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
pub trait SM64Container: Default + ContainerInfo + Copy {
    /// Updates internal state by reading from memory.
    fn update_read(
        &mut self,
        map_file: &MapFile,
        emulator: impl Emulator,
    ) -> Result<(), ContainerIOError> {
        // only update if we have a valid offset
        let offset =
            Self::get_base_type_offset(map_file).ok_or(ContainerIOError::NoBaseTypeOffset)?;
        *self = emulator
            .read(offset)
            .map_err(ContainerIOError::EmulatorReadFail)?;
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
///
/// The reason why this exists is due to the reason that we want to be able to chain writes together, rather than writing everything at once.
pub struct PendingWrite<'a, T: ContainerInfo>(&'a T);

impl<T: SM64Container> PendingWrite<'_, T> {
    /// Updates emulator memory by writing to it.
    pub fn write(
        &self,
        map_file: &MapFile,
        emulator: impl Emulator,
    ) -> Result<(), ContainerIOError> {
        // only update if we have a valid offset
        let offset = <T as ContainerInfo>::get_base_type_offset(map_file)
            .ok_or(ContainerIOError::NoBaseTypeOffset)?;
        emulator
            .write(offset, self.0)
            .map_err(ContainerIOError::EmulatorWriteFail)?;
        Ok(())
    }

    /// Combines two pending writes together.
    ///
    /// - Use when the two pending writes are of the same sm64 type.
    pub fn combine(&self, _other: Self) {} // same type so self can just write so other is ignored
}

#[derive(Debug, Error)]
pub enum ContainerIOError {
    #[error("failed to read from emulator memory: {0}")]
    EmulatorReadFail(anyhow::Error),
    #[error("failed to write to emulator memory: {0}")]
    EmulatorWriteFail(anyhow::Error),
    #[error("failed to get offset for base type")]
    NoBaseTypeOffset,
}
