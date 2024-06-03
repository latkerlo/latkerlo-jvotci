use std::collections::HashMap;

use crate::{
    data::{BANNED_TRIPLES, INITIAL, MZ_VALID, VALID},
    exceptions::Jvonunfli,
    katna::jvokaha2,
    rafsi::RAFSI,
    tarmi::{
        contains_consonant, is_consonant, is_glide, is_only_lojban_characters, is_valid_rafsi,
        is_vowel, rafsi_tarmi, strip_hyphens, tarmi_ignoring_hyphen, BrivlaType, ConsonantSetting,
        Settings, Tarmi, YHyphenSetting,
    },
    tools::{
        analyze_brivla, char, check_zihevla_or_rafsi, normalize, regex_replace_all, slice, slice_,
    },
};
use itertools::Itertools;

#[derive(PartialEq, Clone, Copy)]
pub enum Tosytype {
    Tosynone,
    Tosmabru,
    Tosyhuhu,
}

/// Calculate the score for a rafsi (possibly including a hyphen)
pub fn score(r: &str) -> i32 {
    let t = match tarmi_ignoring_hyphen(r) {
        Tarmi::OtherRafsi => 0,
        t => t as usize,
    };
    (1000 * r.len() - 400 * r.matches('\'').count() + 100 * r.matches('y').count()
        - 10 * t
        - r.chars().filter(|c| "aeiou".contains(*c)).count())
    .try_into()
    .unwrap()
}

/// Clean the tanru
pub fn process_tanru(tanru: &str) -> Vec<String> {
    // split_whitespace trims for us :3
    tanru.split_whitespace().map(normalize).collect_vec()
}

/// Find possible rafsi-hyphen combinations
pub fn get_rafsi_for_rafsi(
    r: &str,
    r_type: &str,
    first: bool,
    last: bool,
    settings: &Settings,
) -> Vec<(String, i32)> {
    let mut res = vec![];
    let r = if !first && is_vowel(char(r, 0)) && !is_glide(r) {
        format!("'{r}")
    } else {
        r.to_string()
    };
    if [
        "ShortBrivla",
        &Tarmi::Ccvc.to_string(),
        &Tarmi::Cvcc.to_string(),
    ]
    .contains(&r_type)
    {
        if !last {
            res.push((format!("{r}y"), 2));
        } else if !is_vowel(char(&r, -1)) {
            res.push((r, 2));
        }
    } else if [
        "LongBrivla",
        &Tarmi::Ccvcv.to_string(),
        &Tarmi::Cvccv.to_string(),
    ]
    .contains(&r_type)
    {
        if last {
            res.push((r, 2))
        } else if !(r_type == Tarmi::Cvccv.to_string() && INITIAL.contains(&slice(&r, 2, 4))) {
            res.push((format!("{r}'y"), 2))
        }
    } else if r_type == "ExperimentalRafsi" {
        let num_consonants = (settings.consonants != ConsonantSetting::Cluster
            && (is_consonant(char(&r, 0)) || settings.glides && is_glide(&r)))
            as i32;
        if last {
            res.push((r, num_consonants));
        } else if !first {
            res.push((format!("{r}'y"), num_consonants));
        } else {
            res.push((format!("{r}'"), num_consonants));
        }
    } else if [
        Tarmi::Cvv.to_string().as_str(),
        Tarmi::Cvhv.to_string().as_str(),
    ]
    .contains(&r_type)
    {
        let num_consonants = (settings.consonants != ConsonantSetting::Cluster) as i32;
        if first {
            res.push((format!("{r}'"), num_consonants));
        } else if !last {
            res.push((format!("{r}'y"), num_consonants));
        }
        res.push((r, num_consonants));
    } else if r_type == Tarmi::Ccv.to_string() {
        res.push((r.clone(), 2));
        res.push((format!("{r}'y"), 2));
    } else if r_type == Tarmi::Cvc.to_string() {
        res.push((r.clone(), 2));
        if !last {
            res.push((format!("{r}y"), 2));
        }
    } else {
        panic!("unrecognized rafsi type {r_type}");
    }
    res
}

/// Get the rafsi list for each word
pub fn get_rafsi_list_list(
    valsi_list: Vec<String>,
    settings: &Settings,
) -> Result<Vec<Vec<(String, i32)>>, Jvonunfli> {
    let mut rafsi_list_list = vec![];
    for (i, mut valsi) in valsi_list.iter().enumerate() {
        let mut rafsi_list = vec![];
        let first = i == 0;
        let last = i == valsi_list.len() - 1;
        let hyphenless = regex_replace_all("^-+|-+$", valsi, "");
        if char(valsi, -1) == '-' {
            let is_short_brivla = char(valsi, 0) != '-';
            valsi = &hyphenless;
            if !is_only_lojban_characters(valsi) {
                return Err(Jvonunfli::NonLojbanCharacterError(format!(
                    "non-lojban character in {{{valsi}}}"
                )));
            }
            if char(valsi, -1) == '\'' {
                return Err(Jvonunfli::NonLojbanCharacterError(format!(
                    "rafsi cannot end with ': {{{valsi}}}"
                )));
            }
            if is_short_brivla {
                let b_type = analyze_brivla(&format!("{valsi}a"), settings);
                if let Err(e) = b_type {
                    match e {
                        Jvonunfli::NoLujvoFoundError(_) => {
                            return Err(Jvonunfli::NoLujvoFoundError(format!(
                                "rafsi + a is not a brivla: {{{valsi}}}"
                            )))
                        }
                        _ => return Err(e),
                    }
                }
                let b_type = b_type.unwrap().0;
                if ![BrivlaType::Zihevla, BrivlaType::Gismu].contains(&b_type) {
                    return Err(Jvonunfli::NoLujvoFoundError(format!(
                        "rafsi + a is not a gismu or zi'evla: {{{valsi}}}"
                    )));
                }
                if valsi.len() > 5 && is_consonant(char(valsi, -1)) {
                    let mut decomposes = true;
                    if let Err(e) = jvokaha2(valsi, settings) {
                        match e {
                            Jvonunfli::DecompositionError(_)
                            | Jvonunfli::InvalidClusterError(_) => {
                                decomposes = false;
                            }
                            _ => return Err(e),
                        }
                    }
                    if decomposes {
                        return Err(Jvonunfli::NoLujvoFoundError(format!(
                            "short zi'evla rafsi falls apart: {{{valsi}}}"
                        )));
                    }
                }
                rafsi_list.extend(get_rafsi_for_rafsi(
                    valsi,
                    "ShortBrivla",
                    first,
                    last,
                    settings,
                ));
            } else {
                let raftai = rafsi_tarmi(valsi);
                if raftai == Tarmi::OtherRafsi {
                    let mut zihevla_or_rafsi = None;
                    let b_type = analyze_brivla(valsi, settings);
                    if let Err(e) = b_type {
                        match e {
                            Jvonunfli::NotBrivlaError(_) => {
                                if settings.exp_rafsi {
                                    let shape = check_zihevla_or_rafsi(valsi, settings, false);
                                    if let Err(e) = shape {
                                        match e {
                                            Jvonunfli::NotZihevlaError(_) => {
                                                return Err(Jvonunfli::NoLujvoFoundError(format!(
                                                    "not a valid rafsi shape: {{{valsi}}}"
                                                )))
                                            }
                                            _ => return Err(e),
                                        }
                                    }
                                    let shape = shape.unwrap();
                                    if shape == BrivlaType::Rafsi {
                                        zihevla_or_rafsi = Some(BrivlaType::Rafsi);
                                    }
                                }
                            }
                            _ => return Err(e),
                        }
                    } else {
                        let b_type = b_type.unwrap().0;
                        if b_type == BrivlaType::Zihevla {
                            zihevla_or_rafsi = Some(BrivlaType::Zihevla);
                        }
                    }
                    if zihevla_or_rafsi.is_none() {
                        return Err(Jvonunfli::NotZihevlaError(format!(
                            "not a valid rafsi or zi'evla shape: {{{valsi}}}"
                        )));
                    }
                    let r_type = if zihevla_or_rafsi == Some(BrivlaType::Zihevla) {
                        "LongBrivla"
                    } else {
                        "ExperimentalRafsi"
                    };
                    rafsi_list.extend(get_rafsi_for_rafsi(valsi, r_type, first, last, settings));
                } else {
                    if !is_valid_rafsi(valsi, settings) {
                        return Err(Jvonunfli::InvalidClusterError(format!(
                            "invalid cluster in rafsi: {{{valsi}}}"
                        )));
                    }
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        valsi,
                        &raftai.to_string(),
                        first,
                        last,
                        settings,
                    ));
                }
            }
        } else {
            if !is_only_lojban_characters(valsi) {
                return Err(Jvonunfli::NonLojbanCharacterError(format!(
                    "non-lojban character in {{{valsi}}}"
                )));
            }
            let short_rafsi_list = RAFSI.get(valsi.as_str());
            if let Some(srl) = short_rafsi_list {
                srl.iter().for_each(|r| {
                    let raftai = rafsi_tarmi(r);
                    if raftai == Tarmi::OtherRafsi && settings.exp_rafsi {
                        return;
                    }
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        r,
                        &raftai.to_string(),
                        first,
                        last,
                        settings,
                    ))
                })
            }
            let b_type = analyze_brivla(valsi, settings);
            if let Err(e) = b_type {
                match e {
                    Jvonunfli::NotBrivlaError(_) => {}
                    _ => return Err(e),
                }
            } else {
                let b_type = b_type.unwrap().0;

                if b_type == BrivlaType::Gismu {
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        slice(valsi, 0, -1),
                        "ShortBrivla",
                        first,
                        last,
                        settings,
                    ));
                }
                if [BrivlaType::Gismu, BrivlaType::Zihevla].contains(&b_type) {
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        valsi,
                        "LongBrivla",
                        first,
                        last,
                        settings,
                    ));
                }
            }
        }
        rafsi_list_list.push(rafsi_list);
    }
    Ok(rafsi_list_list)
}
/// `get_rafsi_list_list` but shorter to write manually
pub fn grill(vl: &str, settings: &Settings) -> Result<Vec<Vec<(String, i32)>>, Jvonunfli> {
    get_rafsi_list_list(process_tanru(vl), settings)
}

/// Try to add a rafsi to a lujvo and calculate the score
#[allow(clippy::too_many_arguments, clippy::type_complexity)] // sorry!
pub fn combine(
    lujvo: &str,
    rafsi: &str,
    lujvo_c: i32,
    rafsi_c: i32,
    lujvo_score: i32,
    indices: Vec<[usize; 2]>,
    mut tosmabru_type: Tosytype,
    tanru_len: usize,
    settings: &Settings,
) -> Option<(Tosytype, i32, i32, String, Vec<[usize; 2]>)> {
    let lujvo_f = char(lujvo, -1);
    let rafsi_i = char(rafsi, 0);
    if is_consonant(lujvo_f)
        && is_consonant(rafsi_i)
        && !if settings.allow_mz {
            MZ_VALID.to_vec()
        } else {
            VALID.to_vec()
        }
        .contains(&format!("{lujvo_f}{rafsi_i}").as_str())
        || BANNED_TRIPLES.contains(&format!("{lujvo_f}{}", slice(rafsi, 0, 2)).as_str())
    {
        return None;
    }
    let raftai1 = tarmi_ignoring_hyphen(rafsi);
    if !"y'".contains(lujvo_f) && raftai1 == Tarmi::OtherRafsi {
        return None;
    }
    let mut hyphen = "";
    if lujvo_f == '\'' {
        if rafsi_i == '\'' || settings.y_hyphens != YHyphenSetting::Standard {
            hyphen = "y";
        } else {
            return None;
        }
    } else if lujvo.len() <= 5 && !settings.generate_cmevla {
        let raftai0 = tarmi_ignoring_hyphen(lujvo);
        if [Tarmi::Cvhv, Tarmi::Cvv].contains(&raftai0) {
            hyphen = if settings.y_hyphens == YHyphenSetting::ForceY {
                "'y"
            } else if rafsi_i == 'r' {
                "n"
            } else {
                "r"
            };
        }
        if tanru_len == 2 && raftai1 == Tarmi::Ccv {
            hyphen = "";
        }
    }
    if tosmabru_type == Tosytype::Tosmabru {
        if !INITIAL.contains(&format!("{lujvo_f}{rafsi_i}").as_str()) {
            tosmabru_type = Tosytype::Tosynone;
        } else if raftai1 == Tarmi::Cvccv {
            if INITIAL.contains(&slice(rafsi, 2, 4)) {
                return None;
            }
            tosmabru_type = Tosytype::Tosynone;
        } else if raftai1 == Tarmi::Cvc {
            if char(rafsi, -1) == 'y' {
                return None;
            }
        } else {
            tosmabru_type = Tosytype::Tosynone;
        }
    } else if tosmabru_type == Tosytype::Tosyhuhu && (rafsi_i != '\'' || contains_consonant(rafsi))
    {
        tosmabru_type = Tosytype::Tosynone;
    }
    let rafsi_start = lujvo.len() + hyphen.len() + (char(rafsi, 0) == '\'') as usize;
    let rafsi_end = rafsi_start + strip_hyphens(rafsi).len();
    let indices = indices
        .iter()
        .chain(&[[rafsi_start, rafsi_end]])
        .cloned()
        .collect_vec();
    let mut new_c = rafsi_c;
    if !hyphen.is_empty() && "nr".contains(hyphen) {
        new_c = 2;
    } else if settings.consonants == ConsonantSetting::Cluster && rafsi_c != 2 {
        let mut i = lujvo.len() as isize - 1;
        while "'y".contains(char(lujvo, i)) {
            i -= 1;
        }
        let mut j = 0;
        while char(rafsi, j) == '\'' {
            j += 1;
        }
        new_c = (is_consonant(char(lujvo, i))
            && (is_consonant(char(rafsi, j)) || settings.glides && is_glide(slice_(rafsi, j))))
            as i32
            * 2;
    }
    let mut total_c = 2.min(lujvo_c + new_c);
    if settings.consonants == ConsonantSetting::OneConsonant && total_c > 0 {
        total_c = 2;
    }
    let hyphen_score = if hyphen == "'y" {
        1700
    } else {
        1100 * hyphen.len() as i32
    };
    Some((
        tosmabru_type,
        total_c,
        lujvo_score + hyphen_score + score(rafsi),
        format!("{lujvo}{hyphen}{rafsi}"),
        indices,
    ))
}

type BestLujvoMap = HashMap<String, (String, i32, Vec<[usize; 2]>)>;

/// Add a candidate to current_best
#[allow(clippy::type_complexity)]
pub fn update_current_best(
    candidate: Option<(Tosytype, i32, i32, String, Vec<[usize; 2]>)>,
    mut current_best: [[BestLujvoMap; 3]; 3],
) -> [[BestLujvoMap; 3]; 3] {
    if candidate.is_none() {
        return current_best;
    }
    let (tosmabru_type, num_consonants, res_score, res_lujvo, res_indices) = candidate.unwrap();
    let lujvo_f = char(&res_lujvo, -1);
    if !current_best[tosmabru_type as usize][num_consonants as usize]
        .contains_key(&format!("{lujvo_f}"))
        || current_best[tosmabru_type as usize][num_consonants as usize]
            .get(&format!("{lujvo_f}"))
            .unwrap()
            .1
            > res_score
    {
        current_best[tosmabru_type as usize][num_consonants as usize]
            .insert(format!("{lujvo_f}"), (res_lujvo, res_score, res_indices));
    }
    current_best
}

/// Create the best lujvo for the tanru (list). Recommended to use `get_lujvo_with_analytics`
/// instead if you have a string
pub fn get_lujvo_from_list(
    valsi_list: Vec<String>,
    settings: &Settings,
) -> Result<(String, i32, Vec<[usize; 2]>), Jvonunfli> {
    let rafsi_list_list = get_rafsi_list_list(valsi_list.clone(), settings);
    let mut current_best = [
        [
            BestLujvoMap::new(),
            BestLujvoMap::new(),
            BestLujvoMap::new(),
        ],
        [
            BestLujvoMap::new(),
            BestLujvoMap::new(),
            BestLujvoMap::new(),
        ],
        [
            BestLujvoMap::new(),
            BestLujvoMap::new(),
            BestLujvoMap::new(),
        ],
    ];
    let rafsi_list_list = rafsi_list_list?;
    if rafsi_list_list.len() < 2 {
        return Err(Jvonunfli::FakeTypeError(format!(
            "rafsi_list_list is too short: {rafsi_list_list:?}"
        )));
    }
    for rafsi0 in &rafsi_list_list[0] {
        for rafsi1 in &rafsi_list_list[1] {
            let mut tosmabru_type = Tosytype::Tosynone;
            if tarmi_ignoring_hyphen(&rafsi0.0) == Tarmi::Cvc && !settings.generate_cmevla {
                tosmabru_type = if char(&rafsi0.0, -1) == 'y' {
                    Tosytype::Tosyhuhu
                } else {
                    Tosytype::Tosmabru
                };
            }
            let res = combine(
                &rafsi0.0,
                &rafsi1.0,
                rafsi0.1,
                rafsi1.1,
                score(&rafsi0.0),
                vec![[0, strip_hyphens(&rafsi0.0).len()]],
                tosmabru_type,
                rafsi_list_list.len(),
                settings,
            );
            current_best = update_current_best(res, current_best);
        }
    }
    let mut previous_best = current_best;
    for rafsi_list in rafsi_list_list.iter().skip(2) {
        current_best = [
            [
                BestLujvoMap::new(),
                BestLujvoMap::new(),
                BestLujvoMap::new(),
            ],
            [
                BestLujvoMap::new(),
                BestLujvoMap::new(),
                BestLujvoMap::new(),
            ],
            [
                BestLujvoMap::new(),
                BestLujvoMap::new(),
                BestLujvoMap::new(),
            ],
        ];
        for rafsi in rafsi_list {
            for tosmabru_type in [Tosytype::Tosynone, Tosytype::Tosmabru, Tosytype::Tosyhuhu] {
                for num_consonants in 0..3 {
                    for (_, lujvo_and_score) in
                        previous_best[tosmabru_type as usize][num_consonants].clone()
                    {
                        let res = combine(
                            &lujvo_and_score.0,
                            &rafsi.0,
                            num_consonants as i32,
                            rafsi.1,
                            lujvo_and_score.1,
                            lujvo_and_score.2,
                            tosmabru_type,
                            rafsi_list_list.len(),
                            settings,
                        );
                        current_best = update_current_best(res, current_best);
                    }
                }
            }
        }
        previous_best = current_best;
    }
    let (mut best_lujvo, mut best_score, mut best_indices) = (String::new(), i32::MAX, vec![]);
    for (c, lujvo_and_score) in &previous_best[0][2] {
        if (is_vowel(char(c, 0)) && !settings.generate_cmevla
            || is_consonant(char(c, 0)) && settings.generate_cmevla)
            && lujvo_and_score.1 < best_score
        {
            (best_lujvo, best_score, best_indices) = lujvo_and_score.clone();
        }
    }
    if best_lujvo.is_empty() {
        return Err(Jvonunfli::NoLujvoFoundError(format!(
            "no lujvo found for {{{}}}",
            valsi_list.join(" ")
        )));
    }
    Ok((best_lujvo, best_score, best_indices))
}

/// Create the best lujvo for the tanru (string)
pub fn get_lujvo_with_analytics(
    tanru: &str,
    settings: &Settings,
) -> Result<(String, i32, Vec<[usize; 2]>), Jvonunfli> {
    get_lujvo_from_list(process_tanru(tanru), settings)
}
/// Create the best lujvo for the tanru (string). Doesns't output the score
pub fn get_lujvo(tanru: &str, settings: &Settings) -> Result<String, Jvonunfli> {
    Ok(get_lujvo_with_analytics(tanru, settings)?.0)
}
