pub mod emulators;
pub mod errors;

use anyhow::Result;
pub use emulators::*;

pub trait Emulator {
    /// Tries to read value from N64 memory at address
    fn read<T: Copy>(&self, address: usize) -> Result<T>;

    /// Tries to write value to N64 memory at address
    fn write<T: Copy>(&self, address: usize, value: &T) -> Result<()>;

    /// Dumps the emulator's RAM
    fn ram_dump(&self) -> Result<Vec<u8>>;
}
