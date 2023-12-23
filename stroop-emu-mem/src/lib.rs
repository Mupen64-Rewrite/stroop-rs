use std::error::Error;

pub mod emulators;
pub mod errors;

pub use emulators::*;

pub trait Emulator {
    /// Tries to read value from N64 memory at address
    fn read<T: Copy>(&self, address: usize) -> Result<T, Box<dyn Error>>;

    /// Tries to write value to N64 memory at address
    fn write<T: Copy>(&self, address: usize, value: &T) -> Result<(), Box<dyn Error>>;

    /// Dumps the emulator's RAM
    fn ram_dump(&self) -> Result<Vec<u8>, Box<dyn Error>>;
}
