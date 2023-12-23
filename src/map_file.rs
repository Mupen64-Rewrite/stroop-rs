use std::error::Error;

use stroop_emu_mem::Emulator;
use strum::{EnumCount, IntoEnumIterator};

use crate::containers::sm64_types::BaseType;

use self::guess_offsets::GUESS_OFFSETS;

pub mod guess_offsets;

/// Represents a map file holding the offsets of types in SM64.
pub struct MapFile(Vec<Option<usize>>);

impl MapFile {
    /// Parses a map file.
    pub fn new(input: &str) -> Self {
        let mut offsets = Vec::new();

        // grab map file entry assuming format:
        //                 0x000000008033b170                gMarioStates\n
        for base_type in BaseType::iter() {
            let base_type = format!("{base_type}");

            if let Some(base_type_index) = input.find(&base_type) {
                let mut addr_index = base_type_index;

                // go backwards until not whitespace
                while addr_index > 0 && &input[addr_index - 1..addr_index] == " " {
                    addr_index -= 1;
                }

                let addr_grab_len = 6;

                if let Some(addr_index_base) = addr_index.checked_sub(addr_grab_len) {
                    let addr = &input[addr_index_base..addr_index];
                    if let Ok(addr) = usize::from_str_radix(addr, 16) {
                        offsets.push(Some(addr));
                        continue;
                    }
                }
            }

            offsets.push(None);
        }

        Self(offsets)
    }

    /// Tries to figure out base type offsets based on the emulator's RAM.
    pub fn new_guess_offsets<E: Emulator>(emulator: E) -> Result<Self, Box<dyn Error>> {
        let mut map_file = Self(vec![None; BaseType::COUNT]);
        let ram_dump = emulator.ram_dump()?;

        for guess_offset in GUESS_OFFSETS {
            // if for some reason somebody has duplicate base types, we don't want to overwrite it
            // TODO check at compile time
            let base_type_index = guess_offset.base_type as usize;
            if map_file.0[base_type_index].is_some() {
                continue;
            }

            let offset = guess_offset
                .patterns
                .iter()
                .find_map(|pattern| pattern.matches(&ram_dump));

            map_file.0[base_type_index] = offset;
        }

        Ok(map_file)
    }

    /// Gets the offset of a type in the map file.
    pub fn get_offset(&self, base_type: BaseType) -> Option<usize> {
        self.0[base_type as usize]
    }
}
