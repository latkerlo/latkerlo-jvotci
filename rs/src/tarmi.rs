//! Various methods for determining the shape of a word.

use crate::{
    data::{INITIAL, VALID},
    tools::char,
};
use regex::Regex;

/// The shape (*tarmi*) of a rafsi.
#[derive(PartialEq, Clone, Debug)]
pub enum Tarmi {
    Hyphen = 0,
    Cvccv = 1,
    Cvcc = 2,
    Ccvcv = 3,
    Ccvc = 4,
    Cvc = 5,
    Cvhv = 6,
    Ccv = 7,
    Cvv = 8,
    Fuhivla = 9,
}
impl Tarmi {
    pub fn as_usize(&self) -> usize {
        self.clone() as usize
    }
}

pub fn is_vowel(c: char) -> bool {
    "aeiou".contains(c)
}

pub fn is_consonant(c: char) -> bool {
    "bcdfgjklmnprstvxz".contains(c)
}

/// Does not include *y*, since internally this is only used on (valid) rafsi.
pub fn is_only_lojban_characters(valsi: &str) -> bool {
    Regex::new("^[aeioubcdfgjklmnprstvxz']+$")
        .unwrap()
        .is_match(valsi)
}

/// This only checks if the *shape* is CVCCV or CCVCV, ignoring if the clusters are valid.
pub fn is_gismu(valsi: &str) -> bool {
    valsi.len() == 5
        && is_consonant(char(valsi, 0))
        && is_consonant(char(valsi, 3))
        && is_vowel(char(valsi, 4))
        && ((is_vowel(char(valsi, 1)) && is_consonant(char(valsi, 2)))
            || (is_consonant(char(valsi, 1)) && is_vowel(char(valsi, 2))))
}

pub fn is_valid_rafsi(rafsi: &str) -> bool {
    let raftai = rafsi_tarmi(rafsi);
    if [Tarmi::Cvccv, Tarmi::Cvcc].contains(&raftai) {
        return VALID.contains(&&rafsi[2..4]);
    }
    if [Tarmi::Ccvcv, Tarmi::Ccvc, Tarmi::Ccv].contains(&raftai) {
        return INITIAL.contains(&&rafsi[0..2]);
    }
    1 <= raftai.as_usize() && raftai.as_usize() <= 8
}

pub fn rafsi_tarmi(rafsi: &str) -> Tarmi {
    match rafsi.len() {
        0 => Tarmi::Fuhivla,
        1 => Tarmi::Hyphen,
        2 if char(rafsi, 0) == '\'' && char(rafsi, 1) == 'y' => Tarmi::Hyphen,
        3 if is_consonant(char(rafsi, 0)) => {
            match (is_vowel(char(rafsi, 1)), is_consonant(char(rafsi, 2))) {
                (true, false) => Tarmi::Cvv,
                (true, true) => Tarmi::Cvc,
                (false, false) => Tarmi::Ccv,
                _ => Tarmi::Fuhivla,
            }
        }
        4 if is_consonant(char(rafsi, 0)) => match (
            is_vowel(char(rafsi, 1)),
            is_consonant(char(rafsi, 2)),
            is_consonant(char(rafsi, 3)),
        ) {
            (true, false, false) if char(rafsi, 2) == '\'' && char(rafsi, 3) != 'y' => Tarmi::Cvhv,
            (true, true, true) => Tarmi::Cvcc,
            (false, false, true) => Tarmi::Ccvc,
            _ => Tarmi::Fuhivla,
        },
        5 if is_gismu(rafsi) => match is_vowel(char(rafsi, 2)) {
            true => Tarmi::Ccvcv,
            false => Tarmi::Cvccv,
        },
        _ if rafsi.len() != 1 && !is_consonant(char(rafsi, 0)) => Tarmi::Fuhivla,
        _ => Tarmi::Fuhivla,
    }
}

pub fn tarmi_ignoring_hyphen(mut rafsi: &str) -> Tarmi {
    if rafsi.ends_with('y') {
        rafsi = &rafsi[..rafsi.len() - 1];
    }
    rafsi_tarmi(rafsi)
}
