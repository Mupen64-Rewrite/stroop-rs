use crate::containers::sm64_types::BaseType;

use self::pattern::Pattern;

mod pattern;

pub struct GuessOffset {
    pub base_type: BaseType,
    pub patterns: &'static [Pattern],
}

pub const GUESS_OFFSETS: &[GuessOffset] = &[
    // GuessOffset {
    //     base_type: BaseType::gMarioStates,
    //     patterns: &[pattern!(4, 80 ?? ?? ?? 33 b1 70), pattern!(50, 12 34 ?? 78)],
    // },
    // GuessOffset {
    //     base_type: BaseType::foo,
    //     patterns: &[pattern!(-24, 80 ?? ?? ?? 33 b1 70)],
    // },
];
