//! Functions for analyzing brivla in general (such as `analyze_brivla`!)

use std::ops::{Bound, RangeBounds};

use itertools::Itertools as _;
use regex::Regex;

#[allow(unused_imports)] // YHyphenSetting::self, needed for docs
use crate::{
    data::{
        BANNED_TRIPLES, FOLLOW_VOWEL_CLUSTERS, HYPHENS, INITIAL, MZ_VALID, START_VOWEL_CLUSTERS,
        VALID,
    },
    exceptions::Jvonunfli::{
        self, DecompositionError, FakeTypeError, InvalidClusterError, NotBrivlaError,
        NotZihevlaError,
    },
    extract,
    katna::{jvokaha, jvokaha2},
    tarmi::{
        BrivlaType::{self, Cmevla, ExtendedLujvo, Gismu, Lujvo, Rafsi, Zihevla},
        ConsonantSetting::{Cluster, OneConsonant, TwoConsonants},
        Settings,
        Tarmi::{self, Ccv, Cvc, Cvhv, Cvv, OtherRafsi},
        YHyphenSetting::{self, Standard},
        is_cmavo_compound, is_consonant, is_gismu, is_glide, is_valid_rafsi, is_vowel,
        is_zihevla_initial_cluster, is_zihevla_middle_cluster, rafsi_tarmi, split_vowel_cluster,
        strip_hyphens,
    },
};

#[inline]
#[must_use = "does not mutate the string"]
/// Replaces all matches of a regex in a string with another string.
pub fn regex_replace_all(regex: &Regex, from: &str, with: &str) -> String {
    regex.replace_all(from, with).to_string()
}

#[macro_export]
/// Pythonic `str`ing `in`dexing.
macro_rules! strin {
    ($s:expr, $i:expr) => {{
        let mut chars = ($s).chars();
        let len = chars.clone().count();
        let positive = |i| -> Option<usize> {
            let j = if i < 0 { len as isize + i } else { i } as usize;
            (j < len).then_some(j)
        };
        positive($i).map(|i| (&mut chars).nth(i).unwrap()).unwrap_or_default()
    }};
}
/// Turns a range into a tuple with its endpoints.
pub fn bounds<S, T, R>(str: S, range: R) -> (isize, isize)
where
    S: AsRef<str>,
    T: Clone + From<isize> + Into<isize> + std::ops::Add<T, Output = T>,
    R: RangeBounds<T>,
{
    let (start, end) = (range.start_bound(), range.end_bound());
    let start = match start {
        // start bounds cannot be excluded
        Bound::Included(b) | Bound::Excluded(b) => b.clone(),
        Bound::Unbounded => T::from(0),
    };
    let end = match end {
        Bound::Included(b) => b.clone() + T::from(1),
        Bound::Excluded(b) => b.clone(),
        Bound::Unbounded => T::from(str.as_ref().len() as isize),
    };
    (start.into(), end.into())
}
#[macro_export]
/// Pythonic `str`ing `sl`icing.
macro_rules! strsl {
    ($s:expr, $r:expr) => {{
        let len = ($s).len();
        let (start, end) = $crate::tools::bounds($s, $r);
        let positive = |i: isize| -> usize {
            if i < 0 { len.saturating_sub((-i) as usize) } else { i as usize }.min(len)
        };
        let (start, end) = (positive(start), positive(end));
        assert!(start <= end, "slice attempt problem: s={start} > e={end}");
        &$s[start..end]
    }};
}

/// Converts a word to standard form (*h* → *'*, no periods/commas, lowercase).
#[must_use = "does not mutate the string"]
pub fn normalize(word: &str) -> String {
    word.to_lowercase().trim_matches('.').replace('h', "'").replace(',', "")
}

/// Returns `true` if given a gismu or lujvo.
/// # Errors
/// Errors besides [`DecompositionError`] and [`InvalidClusterError`] are
/// forwarded from [`jvokaha`].
pub fn is_gismu_or_lujvo(s: &str, settings: &Settings) -> Result<bool, Jvonunfli> {
    if s.len() < 5 || !is_vowel(strin!(s, -1)) {
        Ok(false)
    } else if is_gismu(s, &extract!(settings; allow_mz)) {
        Ok(true)
    } else if let Err(e) = jvokaha(s, &extract!(settings; y_hyphens, allow_mz)) {
        match e {
            DecompositionError(_) | InvalidClusterError(_) => Ok(false),
            _ => Err(e),
        }
    } else {
        Ok(true)
    }
}

/// Returns `true` if given a slinku'i: the input isn't a valid word because
/// putting a CV cmavo in front of it makes it a lujvo (e.g. *pa**slinku'i***)
/// # Errors
/// Errors besides [`DecompositionError`] and [`InvalidClusterError`] are
/// forwarded from [`jvokaha`].
pub fn is_slinkuhi(s: &str, settings: &Settings) -> Result<bool, Jvonunfli> {
    if is_vowel(strin!(s, 0)) {
        // words starting with vowels have an invisible . at the start
        Ok(false)
    } else if let Err(e) = jvokaha(&format!("pa{s}"), &extract!(settings; y_hyphens, allow_mz)) {
        match e {
            DecompositionError(_) | InvalidClusterError(_) => Ok(false),
            _ => Err(e),
        }
    } else {
        Ok(true)
    }
}

/// Checks rules specific to zi'evla or experimental rafsi.
/// # Errors
/// A [`NotZihevlaError`] is returned if the input
/// - is too short to be a zi'evla (either in characters or syllables)
/// - is a tosmabru or slinku'i
/// - has invalid clusters or bad vowels
/// - is just a bunch of cmavo
/// - contains glides after consonants
/// - contains apostrophes in illegal places
#[allow(clippy::missing_panics_doc)] // .unwrap()
pub fn check_zihevla_or_rafsi(
    mut valsi: &str,
    settings: &Settings,
    require_zihevla: bool,
) -> Result<BrivlaType, Jvonunfli> {
    let valsi_ = valsi;
    if require_zihevla && valsi.len() < 4 {
        return Err(NotZihevlaError(format!("{{{valsi}}} is too short to be a zi'evla")));
    }
    let (
        mut chunk,
        mut pos,
        mut num_syllables,
        mut cluster_pos,
        mut num_consonants,
        mut final_consonant_pos,
    ) = (String::new(), 0, 0, None, 0, 0);
    while !valsi.is_empty() {
        if is_consonant(strin!(valsi, 0)) {
            while !valsi.is_empty() && is_consonant(strin!(valsi, 0)) {
                chunk += strsl!(valsi, 0..1);
                valsi = strsl!(valsi, 1..);
            }
            if chunk.len() >= 2 && cluster_pos.is_none() {
                if num_consonants > 1 {
                    // find where the lujvo really starts
                    let pos_ = (1..=pos).find(|p| {
                        let to_part = strsl!(valsi_, ..*p);
                        let smabru_part = strsl!(valsi_, *p..);
                        (is_vowel(strin!(to_part, -1)) || strin!(to_part, -1) == 'y')
                            && is_cmavo_compound(to_part)
                            && (is_glide(smabru_part)
                                || !START_VOWEL_CLUSTERS.iter().any(|v| {
                                    *v == format!(
                                        "{}{}",
                                        strin!(to_part, -1),
                                        strin!(smabru_part, 0)
                                    )
                                }))
                            && analyze_brivla(
                                smabru_part,
                                &extract!(
                                    settings; y_hyphens, consonants, exp_rafsi, glides,
                                    allow_mz
                                ),
                            )
                            .is_ok()
                    });
                    if let Some(pos_) = pos_ {
                        return Err(NotZihevlaError(format!(
                            "{{{valsi_}}} is a tosmabru: {{{} {}}}",
                            strsl!(valsi_, ..pos_),
                            strsl!(valsi_, pos_..)
                        )));
                    }
                }
                cluster_pos = Some(pos);
            }
            if num_syllables == 0 && chunk.len() >= 2 && !INITIAL.contains(&strsl!(&chunk, 0..2)) {
                return Err(NotZihevlaError(format!(
                    "{{{valsi_}}} starts with an invalid cluster"
                )));
            }
            for i in 0..chunk.len().saturating_sub(1) {
                let i = i as isize;
                let cluster = strsl!(&chunk, i..i + 2);
                if !if settings.allow_mz { &MZ_VALID } else { &VALID }.contains(&cluster) {
                    return Err(NotZihevlaError(format!(
                        "{{{valsi_}}} contains an invalid cluster"
                    )));
                }
            }
            for i in 0..chunk.len().saturating_sub(2) {
                let i = i as isize;
                let cluster = strsl!(&chunk, i..i + 3);
                if BANNED_TRIPLES.contains(&cluster) {
                    return Err(NotZihevlaError(format!(
                        "{{{valsi_}}} contains a banned triple (nts/ntc/ndz/ndj)"
                    )));
                }
            }
            if pos == 0 {
                if !is_zihevla_initial_cluster(&chunk) {
                    return Err(NotZihevlaError(format!(
                        "{{{valsi_}}} starts with an invalid cluster"
                    )));
                }
            } else if !is_zihevla_middle_cluster(&chunk) {
                return Err(NotZihevlaError(format!("{{{valsi_}}} contains an invalid cluster")));
            }
            final_consonant_pos = pos;
            num_consonants += chunk.len();
        } else if is_vowel(strin!(valsi, 0)) {
            while !valsi.is_empty() && is_vowel(strin!(valsi, 0)) {
                chunk += strsl!(valsi, 0..1);
                valsi = strsl!(valsi, 1..);
            }
            let syllables = split_vowel_cluster(&chunk);
            if let Err(e) = syllables {
                match e {
                    DecompositionError(_) => {
                        return Err(NotZihevlaError(format!(
                            "{{{valsi_}}} contains a bad vowel sequence"
                        )));
                    }
                    _ => return Err(e),
                }
            } else if cluster_pos.is_none()
                && strin!(valsi_, pos - 1) != '\''
                && let Ok(v @ [_, _, ..]) = syllables.as_deref()
                && pos as usize + v.concat().len() == valsi_.len()
            {
                return Err(NotZihevlaError(format!("{{{valsi_}}} is just a cmavo compound")));
            }
            if pos != 0
                && let Ok([first, ..]) = syllables.as_deref()
                && FOLLOW_VOWEL_CLUSTERS.contains(&first.as_str())
            {
                return Err(NotZihevlaError(format!(
                    "{{{valsi_}}} contains a glide after a non-vowel"
                )));
            }

            num_syllables += syllables.unwrap().len();
        } else if strin!(valsi, 0) == '\'' {
            chunk = "'".to_string();
            valsi = strsl!(valsi, 1..);
            if pos < 1 || !is_vowel(strin!(valsi_, pos - 1)) {
                return Err(NotZihevlaError(format!(
                    "{{{valsi_}}} contains an apostrophe not preceded by a vowel"
                )));
            }
            if valsi.is_empty() || !is_vowel(strin!(valsi_, pos + 1)) {
                return Err(NotZihevlaError(format!(
                    "{{{valsi_}}} contains an apostrophe not followed by a vowel"
                )));
            }
        } else {
            return Err(NotZihevlaError(format!(
                "{{{valsi_}}} contains unexpected character {{{}}} (u+{:04x})",
                strin!(valsi, 0),
                strin!(valsi, 0) as u32
            )));
        }
        pos += chunk.len() as isize;
        chunk = String::new();
    }
    if num_syllables < 2 && (require_zihevla || !settings.exp_rafsi) {
        return Err(NotZihevlaError(format!("{{{valsi_}}} doesn't have enough syllables")));
    }
    if num_syllables > 2 && cluster_pos > Some(0) {
        if is_brivla(strsl!(valsi_, cluster_pos.unwrap()..), &extract!(settings; y_hyphens)) {
            return Err(NotZihevlaError(format!(
                "{{{valsi_}}} is a tosmabru: {{{} {}}}",
                strsl!(valsi_, 0..cluster_pos.unwrap()),
                strsl!(valsi_, cluster_pos.unwrap()..)
            )));
        }
        for i in 1..cluster_pos.unwrap() {
            if (is_consonant(strin!(valsi_, cluster_pos.unwrap() - i))
                || is_glide(strsl!(valsi_, cluster_pos.unwrap() - i..)))
                && is_brivla(
                    strsl!(valsi_, cluster_pos.unwrap() - i..),
                    &extract!(settings; y_hyphens),
                )
            {
                return Err(NotZihevlaError(format!(
                    "{{{valsi_}}} is a tosmabru: {{{} {}}}",
                    strsl!(valsi_, 0..cluster_pos.unwrap() - i),
                    strsl!(valsi_, cluster_pos.unwrap() - i..)
                )));
            }
        }
    }
    if cluster_pos.is_none() {
        if require_zihevla {
            return Err(NotZihevlaError(format!("{{{valsi_}}} is just a cmavo or cmavo compound")));
        }
        if !is_consonant(strin!(valsi_, 0)) && !settings.exp_rafsi {
            return Err(NotZihevlaError(format!("{{{valsi_}}} is an invalid rafsi")));
        }
        if num_consonants > 1 {
            return Err(NotZihevlaError(format!("{{{valsi_}}} is just a cmavo compound")));
        }
        if final_consonant_pos > 0 {
            return Err(NotZihevlaError(format!("{{{valsi_}}} lacks a consonant cluster")));
        }
    } else if !(is_vowel(strin!(valsi_, 0)) && is_consonant(strin!(valsi_, 1)))
        && is_slinkuhi(valsi_, &extract!(settings; y_hyphens, allow_mz))?
    {
        return Err(NotZihevlaError(format!("{{{valsi_}}} is a slinku'i")));
    }
    Ok(if cluster_pos.is_none() { Rafsi } else { Zihevla })
}

/// Returns `true` if given a valid brivla.
#[must_use]
pub fn is_brivla(valsi: &str, settings: &Settings) -> bool {
    let b_type = analyze_brivla(
        valsi,
        &extract!(settings; y_hyphens, exp_rafsi, consonants, glides, allow_mz),
    );
    if let Ok(b_type) = b_type { b_type.0 != Cmevla } else { false }
}

/// Returns the type and decomposition of any brivla or decomposable cmevla.
/// Doesn't check the cmevla morphology rules.
/// # Errors
/// A [`NotBrivlaError`] is returned if the input
/// - is empty
/// - starts or ends with *y* or apostrophe
/// - is a non-decomposable cmevla
/// - begins with CCV'y (explained in the docs for [`jvokaha2`])
/// - fails [`check_zihevla_or_rafsi`] with a [`NotZihevlaError`], in which case
///   the specific message is retained
///
/// or contains
/// - two adjacent or apostrophe-separated *y*s
/// - apostrophes or *y*s in illegal places
/// - a CCV rafsi or lujvo without the final vowel, e.g. *\*-vly-* or
///   *\*-bastry-*
/// - a tosmabru or slinku'i
/// - otherwise invalid rafsi
/// - anything that lacks enough consonants
/// - parts that are just cmavo or cmavo compounds
///
/// or starts with
/// - a cmavo followed by *'y* (unless a relevant [`YHyphenSetting`] is used)
///
/// Otherwise errors are forwarded from `check_zihevla_or_rafsi`, `jvokaha2`,
/// etc.
#[allow(clippy::missing_panics_doc)] // .unwrap()
pub fn analyze_brivla(
    valsi: &str,
    settings: &Settings,
) -> Result<(BrivlaType, Vec<String>), Jvonunfli> {
    let valsi = normalize(valsi);
    let mut is_cmetai = false;
    if valsi.is_empty() {
        return Err(NotBrivlaError("empty string".to_string()));
    }
    if is_consonant(strin!(&valsi, -1)) {
        is_cmetai = true;
    } else if !is_vowel(strin!(&valsi, -1)) {
        return Err(NotBrivlaError(format!("{{{valsi}}} doesn't end in a consonant or vowel")));
    }
    if is_cmetai {
        if is_gismu(&format!("{valsi}a"), &extract!(settings; allow_mz)) {
            return Err(NotBrivlaError(format!("{{{valsi}}} is a non-decomposable cmevla")));
        }
    } else if is_gismu(&valsi, &extract!(settings; allow_mz)) {
        return Ok((Gismu, vec![valsi]));
    }
    let res_parts = jvokaha(&valsi, &extract!(settings; y_hyphens, consonants, glides, allow_mz));
    if let Err(e) = res_parts {
        match e {
            DecompositionError(_) | InvalidClusterError(_) | FakeTypeError(_) => (),
            _ => return Err(e), // NotBrivlaError for CCV'y
        }
    } else {
        let res_parts = res_parts.unwrap();
        return Ok((if is_cmetai { Cmevla } else { Lujvo }, res_parts));
    }
    if !is_vowel(strin!(&valsi, 0)) && !is_consonant(strin!(&valsi, 0)) {
        return Err(NotBrivlaError(format!("{{{valsi}}} doesn't start with a consonant or vowel")));
    }
    let y_parts = valsi.split('y').collect_vec();
    if y_parts.len() == 1 {
        if is_cmetai {
            return Err(NotBrivlaError(format!("{{{valsi}}} is a non-decomposable cmevla")));
        }
        if let Err(e) = check_zihevla_or_rafsi(
            &valsi,
            &extract!(settings; y_hyphens, exp_rafsi, allow_mz),
            true,
        ) {
            match e {
                NotZihevlaError(m) => return Err(NotBrivlaError(m)),
                _ => return Err(e),
            }
        }
        return Ok((Zihevla, vec![valsi]));
    }
    let (
        mut res_parts,
        mut next_hyphen,
        mut has_cluster,
        mut is_mahortai,
        mut consonant_before_break,
        mut num_consonants,
    ) = (vec![], String::new(), false, true, false, 0);
    for i in 0..y_parts.len() {
        if i != 0 {
            next_hyphen += "y";
        }
        let mut part = y_parts[i];
        let mut part_ = part;
        if part.is_empty() {
            return Err(NotBrivlaError(format!("{{{valsi}}} contains two consecutive {{y}}s")));
        }
        if strin!(part, 0) == '\'' {
            part = strsl!(part, 1..);
            part_ = part;
            next_hyphen += "'";
            if part.is_empty() {
                return Err(NotBrivlaError(format!(
                    "{{{valsi}}} has a part consisting of just an apostrophe"
                )));
            }
            if !is_vowel(strin!(part, 0))
                || FOLLOW_VOWEL_CLUSTERS.contains(
                    &split_vowel_cluster(
                        &part.chars().take_while(|c| is_vowel(*c)).collect::<String>(),
                    )
                    .unwrap()[0]
                        .as_str(),
                )
            {
                return Err(NotBrivlaError(format!(
                    "{{{valsi}}} contains an apostrophe not followed by a non-glide vowel"
                )));
            }
        } else if i > 0 && is_vowel(strin!(part, 0)) && !is_glide(part) {
            return Err(NotBrivlaError(format!(
                "{{{valsi}}} contains a {{y}} not followed by an apostrophe or glide"
            )));
        }
        if !next_hyphen.is_empty() {
            res_parts.push(next_hyphen);
            next_hyphen = String::new();
        }
        if rafsi_tarmi(part) == Cvc {
            res_parts.push(part.to_string());
            consonant_before_break = true;
            num_consonants += 2;
            continue;
        }
        if rafsi_tarmi(&format!("{part}a")) == Ccv {
            return Err(NotBrivlaError(format!(
                "{{{valsi}}} contains a CCV rafsi without a vowel"
            )));
        }
        if i > 0 && (is_consonant(strin!(part, 0)) || is_glide(part)) {
            is_mahortai = false;
        }
        if consonant_before_break
            && (is_consonant(strin!(part, 0)) || settings.glides && is_glide(part))
        {
            has_cluster = true;
        }
        let mut can_be_rafsi = true;
        let mut require_cluster @ mut added_a = false;
        let part_a = &format!("{part}a");
        if strin!(part, -1) == '\'' {
            if settings.y_hyphens == Standard
                && !has_cluster
                && i < y_parts.len() - 1
                && strin!(y_parts[i + 1], 0) != '\''
            {
                require_cluster = true;
            }
            part = strsl!(part, 0..-1);
            part_ = part;
            next_hyphen += "'";
            if !is_vowel(strin!(part, -1)) {
                return Err(NotBrivlaError(format!(
                    "{{{part}}} contains an apostrophe not preceded by a vowel"
                )));
            }
        } else if i < y_parts.len() - 1 || is_cmetai {
            if is_vowel(strin!(part, -1)) {
                can_be_rafsi = false;
            }
            part = part_a;
            added_a = true;
            require_cluster = true;
        }
        let mut katnad = false;
        if can_be_rafsi {
            if !part_a.ends_with("'a")
                && !is_gismu(strsl!(part_a, -5..), &extract!(settings; allow_mz))
                && let Ok(decomp) = analyze_brivla(part_a, &extract!(settings; y_hyphens, allow_mz))
                && decomp.0 == Lujvo
            {
                return Err(NotBrivlaError(format!("{{{part_a}}} is a lujvo")));
            }
            let mut found_parts = jvokaha2(part_, &extract!(settings; y_hyphens, allow_mz));
            if let Err(ref e) = found_parts {
                match e {
                    DecompositionError(_) | InvalidClusterError(_) | FakeTypeError(_) => {
                        found_parts = Ok(vec![part.to_string()]);
                    }
                    _ => return Err(e.clone()),
                }
            } else {
                let found_parts = found_parts.clone().unwrap();
                if found_parts.len() < 2
                    && !is_valid_rafsi(&found_parts[0], &extract!(settings; allow_mz))
                {
                    return Err(NotBrivlaError(format!(
                        "{{{}}} is an invalid rafsi",
                        found_parts[0]
                    )));
                }
                res_parts.extend(found_parts.clone());
                katnad = true;
            }
            let found_parts = found_parts.unwrap();
            for fp in found_parts {
                let raftai = rafsi_tarmi(&fp);
                if [Cvv, Cvhv].contains(&raftai) {
                    num_consonants += 1;
                } else if raftai != OtherRafsi {
                    num_consonants += 2;
                    has_cluster = true;
                }
            }
        }
        if katnad {
            if i == 0 {
                let mut to_part = "";
                let mut smabru_part = "";
                if rafsi_tarmi(strsl!(part, 0..4)) == Cvhv {
                    to_part = strsl!(part, 0..4);
                    smabru_part = strsl!(part, 4..);
                } else if rafsi_tarmi(strsl!(part, 0..3)) == Cvv {
                    to_part = strsl!(part, 0..3);
                    smabru_part = strsl!(part, 3..);
                } else if is_consonant(strin!(part, 0)) && is_vowel(strin!(part, 1)) {
                    to_part = strsl!(part, 0..2);
                    smabru_part = strsl!(part, 2..);
                }
                if !smabru_part.is_empty() {
                    let hyphenless = strip_hyphens(smabru_part);
                    if added_a {
                        smabru_part = strsl!(smabru_part, 0..-1);
                    } else {
                        smabru_part = &hyphenless;
                    }
                    if is_valid_rafsi(smabru_part, &Settings::default())
                        && !(rafsi_tarmi(smabru_part) == Ccv
                            && strin!(strsl!(y_parts[i], to_part.len() as isize..), 3) == '\'')
                    {
                        return Err(NotBrivlaError(format!(
                            "{{{part}}} is a tosmabru: {{{to_part} {smabru_part}{}}}",
                            if added_a { "a" } else { "" }
                        )));
                    }
                    if let Err(e) = jvokaha(smabru_part, &extract!(settings; y_hyphens, allow_mz)) {
                        match e {
                            DecompositionError(_) | InvalidClusterError(_) | FakeTypeError(_) => (),
                            _ => return Err(e),
                        }
                    } else {
                        return Err(NotBrivlaError(format!(
                            "{{{part}}} is a tosmabru: {{{to_part} {smabru_part}{}}}",
                            if added_a { "a" } else { "" }
                        )));
                    }
                } else if settings.y_hyphens == Standard
                    && !is_cmetai
                    && !to_part.is_empty()
                    && !y_parts[i + 1].starts_with('\'')
                {
                    let rest = strsl!(&valsi, to_part.len() as isize + 2..);
                    if is_cmavo_compound(rest) {
                        return Err(NotBrivlaError(format!(
                            "{{{valsi}}} is just a cmavo compound"
                        )));
                    }
                    if analyze_brivla(rest, settings).is_err() {
                        return Err(NotBrivlaError(format!(
                            "{{{to_part}'y}} falls off because the y-hyphen setting is Standard"
                        )));
                    }
                    return Err(NotBrivlaError(format!(
                        "{{{valsi}}} is a tosmabru: {{{to_part}'y {rest}}}"
                    )));
                }
            }
        } else {
            let require_zihevla = require_cluster || !settings.exp_rafsi;
            let shape_type = check_zihevla_or_rafsi(
                part,
                &extract!(settings; y_hyphens, exp_rafsi, allow_mz),
                require_zihevla,
            );
            if let Err(e) = shape_type {
                match e {
                    NotZihevlaError(m) => return Err(NotBrivlaError(m)),
                    _ => return Err(e),
                }
            }
            let shape_type = shape_type.unwrap();
            if shape_type == Zihevla {
                has_cluster = true;
            }
            if is_consonant(strin!(part, 0)) || settings.glides && is_glide(part) {
                num_consonants += 1;
            }
            res_parts.push(part_.to_string());
        }
        consonant_before_break = false;
    }
    if !has_cluster && !is_cmetai {
        if settings.consonants == Cluster {
            return Err(NotBrivlaError(format!("{{{valsi}}} lacks a consonant cluster")));
        }
        if settings.consonants == TwoConsonants && num_consonants < 2
            || settings.consonants == OneConsonant && num_consonants < 1
        {
            return Err(NotBrivlaError(format!("{{{valsi}}} doesn't have enough consonants")));
        }
        if is_mahortai {
            return Err(NotBrivlaError(format!("{{{valsi}}} is just a cmavo or cmavo compound")));
        }
    }
    if !(is_vowel(strin!(&valsi, 0))
        && (is_consonant(strin!(&valsi, 1)) || strin!(&valsi, 1) == 'y'))
        && is_slinkuhi(&valsi, &extract!(settings; y_hyphens, allow_mz)).unwrap()
    {
        Err(NotBrivlaError(format!("{{{valsi}}} is a slinku'i")))
    } else {
        Ok((if is_cmetai { Cmevla } else { ExtendedLujvo }, res_parts))
    }
}

/// Gets the start/end positions of each rafsi.
pub fn get_rafsi_indices(rl: &[&str]) -> Vec<[usize; 2]> {
    let mut pos = 0;
    let mut indices = vec![];
    for r in rl {
        if !HYPHENS.contains(r) {
            indices.push([pos, pos + r.len()]);
        }
        pos += r.len();
    }
    indices
}
