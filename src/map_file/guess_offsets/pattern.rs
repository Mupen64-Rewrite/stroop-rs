pub struct Pattern {
    pattern: &'static [Option<u8>],
    offset: usize,
}

impl Pattern {
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

            return Some(data_offset + self.offset);
        }
    }
}
