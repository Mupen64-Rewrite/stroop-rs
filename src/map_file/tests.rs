use crate::{containers::sm64_types::BaseType, map_file::MapFile};

const SM64_US_MAP: &str = include_str!("../map_files/sm64.us.map");
const FAKE_MAP: &str = include_str!("tests.rs");

#[test]
fn parse_sm64_us_map() {
    let map = MapFile::new(SM64_US_MAP);

    assert_eq!(map.get_offset(BaseType::gMarioStates), Some(0x33b170));
}

#[test]
fn invalid_offset() {
    // dummy file
    let map = MapFile::new(FAKE_MAP);

    assert_eq!(map.get_offset(BaseType::gMarioStates), None);
}
