use crate::containers::sm64_types::BaseType;

pub struct GuessOffset {
    pub base_type: BaseType,
    pub patterns: &'static [Pattern],
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    pattern: &'static [Option<u8>],
    offset: isize,
}

impl Pattern {
    pub const fn new(offset: isize, pattern: &'static [Option<u8>]) -> Self {
        if pattern.is_empty() {
            panic!("pattern must not be empty");
        }

        if pattern[0].is_none() {
            panic!("pattern must not start with None");
        }

        if pattern[pattern.len() - 1].is_none() {
            panic!("pattern must not end with None");
        }

        Self { pattern, offset }
    }

    pub fn matches(&self, mut data: &[u8]) -> Option<usize> {
        // we don't let pattern to be constructed by the user, so head is always Some
        let head = self.pattern[0].unwrap();
        let mut data_offset = 0;

        loop {
            // advance data slice to head
            data = match data.iter().position(|&f| f == head) {
                Some(index) => {
                    data_offset += index;
                    &data[index..]
                }
                None => return None,
            };

            // not enough bytes?
            if data.len() < self.pattern.len() {
                return None;
            }

            // check if pattern matches
            for pattern in self.pattern[1..].iter() {
                if let Some(pattern) = pattern {
                    if data[0] != *pattern {
                        data_offset += 1;
                        data = &data[1..];
                        continue;
                    }
                }
            }

            let offset = data_offset.checked_add_signed(self.offset)?;

            return Some(offset);
        }
    }
}
