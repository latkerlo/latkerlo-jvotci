use crate::{
    data::{FOLLOW_VOWEL_CLUSTERS, INITIAL, MZ_VALID, START_VOWEL_CLUSTERS, VALID},
    exceptions::Jvonunfli,
    jvozba::Tosytype,
    strin, strsl,
    tools::regex_replace_all,
};
use itertools::{iproduct, Itertools as _};
use regex::Regex;
use std::{collections::VecDeque, fmt, str::FromStr, sync::LazyLock};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BrivlaType {
    Gismu,
    Zihevla,
    Lujvo,
    ExtendedLujvo,
    Rafsi,
    Cmevla,
}
/// Hyphen options for gluing CVV or CV'V rafsi to the front.
///
/// Setting `AllowY` makes *'y* a valid replacement for CLL's *r*/*n* hyphens. `ForceY` requires
/// *'y*, trating e.g. *voirli'u* as a zi'evla.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum YHyphenSetting {
    #[default]
    Standard,
    AllowY,
    ForceY,
}
/// Minimum consonant requirements.
///
/// With a non`Standard` [`YHyphenSetting`], there are some strings e.g. *nei'ynei* that cannot fall
/// apart or combine with other words, and do not break any of Lojban's morphology. Setting
/// `TwoConsonants` or `OneConsonant` lets these be valid lujvo.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ConsonantSetting {
    #[default]
    Cluster,
    TwoConsonants,
    OneConsonant,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct Settings {
    /// Whether the lujvo should end in a consonant. This only affects *making* lujvo, not
    /// decomposing them
    pub generate_cmevla: bool,
    pub y_hyphens: YHyphenSetting,
    pub consonants: ConsonantSetting,
    /// Whether any cmavo not containing *y* may be a rafsi
    pub exp_rafsi: bool,
    /// Whether semivowel *i* and *u* are treated as consonants. Together with `consonants`,
    /// `exp_rafsi`, and `y_hyphens` this may poduce lujvo with no actual consonants like
    /// *ia'yia*
    pub glides: bool,
    /// Whether *mz* is a valid cluster
    pub allow_mz: bool,
}

/// Keep only certain fields of a [`Settings`] and replace the rest with their defaults
#[macro_export]
macro_rules! extract {
    ($s:ident, $($part:ident),+) => {
        Settings {
            $($part: $s.$part),+,
            ..Settings::default()
        }
    };
}
/// A list of every [`Settings`]
pub static SETTINGS_ITERATOR: LazyLock<Vec<Settings>> = LazyLock::new(|| {
    iproduct!(
        ["", "c"],
        ["", "A", "F"],
        ["", "2", "1"],
        ["", "r"],
        ["", "g"],
        ["", "z"]
    )
    .map(
        |(generate_cmevla, y_hyphens, exp_rafsi, consonants, glides, allow_mz)| {
            Settings::from_str(&format!(
                "{generate_cmevla}{y_hyphens}{exp_rafsi}{consonants}{glides}{allow_mz}"
            ))
            .unwrap()
        },
    )
    .collect_vec()
});

impl fmt::Display for Settings {
    /// A representation of `self` as a string. Can be reparsed with the `FromStr` implementation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "{}{}{}{}{}{}",
            if self.generate_cmevla { "c" } else { "" },
            match self.y_hyphens {
                YHyphenSetting::Standard => "",
                YHyphenSetting::AllowY => "A",
                YHyphenSetting::ForceY => "F",
            },
            match self.consonants {
                ConsonantSetting::Cluster => "",
                ConsonantSetting::TwoConsonants => "2",
                ConsonantSetting::OneConsonant => "1",
            },
            if self.exp_rafsi { "r" } else { "" },
            if self.glides { "g" } else { "" },
            if self.allow_mz { "z" } else { "" },
        );
        write!(f, "{s}")
    }
}
#[derive(Debug)]
pub struct SettingsError;
impl FromStr for Settings {
    type Err = SettingsError;
    /// Returns a `SettingsError` if given any characters other than `cSAFC21rgz` or there are
    /// multiple of any. `crgz` activate `generate_cmevla`, `exp_rafsi`, `glides`, and `allow_mz`;
    /// `SAF` and `C21` select a [`YHyphenSetting`] and [`ConsonantSetting`]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "crgz"
            .chars()
            .any(|x| s.chars().filter(|c| *c == x).count() > 1)
            || s.chars().filter(|c| "SAF".contains(*c)).count() > 1
            || s.chars().filter(|c| "C21".contains(*c)).count() > 1
            || s.chars().filter(|c| !"cSAFC21rgz".contains(*c)).count() != 0
        {
            return Err(SettingsError);
        }
        let generate_cmevla = s.contains('c');
        let exp_rafsi = s.contains('r');
        let glides = s.contains('g');
        let allow_mz = s.contains('z');
        let y_hyphens = if s.contains('A') {
            YHyphenSetting::AllowY
        } else if s.contains('F') {
            YHyphenSetting::ForceY
        } else {
            YHyphenSetting::Standard
        };
        let consonants = if s.contains('2') {
            ConsonantSetting::TwoConsonants
        } else if s.contains('1') {
            ConsonantSetting::OneConsonant
        } else {
            ConsonantSetting::Cluster
        };
        Ok(Self {
            generate_cmevla,
            y_hyphens,
            consonants,
            exp_rafsi,
            glides,
            allow_mz,
        })
    }
}

/// Auto-impl `Display` on an enum
#[macro_export]
macro_rules! auto_to_string {
    ($($e:ident),*) => {
        $(
            impl std::fmt::Display for $e {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{self:?}")
                }
            }
        )*
    };
}

auto_to_string!(
    BrivlaType,
    YHyphenSetting,
    ConsonantSetting,
    Tarmi,
    Tosytype
);

#[inline]
/// True if `c` is a vowel (non-*y*)
pub fn is_vowel(c: char) -> bool {
    "aeiou".contains(c)
}
#[inline]
/// True if `c` is a consonant
pub fn is_consonant(c: char) -> bool {
    "bcdfgjklmnprstvxz".contains(c)
}
#[inline]
/// True if `s` is an on-glide (*i*/*u* + vowel)
pub fn is_glide(s: &str) -> bool {
    s.len() >= 2 && "iu".contains(strin!(s, 0)) && is_vowel(strin!(s, 1))
}
/// True if there are only Lojban letters in `s` (non-*y*, -period, -comma)
pub fn is_only_lojban_characters(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| "aeioubcdfgjklmnprstvxz'".contains(c))
}
/// True if any character is a consonant
pub fn contains_consonant(s: &str) -> bool {
    s.chars().any(is_consonant)
}

/// True if `v` is CVCCV or CCVCV. Doesn't check clusters
pub fn is_gismu_shape(v: &str) -> bool {
    v.len() == 5
        && is_consonant(strin!(v, 0))
        && is_consonant(strin!(v, 3))
        && is_vowel(strin!(v, 4))
        && (is_vowel(strin!(v, 1)) && is_consonant(strin!(v, 2))
            || is_consonant(strin!(v, 1)) && is_vowel(strin!(v, 2)))
}
/// True if `v` is a valid gismu
pub fn is_gismu(v: &str, settings: &Settings) -> bool {
    is_gismu_shape(v)
        && if is_vowel(strin!(v, 1)) {
            if settings.allow_mz { &MZ_VALID } else { &VALID }.contains(&strsl!(v, 2..4))
        } else {
            INITIAL.contains(&strsl!(v, 0..2))
        }
}

/// Split consecutive vowels into syllables
/// # Errors
/// if given a bad vowel sequence
pub fn split_vowel_cluster(v: &str) -> Result<Vec<String>, Jvonunfli> {
    let old_v = v;
    let mut v = v;
    let mut res = VecDeque::new();
    macro_rules! add_to_res {
        ($new_c:expr) => {
            let new_v = strsl!(v, 0..-($new_c.len() as isize));
            if strin!($new_c, 0) == 'i' && ["ai", "ei", "oi"].contains(&strsl!(new_v, -2..))
                || strin!($new_c, 0) == 'u' && strsl!(new_v, -2..) == "au"
            {
                return Err(Jvonunfli::DecompositionError(format!(
                    "{{{old_v}}} is a bad vowel sequence"
                )));
            }
            res.push_front($new_c.to_string());
        };
    }
    loop {
        if v.len() > 3 && FOLLOW_VOWEL_CLUSTERS.contains(&strsl!(v, -3..)) {
            add_to_res!(strsl!(v, -3..));
            v = strsl!(v, 0..-3);
        } else if v.len() > 2 && FOLLOW_VOWEL_CLUSTERS.contains(&strsl!(v, -2..)) {
            add_to_res!(strsl!(v, -2..));
            v = strsl!(v, 0..-2);
        } else if START_VOWEL_CLUSTERS.contains(&v) {
            res.push_front(v.to_string());
            return Ok(res.iter().cloned().collect());
        } else {
            return Err(Jvonunfli::DecompositionError(format!(
                "{{{old_v}}} is a bad vowel sequence"
            )));
        }
    }
}

/// True if `c` can start a zi'evla
pub fn is_zihevla_initial_cluster(c: &str) -> bool {
    match c.len() {
        1 => true,
        2 => INITIAL.contains(&c),
        3 => INITIAL.contains(&strsl!(c, 0..2)) && INITIAL.contains(&strsl!(c, 1..)),
        _ => false,
    }
}

static ZIHEVLA_MIDDLE_1: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)?$").unwrap()
});
static ZIHEVLA_MIDDLE_2: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        "^([bcdfgjklmnprstvxz])?((?:[bcdfgjklmnprstvxz][lmnr])*)(?:\
         ([bcdfgjkpstvxz][bcdfgjklmnprstvxz]?[lmnr]?)|([bcdfgjklmnprstvxz]))$",
    )
    .unwrap()
});
/// True if `c` can be in a zi'evla
#[allow(clippy::missing_panics_doc)] // .unwrap()
pub fn is_zihevla_middle_cluster(c: &str) -> bool {
    if c.len() < 3
        || c.len() == 3
            && (SONORANT_CONSONANTS.contains(strin!(c, 1))
                || VALID.contains(&strsl!(c, 0..2)) && INITIAL.contains(&strsl!(c, 1..)))
    {
        return true;
    }
    // i don't know how many of these parentheses are unnecessary
    let matches = if strin!(c, -2) == 'm' && INITIAL.contains(&strsl!(c, -2..)) {
        ZIHEVLA_MIDDLE_1.captures(strsl!(
            c,
            0..-2 - is_zihevla_initial_cluster(strsl!(c, -3..)) as isize
        ))
    } else {
        ZIHEVLA_MIDDLE_2.captures(c)
    };
    if matches.is_none() {
        return false;
    }
    let matches = matches.unwrap();
    matches.get(matches.len() - 2).is_none()
        || is_zihevla_initial_cluster(&matches[matches.len() - 2])
}

#[inline]
/// True if `r` is a valid CLL rafsi
pub fn is_valid_rafsi(r: &str, settings: &Settings) -> bool {
    let t = rafsi_tarmi(r);
    if [Tarmi::Cvccv, Tarmi::Cvcc].contains(&t) {
        if settings.allow_mz { &MZ_VALID } else { &VALID }.contains(&strsl!(r, 2..4))
    } else if [Tarmi::Ccvcv, Tarmi::Ccvc, Tarmi::Ccv].contains(&t) {
        INITIAL.contains(&strsl!(r, 0..2))
    } else {
        1 <= t as i8 && t as i8 <= 8
    }
}

#[inline]
/// Get the shape of a rafsi
pub fn rafsi_tarmi(r: &str) -> Tarmi {
    let l = r.len();
    if l == 0 {
        return Tarmi::OtherRafsi;
    } else if r == "'y" {
        return Tarmi::Hyphen;
    } else if l != 1 && !is_consonant(strin!(r, 0)) {
        return Tarmi::OtherRafsi;
    }
    match l {
        1 if !is_vowel(strin!(r, 0)) => Tarmi::Hyphen,
        3 => match (is_vowel(strin!(r, 1)), is_vowel(strin!(r, 2))) {
            (true, false) if is_consonant(strin!(r, 2)) => Tarmi::Cvc,
            (true, true) => Tarmi::Cvv,
            (false, true) => Tarmi::Ccv,
            _ => Tarmi::OtherRafsi,
        },
        4 if strin!(r, 3) != '\'' => {
            match (is_vowel(strin!(r, 1)), strin!(r, 2), is_vowel(strin!(r, 3))) {
                (true, '\'', true) => Tarmi::Cvhv,
                (true, _, false) if is_consonant(strin!(r, 3)) => Tarmi::Cvcc,
                (false, v, false) if is_vowel(v) => Tarmi::Ccvc,
                _ => Tarmi::OtherRafsi,
            }
        }
        5 if is_gismu_shape(r) => {
            if is_vowel(strin!(r, 2)) {
                Tarmi::Ccvcv
            } else {
                Tarmi::Cvccv
            }
        }
        _ => Tarmi::OtherRafsi,
    }
}

static BOUNDARY_Y_HYPHENS: LazyLock<Regex> = LazyLock::new(|| Regex::new("^['y]+|['y]+$").unwrap());
#[inline]
/// Remove hyphens from the rafsi
pub fn strip_hyphens(r: &str) -> String {
    regex_replace_all(&BOUNDARY_Y_HYPHENS, r, "")
}
/// Get the rafsi's shape without hyphens
pub fn tarmi_ignoring_hyphen(r: &str) -> Tarmi {
    rafsi_tarmi(&strip_hyphens(r))
}
