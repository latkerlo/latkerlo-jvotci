use regex::Regex;

use crate::{
    data::{
        FOLLOW_VOWEL_CLUSTERS, INITIAL, MZ_VALID, START_VOWEL_CLUSTERS, VALID, ZIHEVLA_INITIAL,
    },
    exceptions::Jvonunfli,
    tools::{char, regex_replace_all, slice, slice_},
};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tarmi {
    Hyphen,
    Cvccv,
    Cvcc,
    Ccvcv,
    Ccvc,
    Cvc,
    Cvhv,
    Ccv,
    Cvv,
    OtherRafsi,
}

pub const SONORANT_CONSONANTS: &str = "lmnr";

#[derive(Debug, PartialEq)]
pub enum BrivlaType {
    Gismu,
    Zihevla,
    Lujvo,
    ExtendedLujvo,
    Rafsi,
    Cmevla,
}
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum YHyphenSetting {
    #[default]
    Standard,
    AllowY,
    ForceY,
}
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ConsonantSetting {
    #[default]
    Cluster,
    TwoConsonants,
    OneConsonant,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Settings {
    pub generate_cmevla: bool,
    pub y_hyphens: YHyphenSetting,
    pub exp_rafsi: bool,
    pub consonants: ConsonantSetting,
    pub glides: bool,
    pub allow_mz: bool,
}

/// Auto-impl `Display` on an enum
#[macro_export]
macro_rules! auto_to_string {
    ($($e:ident),*) => {
        use std::fmt::{self, Display, Formatter};
        $(
            impl Display for $e {
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    write!(f, "{self:?}")
                }
            }
        )*
    };
}

auto_to_string!(BrivlaType, YHyphenSetting, ConsonantSetting, Tarmi);

/// True if `c` is a vowel (non-*y*)
pub fn is_vowel(c: char) -> bool {
    "aeiou".contains(c)
}
/// True if `c` is a consonant
pub fn is_consonant(c: char) -> bool {
    "bcdfgjklmnprstvxz".contains(c)
}
/// True if `s` is an on-glide (*i*/*u* + vowel)
pub fn is_glide(s: &str) -> bool {
    s.len() == 2 && "iu".contains(char(s, 0)) && is_vowel(char(s, 1))
}
/// True if there are only Lojban letters in `s` (non-*y*, -period, -comma)
pub fn is_only_lojban_characters(s: &str) -> bool {
    s.chars().all(|c| "aeioubcdfgjklmnprstvxz'".contains(c))
}
/// True if any character is a consonant
pub fn contains_consonant(s: &str) -> bool {
    s.chars().any(is_consonant)
}

/// True if `v` is CVCCV or CCVCV. Doesn't check clusters
pub fn is_gismu_shape(v: &str) -> bool {
    v.len() == 5
        && is_consonant(char(v, 0))
        && is_consonant(char(v, 3))
        && is_vowel(char(v, 4))
        && (is_vowel(char(v, 1)) && is_consonant(char(v, 2))
            || is_consonant(char(v, 1)) && is_vowel(char(v, 2)))
}
/// True if `v` is a valid gismu
pub fn is_gismu(v: &str, settings: &Settings) -> bool {
    is_gismu_shape(v)
        && if is_vowel(char(v, 1)) {
            if settings.allow_mz {
                MZ_VALID.to_vec()
            } else {
                VALID.to_vec()
            }
            .contains(&slice(v, 2, 4))
        } else {
            INITIAL.contains(&slice(v, 0, 2))
        }
}

/// Split consecutive vowels into syllables
pub fn split_vowel_cluster(v: &str) -> Result<Vec<String>, Jvonunfli> {
    let old_v = v;
    let mut v = v;
    let mut res = VecDeque::new();
    macro_rules! add_to_res {
        ($new_c:expr) => {
            let new_v = slice(v, 0, -($new_c.len() as isize));
            if char($new_c, 0) == 'i' && ["ai", "ei", "oi"].contains(&slice_(new_v, -2))
                || char($new_c, 0) == 'u' && slice_(new_v, -2) == "au"
            {
                return Err(Jvonunfli::DecompositionError(format!(
                    "couldn't decompose {{{old_v}}}"
                )));
            }
            res.push_front($new_c.to_string());
        };
    }
    loop {
        if v.len() > 3 && FOLLOW_VOWEL_CLUSTERS.contains(&slice_(v, -3)) {
            add_to_res!(slice_(v, -3));
            v = slice(v, 0, -3);
        } else if v.len() > 2 && FOLLOW_VOWEL_CLUSTERS.contains(&slice_(v, -2)) {
            add_to_res!(slice_(v, -2));
            v = slice(v, 0, -2);
        } else if START_VOWEL_CLUSTERS.contains(&v) {
            res.push_front(v.to_string());
            return Ok(res.iter().cloned().collect());
        } else {
            return Err(Jvonunfli::DecompositionError(format!(
                "couldn't decompose {{{old_v}}}"
            )));
        }
    }
}

/// True if `c` can start a zi'evla
pub fn is_zihevla_initial_cluster(c: &str) -> bool {
    c.len() <= 3
        && (c.len() == 2 && INITIAL.contains(&c)
            || c.len() == 3
                && INITIAL.contains(&slice(c, 0, 2))
                && ZIHEVLA_INITIAL.contains(&slice_(c, 1)))
}
/// True if `c` can be in a zi'evla
pub fn is_zihevla_middle_cluster(c: &str) -> bool {
    if c.len() < 3
        || c.len() == 3
            && (SONORANT_CONSONANTS.contains(char(c, 1))
                || VALID.contains(&slice(c, 0, 2)) && INITIAL.contains(&slice_(c, 1)))
    {
        return true;
    }
    // i don't know how many of these parentheses are unnecessary
    let regex = Regex::new(r"^([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstv00xz][lmnr])*)?$").unwrap();
    let matches = if char(c, -2) == 'm' && INITIAL.contains(&slice_(c, -2)) {
        regex.captures(slice(
            c,
            0,
            if is_zihevla_initial_cluster(slice_(c, -3)) {
                -3
            } else {
                -2
            },
        ))
    } else {
        Regex::new(
            "^([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)(?:\
             ([bcdfgjkpstvxz][bcdfgjklmnprstvxz]?[lmnr]?)|([bcdfgjklmnprstvxz]))$",
        )
        .unwrap()
        .captures(c)
    }
    .unwrap();
    matches.get(matches.len() - 2).is_some()
        && is_zihevla_initial_cluster(&matches[matches.len() - 2])
}

/// True if `r` is a valid CLL rafsi
pub fn is_valid_rafsi(r: &str, settings: &Settings) -> bool {
    let t = rafsi_tarmi(r);
    if [Tarmi::Cvccv, Tarmi::Cvcc].contains(&t) {
        if settings.allow_mz {
            MZ_VALID.to_vec()
        } else {
            VALID.to_vec()
        }
        .contains(&slice(r, 2, 4))
    } else if [Tarmi::Ccvcv, Tarmi::Ccvc, Tarmi::Ccv].contains(&t) {
        INITIAL.contains(&slice(r, 0, 2))
    } else {
        1 <= t as i8 && t as i8 <= 8
    }
}

/// Get the shape of a rafsi
pub fn rafsi_tarmi(r: &str) -> Tarmi {
    let l = r.len();
    if l == 0 {
        return Tarmi::OtherRafsi;
    } else if l == 2 && slice(r, 0, 2) == "'y" {
        return Tarmi::Hyphen;
    } else if l != 1 && !is_consonant(char(r, 0)) {
        return Tarmi::OtherRafsi;
    }
    match l {
        1 => {
            if is_vowel(char(r, 0)) {
                Tarmi::OtherRafsi
            } else {
                Tarmi::Hyphen
            }
        }
        3 => match (is_vowel(char(r, 1)), is_vowel(char(r, 2))) {
            (true, true) => Tarmi::Cvv,
            (true, false) => Tarmi::Cvc,
            (false, true) => Tarmi::Ccv,
            _ => Tarmi::OtherRafsi,
        },
        4 => match (is_vowel(char(r, 1)), is_vowel(char(r, 3))) {
            (true, true) if char(r, 2) == '\'' => Tarmi::Cvhv,
            (true, false) if is_consonant(char(r, 2)) => Tarmi::Cvcc,
            (false, false) if is_vowel(char(r, 2)) => Tarmi::Ccvc,
            _ => Tarmi::OtherRafsi,
        },
        5 if is_gismu_shape(r) => {
            if is_vowel(char(r, 2)) {
                Tarmi::Ccvcv
            } else {
                Tarmi::Cvccv
            }
        }
        _ => Tarmi::OtherRafsi,
    }
}
/// Remove hyphens from the rafsi
pub fn strip_hyphens(r: &str) -> String {
    regex_replace_all("^['y]+|['y]+$", r, "")
}
/// Get the rafsi's shape without hyphens
pub fn tarmi_ignoring_hyphen(r: &str) -> Tarmi {
    rafsi_tarmi(&strip_hyphens(r))
}
