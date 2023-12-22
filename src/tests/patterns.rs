use crate::map_file::guess_offsets::types::Pattern;
use stroop_macros::pattern;

#[test]
fn match_exact() {
    let bytes = &[0x00, 0x01, 0x02, 0x03, 0x04];
    let pattern = pattern!(0, "01 02 03");

    assert_eq!(pattern.matches(bytes), Some(1));
}

#[test]
fn match_exact_offset() {
    let bytes = &[0x00, 0x01, 0x02, 0x03, 0x04];
    let pattern = pattern!(1, "01 02 03");

    assert_eq!(pattern.matches(bytes), Some(2));
}

#[test]
fn match_exact_offset2() {
    let bytes = &[0x00, 0x01, 0x02, 0x03, 0x04];
    let pattern = pattern!(3, "01 02 03");

    assert_eq!(pattern.matches(bytes), Some(4));
}

#[test]
fn match_exact_offset3() {
    let bytes = &[0x00, 0x01, 0x02, 0x03, 0x04];
    let pattern = pattern!(isize::MAX, "01 02 03");

    assert_eq!(pattern.matches(bytes), Some(isize::MAX as usize + 1));
}

#[test]
fn match_exact_neg_offset() {
    let bytes = &[0x00, 0x01, 0x02, 0x03, 0x04];
    let pattern = pattern!(-1, "01 02 03");

    assert_eq!(pattern.matches(bytes), Some(0));
}

#[test]
fn match_start() {
    let bytes = &[0x00, 0x01, 0x02, 0x03];
    let pattern = pattern!(0, "00 01 02");

    assert_eq!(pattern.matches(bytes), Some(0));
}

#[test]
fn match_end() {
    let bytes = &[0x00, 0x01, 0x02, 0x03];
    let pattern = pattern!(0, "01 02 03");

    assert_eq!(pattern.matches(bytes), Some(1));
}

#[test]
fn match_retry() {
    let bytes = &[0x00, 0x02, 0x00, 0x01, 0x02, 0x03];
    let pattern = pattern!(0, "00 01 02");

    assert_eq!(pattern.matches(bytes), Some(2));
}

#[test]
fn match_wildcard() {
    let bytes = &[0x00, 0x02, 0x00, 0x01, 0x02, 0x03];
    let pattern = pattern!(0, "00 ?? 02");

    assert_eq!(pattern.matches(bytes), Some(2));
}

#[test]
fn not_found_head() {
    let bytes = &[0x00, 0x01, 0x02, 0x03];
    let pattern = pattern!(0, "04");

    assert_eq!(pattern.matches(bytes), None);
}

#[test]
fn not_match() {
    let bytes = &[0x00, 0x01, 0x00, 0x02, 0x00, 0x03];
    let pattern = pattern!(0, "00 04");

    assert_eq!(pattern.matches(bytes), None);
}

#[test]
fn not_enough_data() {
    let bytes = &[0x00];
    let pattern = pattern!(0, "00 01");

    assert_eq!(pattern.matches(bytes), None);
}

#[test]
fn offset_underflow() {
    let bytes = &[0x00, 0x01, 0x02, 0x03];
    let pattern = pattern!(-1, "00 01 02");

    assert_eq!(pattern.matches(bytes), None);
}
