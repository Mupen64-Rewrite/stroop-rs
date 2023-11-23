//! Contains code that interacts with emulator memory

use std::mem;

use super::sm64_types::SM64Container;

/// Container to read / write from the emulator's N64 RAM
pub struct EmulatorMemory {
    n64_offset: usize,
    ram_size: usize,
}

impl EmulatorMemory {
    /// Creates a new instance of EmulatorMemory
    /// # Arguments
    /// - `n64_offset` - The offset to where the start of the N64 RAM is in the emulator
    /// - `ram_extension` - Whether the N64 RAM has been extended to 8mb or not
    fn new(n64_offset: usize, ram_extension: bool) -> Self {
        let ram_size = if ram_extension {
            0x800000 // 8mb
        } else {
            0x400000 // 4mb
        };

        Self {
            n64_offset,
            ram_size: ram_size,
        }
    }

    /// Reads bytes from the emulator's N64 RAM
    /// # Arguments
    /// - `address` - The N64 address to read from without the 0x80__ prefix
    /// - `size` - The number of bytes to read
    /// # Returns
    /// - A slice of bytes read from the N64 RAM
    /// - Number of actually read bytes
    /// # Note
    /// Check the number of bytes read to see if you read all the bytes you wanted to. It will show you how many bytes were read depending on if you tried to read more bytes than the N64 RAM has or anything else.
    fn read_bytes(&self, address: usize, size: usize) -> (&[u8], usize) {
        let size = self.fix_size(address, size);

        todo!()
    }

    /// Writes bytes to the emulator's N64 RAM
    /// # Arguments
    /// - `address` - The N64 address to write to without the 0x80__ prefix
    /// - `data` - The data to write
    /// # Returns
    /// - Number of bytes written
    /// # Note
    /// Check the number of bytes written to see if you wrote all the bytes you wanted to. It will show you how many bytes were written depending on if you tried to write more bytes than the N64 RAM has or anything else.
    fn write_bytes(&self, address: usize, data: &[u8]) -> usize {
        let size = self.fix_size(address, data.len());

        todo!()
    }

    /// Reads bytes into a container types
    #[allow(unsafe_code)]
    pub fn read<T>(&self, address: usize) -> T
    where
        T: SM64Container + Default,
    {
        let t = T::default();
        let size = mem::size_of::<T>();
        let (bytes, size) = self.read_bytes(address, size);
        unsafe {
            // copy to new instance
            let mut t = t;
            let t_ptr = &mut t as *mut T as *mut u8;
            t_ptr.copy_from_nonoverlapping(bytes.as_ptr(), size);
            t
        }
    }

    /// Fixes the size to read / write to the emulator's N64 RAM for safety
    fn fix_size(&self, address: usize, size: usize) -> usize {
        if address >= self.ram_size {
            return 0;
        }

        let end = match address.checked_add(size) {
            Some(end) => end,
            None => return usize::MAX - address,
        };
        if end > self.ram_size {
            self.ram_size - address
        } else {
            size
        }
    }
}
