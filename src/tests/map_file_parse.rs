use crate::{containers::sm64_types::BaseType, map_file::MapFile};

const SM64_US_MAP: &str = include_str!("../map_files/sm64.us.map");
const SM64_US_MAP_WIN_NEWLINE: &str = "0x000000008033b171                gMarioStatess\r\n0x000000008033b170                gMarioStates\r\n";
const SM64_US_MAP_UNIX_NEWLINE: &str = "0x000000008033b171                gMarioStatess\n0x000000008033b170                gMarioStates\n";

#[test]
fn parse_sm64_us_map() {
    let map = MapFile::new(SM64_US_MAP);

    assert_eq!(map.get_offset(BaseType::gMarioStates), Some(0x33b170));
}

#[test]
fn parse_sm64_us_map_win() {
    let map = MapFile::new(SM64_US_MAP_WIN_NEWLINE);

    assert_eq!(map.get_offset(BaseType::gMarioStates), Some(0x33b170));
}

#[test]
fn parse_sm64_us_map_unix() {
    let map = MapFile::new(SM64_US_MAP_UNIX_NEWLINE);

    assert_eq!(map.get_offset(BaseType::gMarioStates), Some(0x33b170));
}

#[test]
fn invalid_offset() {
    // dummy file
    let map = MapFile::new("foo");

    assert_eq!(map.get_offset(BaseType::gMarioStates), None);
}
