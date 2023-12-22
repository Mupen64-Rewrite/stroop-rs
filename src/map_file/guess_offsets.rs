//! Contains the patterns used to guess the offsets of the structs in the map file.

use self::types::*;
// use crate::containers::sm64_types::BaseType;
// use stroop_macros::pattern;

pub mod types;

pub const GUESS_OFFSETS: &[GuessOffset] = &[
    // GuessOffset {
    //     base_type: BaseType::gMarioStates,
    //     patterns: &[
    //         pattern!(4, "80 ?? ?? ?? 33 b1 70"),
    //         pattern!(50, "12 34 ?? 78"),
    //     ],
    // },
    // GuessOffset {
    //     base_type: BaseType::something_else,
    //     patterns: &[
    //         pattern!(4, "80 ?? ?? ?? 33 b1 70"),
    //         pattern!(50, "12 34 ?? 78"),
    //     ],
    // },
];
