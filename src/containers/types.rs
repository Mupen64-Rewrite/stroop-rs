/// A simple padding type to fill in struct with padding bytes.
///
/// # Generic
/// - `N` - The number of padding bytes to add
#[repr(C)]
pub struct Padding<const N: usize>([u8; N]);

impl<const N: usize> Default for Padding<N> {
    fn default() -> Self {
        Self([0; N])
    }
}
