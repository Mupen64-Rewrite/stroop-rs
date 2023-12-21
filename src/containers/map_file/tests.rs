use crate::containers::{map_file::MapFile, sm64_types::BaseType};

const SM64_US_MAP: &str = include_str!("../../map_files/sm64.us.map");

#[test]
fn parse_sm64_us_map() {
    let map = MapFile::new(SM64_US_MAP);

    assert_eq!(map.get_offset(BaseType::gMarioStates), Some(0x33b170));
}
