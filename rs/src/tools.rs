use crate::{
    data::{
        BANNED_TRIPLES, FOLLOW_VOWEL_CLUSTERS, HYPHENS, INITIAL, MZ_VALID, START_VOWEL_CLUSTERS,
        VALID,
    },
    exceptions::Jvonunfli,
    katna::{jvokaha, jvokaha2},
    tarmi::{
        is_consonant, is_gismu, is_glide, is_valid_rafsi, is_vowel, is_zihevla_initial_cluster,
        is_zihevla_middle_cluster, rafsi_tarmi, split_vowel_cluster, strip_hyphens, BrivlaType,
        ConsonantSetting, Settings, Tarmi, YHyphenSetting,
    },
};
use itertools::Itertools;
use regex::Regex;

pub fn regex_replace_all(regex: &str, from: &str, with: &str) -> String {
    Regex::new(regex)
        .unwrap()
        .replace_all(from, with)
        .to_string()
}

pub fn char(s: &str, i: isize) -> char {
    if s.is_empty() {
        return char::default();
    }
    let i = (s.len() as isize + i) % s.len() as isize;
    s.chars().nth(i as usize).unwrap_or_default()
}
pub fn slice(s: &str, i: isize, j: isize) -> &str {
    let mut i = if i >= 0 { 0 } else { s.len() as isize } + i;
    let mut j = if j >= 0 { 0 } else { s.len() as isize } + j;
    i = i.clamp(0, s.len() as isize);
    j = j.clamp(0, s.len() as isize);
    &s[i as usize..j as usize]
}
pub fn slice_(s: &str, i: isize) -> &str {
    slice(s, i, s.len() as isize)
}

/// Convert word to standard form (*h* → *'*, no periods/commas, lowercase)
pub fn normalize(word: &str) -> String {
    regex_replace_all(r"^\.|,", &word.to_lowercase(), "").replace("h", "'")
}

/// True if `s` is a gismu or lujvo
// #[deprecated(since = "2.0.0", note = "relic from pre-zi'evlupdate (2.0.0)")]
pub fn is_gismu_or_lujvo(s: &str, settings: &Settings) -> Result<bool, Jvonunfli> {
    if s.len() < 5 || !is_vowel(char(s, -1)) {
        return Ok(false);
    }
    if is_gismu(s, settings) {
        return Ok(true);
    }
    if let Err(e) = jvokaha(s, settings) {
        match e {
            Jvonunfli::DecompositionError(_) | Jvonunfli::InvalidClusterError(_) => {
                return Ok(false)
            }
            _ => return Err(e),
        }
    }
    Ok(true)
}

/// True if `s` isn't a valid word because putting a CV cmavo in front of it makes it a lujvo (e.g.
/// *pa \*slinku'i*)
pub fn is_slinkuhi(s: &str, settings: &Settings) -> Result<bool, Jvonunfli> {
    if let Err(e) = jvokaha(&format!("pa{s}"), settings) {
        match e {
            Jvonunfli::DecompositionError(_) | Jvonunfli::InvalidClusterError(_) => Ok(false),
            _ => Err(e),
        }
    } else {
        Ok(true)
    }
}

/// Check rules specific to zi'evla or experimental rafsi. May return Zihevla when not given a valid
/// word
pub fn check_zihevla_or_rafsi(
    mut valsi: &str,
    settings: &Settings,
    require_zihevla: bool,
) -> Result<BrivlaType, Jvonunfli> {
    let valsi_ = valsi;
    if require_zihevla && valsi.len() < 4 {
        return Err(Jvonunfli::NotZihevlaError(format!(
            "too short to be zi'evla: {{{valsi}}}"
        )));
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
        if is_consonant(char(valsi, 0)) {
            while !valsi.is_empty() && is_consonant(char(valsi, 0)) {
                chunk += slice(valsi, 0, 1);
                valsi = slice_(valsi, 1);
            }

            if chunk.len() >= 2 && cluster_pos.is_none() {
                if num_consonants > 1 {
                    return Err(Jvonunfli::NotZihevlaError(format!(
                        "too many consonants before first cluster: {{{valsi_}}}"
                    )));
                }
                cluster_pos = Some(pos);
            }
            if num_syllables == 0 && chunk.len() >= 2 && !INITIAL.contains(&slice(&chunk, 0, 2)) {
                return Err(Jvonunfli::NotZihevlaError(format!(
                    "invalid word initial: {{{valsi_}}}"
                )));
            }
            for i in 0..chunk.len().saturating_sub(1) {
                let i = i as isize;
                let cluster = slice(&chunk, i, i + 2);
                if !if settings.allow_mz {
                    MZ_VALID.to_vec()
                } else {
                    VALID.to_vec()
                }
                .contains(&cluster)
                {
                    return Err(Jvonunfli::NotZihevlaError(format!(
                        "invalid cluster {{{cluster}}} in word {{{valsi_}}}"
                    )));
                }
            }
            for i in 0..chunk.len().saturating_sub(2) {
                let i = i as isize;
                let cluster = slice(&chunk, i, i + 3);
                if BANNED_TRIPLES.contains(&cluster) {
                    return Err(Jvonunfli::NotZihevlaError(format!(
                        "banned triple {{{cluster}}} in word {{{valsi_}}}"
                    )));
                }
            }
            if pos == 0 {
                if !is_zihevla_initial_cluster(&chunk) {
                    return Err(Jvonunfli::NotZihevlaError(format!(
                        "invalid zi'evla initial cluster {{{chunk}}} in {{{valsi_}}}"
                    )));
                }
            } else if !is_zihevla_middle_cluster(&chunk) {
                return Err(Jvonunfli::NotZihevlaError(format!(
                    "invalid zi'evla middle cluster {{{chunk}}} in {{{valsi_}}}"
                )));
            }
            final_consonant_pos = pos;
            num_consonants += chunk.len();
        } else if is_vowel(char(valsi, 0)) {
            while !valsi.is_empty() && is_vowel(char(valsi, 0)) {
                chunk += slice(valsi, 0, 1);
                valsi = slice_(valsi, 1);
            }
            if pos == 0 {
                if START_VOWEL_CLUSTERS.contains(&chunk.as_str())
                    || FOLLOW_VOWEL_CLUSTERS.contains(&chunk.as_str())
                {
                    num_syllables += 1;
                } else {
                    return Err(Jvonunfli::NotZihevlaError(format!(
                        "starts with bad vowels: {{{valsi_}}}"
                    )));
                }
            } else {
                if let Err(e) = split_vowel_cluster(&chunk) {
                    match e {
                        Jvonunfli::DecompositionError(_) => {
                            return Err(Jvonunfli::NotZihevlaError(format!(
                                "vowel decomposition error: {{{chunk}}} in {{{valsi_}}}"
                            )))
                        }
                        _ => return Err(e),
                    }
                }
                num_syllables += split_vowel_cluster(&chunk).unwrap().len();
            }
        } else if char(valsi, 0) == '\'' {
            chunk = "'".to_string();
            valsi = slice_(valsi, 1);
            if pos < 1 || !is_vowel(char(valsi_, pos - 1)) {
                return Err(Jvonunfli::NotZihevlaError(format!(
                    "' not preceded by vowel: {{{valsi_}}}"
                )));
            }
            if valsi.is_empty() || !is_vowel(char(valsi_, pos + 1)) {
                return Err(Jvonunfli::NotZihevlaError(format!(
                    "' not followed by vowel: {{{valsi_}}}"
                )));
            }
        } else {
            return Err(Jvonunfli::NotZihevlaError(format!(
                "unexpected character {{{}}} in {{{valsi_}}}",
                char(valsi, 0)
            )));
        }
        pos += chunk.len() as isize;
        chunk = String::new();
    }
    if num_syllables < 2 && (require_zihevla || !settings.exp_rafsi) {
        return Err(Jvonunfli::NotZihevlaError(format!(
            "too few syllables: {{{valsi_}}}"
        )));
    } else if num_syllables > 2 && cluster_pos.is_some() && cluster_pos > Some(0) {
        if is_brivla(slice_(valsi_, cluster_pos.unwrap()), settings)? {
            return Err(Jvonunfli::NotZihevlaError(format!(
                "falls apart at cluster: {{{}_{}}}",
                slice(valsi_, 0, cluster_pos.unwrap()),
                slice_(valsi_, cluster_pos.unwrap())
            )));
        }
        for i in 1..cluster_pos.unwrap() {
            if is_consonant(char(valsi_, cluster_pos.unwrap() - i))
                && is_brivla(slice_(valsi_, cluster_pos.unwrap() - i), settings)?
            {
                return Err(Jvonunfli::NotZihevlaError(format!(
                    "falls apart before cluster: {{{}_{}}}",
                    slice(valsi_, 0, cluster_pos.unwrap() - i),
                    slice_(valsi_, cluster_pos.unwrap() - i)
                )));
            }
        }
    }
    if cluster_pos.is_none() {
        if require_zihevla {
            return Err(Jvonunfli::NotZihevlaError(format!(
                "no cluster: {{{valsi_}}}"
            )));
        }
        if !is_consonant(char(valsi_, 0)) && !settings.exp_rafsi {
            return Err(Jvonunfli::NotZihevlaError(format!(
                "not valid rafsi shape: {{{valsi_}}}"
            )));
        }
        if num_consonants > 1 {
            return Err(Jvonunfli::NotZihevlaError(format!(
                "too many consonants without cluster: {{{valsi_}}}"
            )));
        }
        if final_consonant_pos > 0 {
            return Err(Jvonunfli::NotZihevlaError(format!(
                "non-initial consonant(s) without cluster: {{{valsi_}}}"
            )));
        }
    } else if !(is_vowel(char(valsi_, 0)) && is_consonant(char(valsi_, 1)))
        && is_slinkuhi(valsi_, settings)?
    {
        return Err(Jvonunfli::NotZihevlaError(format!(
            "slinku'i: {{pa,{valsi_}}}"
        )));
    }
    Ok(if cluster_pos.is_none() {
        BrivlaType::Rafsi
    } else {
        BrivlaType::Zihevla
    })
}

/// True if given a valid brivla
pub fn is_brivla(valsi: &str, settings: &Settings) -> Result<bool, Jvonunfli> {
    let b_type = analyze_brivla(valsi, settings);
    if let Err(e) = b_type {
        match e {
            Jvonunfli::NotBrivlaError(_) => return Ok(false),
            _ => return Err(e),
        }
    }
    Ok(b_type.unwrap().0 != BrivlaType::Cmevla)
}

/// Return type & decomposition of any brivla or decomposable cmevla. Doesn't check the cmevla
/// morphology rules
pub fn analyze_brivla(
    valsi: &str,
    settings: &Settings,
) -> Result<(BrivlaType, Vec<String>), Jvonunfli> {
    let valsi = normalize(valsi);
    let mut is_cmetai = false;
    if valsi.is_empty() {
        return Err(Jvonunfli::NotBrivlaError("empty string".to_string()));
    } else if is_consonant(char(&valsi, -1)) {
        is_cmetai = true;
    } else if !is_vowel(char(&valsi, -1)) {
        return Err(Jvonunfli::NotBrivlaError(format!(
            "doesn't end in consonant or vowel: {{{valsi}}}"
        )));
    }
    if is_cmetai {
        if is_gismu(&format!("{valsi}a"), settings) {
            return Err(Jvonunfli::NotBrivlaError(format!(
                "non-decomposable cmevla: {{{valsi}}}"
            )));
        }
    } else if is_gismu(&valsi, settings) {
        return Ok((BrivlaType::Gismu, vec![valsi]));
    }
    let res_parts = jvokaha(&valsi, settings);
    if let Err(ref e) = res_parts {
        match e {
            Jvonunfli::DecompositionError(_)
            | Jvonunfli::InvalidClusterError(_)
            | Jvonunfli::FakeTypeError(_) => (),
            _ => return Err(e.clone()),
        }
    } else {
        let res_parts = res_parts.unwrap();
        return Ok((
            if is_cmetai {
                BrivlaType::Cmevla
            } else {
                BrivlaType::Lujvo
            },
            res_parts,
        ));
    }
    if !is_vowel(char(&valsi, 0)) && !is_consonant(char(&valsi, 0)) {
        return Err(Jvonunfli::NotBrivlaError(format!(
            "doesn't start with vowel or consonant: {{{valsi}}}"
        )));
    }
    let y_parts = valsi.split('y').collect_vec();
    if y_parts.len() == 1 {
        if is_cmetai {
            return Err(Jvonunfli::NotBrivlaError(format!(
                "non-decomposable cmevla: {{{valsi}}}"
            )));
        }
        if let Err(e) = check_zihevla_or_rafsi(&valsi, settings, true) {
            match e {
                Jvonunfli::NotZihevlaError(_) => {
                    return Err(Jvonunfli::NotBrivlaError(format!(
                        "no hyphens, and not valid zi'evla: {{{valsi}}}"
                    )))
                }
                _ => return Err(e),
            }
        }
        return Ok((BrivlaType::Zihevla, vec![valsi]));
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
            return Err(Jvonunfli::NotBrivlaError(format!("double y: {{{valsi}}}")));
        }
        if char(part, 0) == '\'' {
            part = slice_(part, 1);
            part_ = part;
            next_hyphen += "'";
            if part.is_empty() {
                return Err(Jvonunfli::NotBrivlaError(format!(
                    "part consisting of only apostrophe in {{{valsi}}}"
                )));
            }
            if !is_vowel(char(part, 0)) || is_glide(part) {
                return Err(Jvonunfli::NotBrivlaError(format!(
                    "consonant or glide after ': {{{part_}}}"
                )));
            }
        } else if i > 0 && is_vowel(char(part, 0)) && !is_glide(part) {
            return Err(Jvonunfli::NotBrivlaError(format!(
                "non-glide vowel after y: {{{part}}}"
            )));
        }
        if !next_hyphen.is_empty() {
            res_parts.push(next_hyphen);
            next_hyphen = String::new();
        }
        if rafsi_tarmi(part) == Tarmi::Cvc {
            res_parts.push(part.to_string());
            consonant_before_break = true;
            num_consonants += 2;
            continue;
        }
        if rafsi_tarmi(&format!("{part}a")) == Tarmi::Ccv {
            return Err(Jvonunfli::NotBrivlaError(format!(
                "can't drop vowel on CCV rafsi: {{{part}}}"
            )));
        }
        if i > 0 && (is_consonant(char(part, 0)) || is_glide(part)) {
            is_mahortai = false;
        }
        if consonant_before_break
            && (is_consonant(char(part, 0)) || settings.glides && is_glide(part))
        {
            has_cluster = true;
        }
        let (mut can_be_rafsi, mut require_cluster, mut added_a) = (true, false, false);
        let parta = &format!("{part}a");
        if char(part, -1) == '\'' {
            if settings.y_hyphens == YHyphenSetting::Standard
                && !has_cluster
                && i < y_parts.len() - 1
                && char(y_parts[i + 1], 0) != '\''
            {
                require_cluster = true;
            }
            part = slice(part, 0, -1);
            part_ = part;
            next_hyphen += "'";
            if !is_vowel(char(part, -1)) {
                return Err(Jvonunfli::NotBrivlaError(format!(
                    "non-vowel before ': {{{part}}}"
                )));
            }
        } else if i < y_parts.len() - 1 || is_cmetai {
            if is_vowel(char(part, -1)) {
                can_be_rafsi = false;
            }
            part = parta;
            added_a = true;
            require_cluster = true;
        }
        let mut katnad = false;
        if can_be_rafsi {
            let mut found_parts = jvokaha2(part_, settings);
            if let Err(ref e) = found_parts {
                match e {
                    Jvonunfli::DecompositionError(_)
                    | Jvonunfli::InvalidClusterError(_)
                    | Jvonunfli::FakeTypeError(_) => found_parts = Ok(vec![part.to_string()]),
                    _ => return Err(e.clone()),
                }
            }
            let found_parts = found_parts.unwrap();
            if found_parts.len() < 2 && !is_valid_rafsi(&found_parts[0], settings) {
                return Err(Jvonunfli::NotBrivlaError(format!(
                    "invalid rafsi {{{}}}",
                    found_parts[0]
                )));
            }
            res_parts.extend(found_parts.clone());
            katnad = true;
            for fp in found_parts {
                let raftai = rafsi_tarmi(&fp);
                if [Tarmi::Cvv, Tarmi::Cvhv].contains(&raftai) {
                    num_consonants += 1
                } else if raftai != Tarmi::OtherRafsi {
                    num_consonants += 2;
                    has_cluster = true;
                }
            }
        }
        if katnad {
            if [Tarmi::Cvv, Tarmi::Cvhv].contains(&rafsi_tarmi(part))
                && require_cluster
                && !has_cluster
                && (settings.y_hyphens == YHyphenSetting::Standard
                    || !(i == y_parts.len() - 2
                        && [Tarmi::Cvv, Tarmi::Cvhv].contains(&rafsi_tarmi(y_parts[1]))))
            {
                return Err(Jvonunfli::NotBrivlaError(format!(
                    "falls off due to y: {{{part}}}"
                )));
            }
            if i == 0 {
                let mut smabru_part = "";
                if rafsi_tarmi(slice(part, 0, 4)) == Tarmi::Cvhv {
                    smabru_part = slice_(part, 4);
                } else if rafsi_tarmi(slice(part, 0, 3)) == Tarmi::Cvv {
                    smabru_part = slice_(part, 3);
                } else if is_consonant(char(part, 0)) && is_vowel(char(part, 1)) {
                    smabru_part = slice_(part, 2);
                }
                if !smabru_part.is_empty() {
                    let hyphenless = strip_hyphens(smabru_part);
                    if added_a {
                        smabru_part = slice(smabru_part, 0, -1);
                    } else {
                        smabru_part = &hyphenless;
                    }
                    if is_valid_rafsi(smabru_part, settings) {
                        return Err(Jvonunfli::NotBrivlaError(format!("tosmabru: {{{part}}}")));
                    }
                    if let Err(e) = jvokaha(smabru_part, settings) {
                        match e {
                            Jvonunfli::DecompositionError(_)
                            | Jvonunfli::InvalidClusterError(_) => (),
                            _ => return Err(e),
                        }
                    } else {
                        return Err(Jvonunfli::NotBrivlaError(format!("tosmabru: {{{part}}}")));
                    }
                }
            }
        } else {
            let require_zihevla = require_cluster || !settings.exp_rafsi;
            let shape_type = check_zihevla_or_rafsi(part, settings, require_zihevla);
            if let Err(e) = shape_type {
                match e {
                    Jvonunfli::NotZihevlaError(m) => return Err(Jvonunfli::NotBrivlaError(m)),
                    _ => return Err(e),
                }
            }
            let shape_type = shape_type.unwrap();
            if shape_type == BrivlaType::Zihevla {
                has_cluster = true;
            }
            if is_consonant(char(part, 0)) || settings.glides && is_glide(part) {
                num_consonants += 1;
            }
            res_parts.push(part_.to_string());
        }
        consonant_before_break = false;
    }
    if !has_cluster {
        if settings.consonants == ConsonantSetting::Cluster {
            return Err(Jvonunfli::NotBrivlaError(format!(
                "no clusters: {{{valsi}}}"
            )));
        } else if settings.consonants == ConsonantSetting::TwoConsonants && num_consonants < 2
            || settings.consonants == ConsonantSetting::OneConsonant && num_consonants < 1
        {
            return Err(Jvonunfli::NotBrivlaError(format!(
                "not enough consonants: {{{valsi}}}"
            )));
        } else if is_mahortai {
            return Err(Jvonunfli::NotBrivlaError(format!(
                "cmavo or cmavo-compound shaped: {{{valsi}}}"
            )));
        }
    }
    if !(is_vowel(char(&valsi, 0)) && (is_consonant(char(&valsi, 1)) || char(&valsi, 1) == 'y'))
        && is_slinkuhi(&valsi, settings).unwrap()
    {
        return Err(Jvonunfli::NotBrivlaError(format!(
            "slinku'i: {{pa,{valsi}}}"
        )));
    }
    Ok((
        if is_cmetai {
            BrivlaType::Cmevla
        } else {
            BrivlaType::ExtendedLujvo
        },
        res_parts,
    ))
}

/// Get the start/end positions of each rafsi
pub fn get_rafsi_indices(rl: Vec<&str>) -> Vec<[usize; 2]> {
    let mut pos = 0;
    let mut indices = vec![];
    rl.iter().for_each(|r| {
        if !HYPHENS.contains(r) {
            indices.push([pos, pos + r.len()])
        }
        pos += r.len();
    });
    indices
}