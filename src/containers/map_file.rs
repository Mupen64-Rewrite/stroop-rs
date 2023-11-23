use std::{iter::repeat, mem::variant_count};

use super::sm64_types::BaseType;

/// Represents a map file holding the offsets of types in SM64.
pub struct MapFile(Vec<usize>);

impl MapFile {
    /// Parses a map file.
    fn new(input: &str) -> Self {
        let mut offsets = repeat(0).take(BASE_TYPE_COUNT).collect::<Vec<_>>();

        todo!();

        Self(offsets)
    }

    /// Gets the offset of a type in the map file.
    pub fn get_offset(&self, base_type: BaseType) -> usize {
        self.0[base_type as usize]
    }
}

const BASE_TYPE_COUNT: usize = variant_count::<BaseType>();
