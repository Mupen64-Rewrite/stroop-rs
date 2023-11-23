use std::{iter::repeat, mem::variant_count};

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

/// A base type of something in SM64.
///
/// This is used to determine the base offset of a type through the map file.
///
/// # Naming
/// The name directly corresponds to the name of the type in the map file.
#[allow(non_camel_case_types)]
#[repr(usize)]
pub enum BaseType {
    gMarioStates,
}

const BASE_TYPE_COUNT: usize = variant_count::<BaseType>();
