use process_memory::*;
use sysinfo::{ProcessesToUpdate, System};

use crate::{Emulator, errors::StaticMemoryEmulatorError};

pub struct StaticMemoryEmulator {
    n64_offset: usize,
    ram_size: usize,
    handle: ProcessHandle,
}

impl StaticMemoryEmulator {
    pub fn new(
        n64_offset: usize,
        ram_extension: bool,
        process_name: &str,
    ) -> Result<Self, StaticMemoryEmulatorError> {
        let ram_size = if ram_extension {
            0x800000 // 8mb
        } else {
            0x400000 // 4mb
        };

        let mut system = System::new();
        system.refresh_processes(ProcessesToUpdate::All, true);

        let process = system
            .processes_by_name(process_name.as_ref())
            .next()
            .ok_or(StaticMemoryEmulatorError::NoProcessFound)?;

        let pid = process.pid().as_u32() as Pid;

        let handle = pid.try_into_process_handle()?;

        Ok(Self {
            n64_offset,
            ram_size,
            handle,
        })
    }
}

impl Emulator for StaticMemoryEmulator {
    fn read<T: Copy>(&self, address: usize) -> Result<T, Box<dyn std::error::Error>> {
        let member = DataMember::new_offset(self.handle, vec![self.n64_offset + address]);
        let value = unsafe { member.read() }?;
        Ok(value)
    }

    fn write<T: Copy>(&self, address: usize, value: &T) -> Result<(), Box<dyn std::error::Error>> {
        let member = DataMember::new_offset(self.handle, vec![self.n64_offset + address]);
        member.write(value)?;
        Ok(())
    }

    fn ram_dump(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let ram = copy_address(self.n64_offset, self.ram_size, &self.handle)?;
        Ok(ram)
    }
}
