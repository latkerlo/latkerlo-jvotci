use crate::{
    data::{BANNED_TRIPLES, INITIAL, MZ_VALID, VALID},
    exceptions::Jvonunfli,
    extract,
    katna::jvokaha2,
    rafsi::RAFSI,
    strin, strsl,
    tarmi::{
        contains_consonant, is_consonant, is_glide, is_only_lojban_characters, is_valid_rafsi,
        is_vowel, rafsi_tarmi, strip_hyphens, tarmi_ignoring_hyphen, BrivlaType, ConsonantSetting,
        Settings, Tarmi, YHyphenSetting,
    },
    tools::{analyze_brivla, check_zihevla_or_rafsi, normalize, regex_replace_all},
};
use indexmap::IndexMap;
use itertools::Itertools as _;
use regex::Regex;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tosytype {
    Tosynone,
    Tosmabru,
    Tosyhuhu,
}

/// Calculate the score for a rafsi (possibly including a hyphen). Use
/// [`score_lujvo`][`crate::score_lujvo`] to find the score of a lujvo
pub fn score(r: &str) -> i32 {
    let t = tarmi_ignoring_hyphen(r) as usize % 9;
    (1000 * r.len() - 400 * r.matches('\'').count() + 100 * r.matches('y').count()
        - 10 * t
        - r.chars().filter(|c| "aeiou".contains(*c)).count()) as _
}
pub(crate) fn tiebreak(lujvo: &str) -> i32 {
    (rafsi_tarmi(strsl!(lujvo, 0..3)) == Tarmi::Cvv
        && [Tarmi::Ccv, Tarmi::Ccvc, Tarmi::Cvc, Tarmi::Cvcc]
            .contains(&rafsi_tarmi(strsl!(lujvo, 3..)))) as i32
}

/// Clean the tanru
pub fn process_tanru(tanru: &str) -> Vec<String> {
    // split_whitespace trims for us :3
    tanru.split_whitespace().map(normalize).collect_vec()
}

/// Find possible rafsi-hyphen combinations
/// # Errors
/// if given a bad `r_type`
pub fn get_rafsi_for_rafsi(
    r: &str,
    r_type: &str,
    first: bool,
    last: bool,
    settings: &Settings,
) -> Result<Vec<(String, i32)>, Jvonunfli> {
    let mut res = vec![];
    let r = if !first && is_vowel(strin!(r, 0)) && !is_glide(r) {
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
        } else if !is_vowel(strin!(&r, -1)) {
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
            res.push((r, 2));
        } else {
            res.push((format!("{r}'y"), 2));
        }
    } else if r_type == "ExperimentalRafsi" {
        let num_consonants = (settings.consonants != ConsonantSetting::Cluster
            && (is_consonant(strin!(&r, 0)) || settings.glides && is_glide(&r)))
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
        // fake FakeTypeError lol
        return Err(Jvonunfli::FakeTypeError(format!(
            "unrecognized rafsi type {r_type}"
        )));
    }
    Ok(res)
}

#[allow(clippy::missing_panics_doc)] // .unwrap()
/// Get the rafsi list for each word
/// # Errors
/// if any word is an invalid brivla
pub fn get_rafsi_list_list(
    valsi_list: &[String],
    settings: &Settings,
) -> Result<Vec<Vec<(String, i32)>>, Jvonunfli> {
    let mut rafsi_list_list = vec![];
    let hyphens = Regex::new("^-+|-+$").unwrap();
    for (i, mut valsi) in valsi_list.iter().enumerate() {
        let mut rafsi_list = vec![];
        let first = i == 0;
        let last = i == valsi_list.len() - 1;
        let hyphenless = regex_replace_all(&hyphens, valsi, "");
        if strin!(valsi, -1) == '-' {
            let is_short_brivla = strin!(valsi, 0) != '-';
            valsi = &hyphenless;
            if !is_only_lojban_characters(valsi) {
                return Err(Jvonunfli::NonLojbanCharacterError(format!(
                    "{{{valsi}}} contains a non-lojban character"
                )));
            }
            if strin!(valsi, -1) == '\'' {
                return Err(Jvonunfli::NonLojbanCharacterError(format!(
                    "{{{valsi}}} ends in an apostrophe"
                )));
            }
            if is_short_brivla {
                let b_type = analyze_brivla(
                    &format!("{valsi}a"),
                    &extract!(settings, y_hyphens, exp_rafsi, allow_mz),
                );
                if let Err(e) = b_type {
                    match e {
                        Jvonunfli::NotBrivlaError(_) => {
                            return Err(Jvonunfli::NoLujvoFoundError(format!(
                                "{{{valsi}a}} is not a brivla"
                            )))
                        }
                        _ => return Err(e),
                    }
                }
                let b_type = b_type.unwrap().0;
                if ![BrivlaType::Zihevla, BrivlaType::Gismu].contains(&b_type) {
                    return Err(Jvonunfli::NoLujvoFoundError(format!(
                        "{{{valsi}a}} is not a gismu or zi'evla"
                    )));
                }
                if valsi.len() > 5 && is_consonant(strin!(valsi, -1)) {
                    let mut decomposes = true;
                    if let Err(e) = jvokaha2(valsi, &extract!(settings, y_hyphens, allow_mz)) {
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
                            "{{{valsi}a}} is a valid zi'evla, but without the final vowel it is a \
                             cmejvo"
                        )));
                    }
                }
                rafsi_list.extend(get_rafsi_for_rafsi(
                    valsi,
                    "ShortBrivla",
                    first,
                    last,
                    &extract!(settings, consonants, glides),
                )?);
            } else {
                let raftai = rafsi_tarmi(valsi);
                if raftai == Tarmi::OtherRafsi {
                    let mut zihevla_or_rafsi = None;
                    let b_type =
                        analyze_brivla(valsi, &extract!(settings, y_hyphens, exp_rafsi, allow_mz));
                    if let Err(e) = b_type {
                        match e {
                            Jvonunfli::NotBrivlaError(_) => {
                                if settings.exp_rafsi {
                                    let shape = check_zihevla_or_rafsi(
                                        valsi,
                                        &extract!(settings, y_hyphens, exp_rafsi, allow_mz),
                                        false,
                                    );
                                    if let Err(e) = shape {
                                        match e {
                                            Jvonunfli::NotZihevlaError(m) => {
                                                return Err(Jvonunfli::NoLujvoFoundError(m))
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
                            "{{{valsi}}} is an invalid rafsi or zi'evla"
                        )));
                    }
                    let r_type = if zihevla_or_rafsi == Some(BrivlaType::Zihevla) {
                        "LongBrivla"
                    } else {
                        "ExperimentalRafsi"
                    };
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        valsi,
                        r_type,
                        first,
                        last,
                        &extract!(settings, consonants, glides),
                    )?);
                } else {
                    if !is_valid_rafsi(valsi, &extract!(settings, allow_mz)) {
                        return Err(Jvonunfli::InvalidClusterError(format!(
                            "{{{valsi}}} contains an invalid cluster"
                        )));
                    }
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        valsi,
                        &raftai.to_string(),
                        first,
                        last,
                        &extract!(settings, consonants, glides),
                    )?);
                }
            }
        } else {
            if !is_only_lojban_characters(valsi) {
                return Err(Jvonunfli::NonLojbanCharacterError(format!(
                    "{{{valsi}}} contains a non-lojban character"
                )));
            }
            let short_rafsi_list = RAFSI.get(valsi.as_str());
            if let Some(srl) = short_rafsi_list {
                for r in srl {
                    let raftai = rafsi_tarmi(r);
                    if raftai == Tarmi::OtherRafsi && !settings.exp_rafsi {
                        continue;
                    }
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        r,
                        &raftai.to_string(),
                        first,
                        last,
                        &extract!(settings, consonants, glides),
                    )?);
                }
            }
            let b_type = analyze_brivla(valsi, &extract!(settings, y_hyphens, exp_rafsi, allow_mz));
            if let Err(e) = b_type {
                match e {
                    Jvonunfli::NotBrivlaError(_) => {}
                    _ => return Err(e),
                }
            } else {
                let b_type = b_type.unwrap().0;
                if b_type == BrivlaType::Gismu {
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        strsl!(valsi, 0..-1),
                        "ShortBrivla",
                        first,
                        last,
                        &extract!(settings, consonants, glides),
                    )?);
                }
                if [BrivlaType::Gismu, BrivlaType::Zihevla].contains(&b_type) {
                    rafsi_list.extend(get_rafsi_for_rafsi(
                        valsi,
                        "LongBrivla",
                        first,
                        last,
                        &extract!(settings, consonants, glides),
                    )?);
                }
            }
        }
        rafsi_list_list.push(rafsi_list);
    }
    Ok(rafsi_list_list)
}
/// = [`get_rafsi_list_list`] but shorter to write manually
#[allow(clippy::missing_errors_doc)]
pub fn grll(vl: &str, settings: &Settings) -> Result<Vec<Vec<(String, i32)>>, Jvonunfli> {
    get_rafsi_list_list(&process_tanru(vl), settings)
}

/// Try to add a rafsi to a lujvo and calculate the score
#[allow(clippy::too_many_arguments, clippy::type_complexity)] // sorry!
pub fn combine(
    lujvo: &str,
    rafsi: &str,
    lujvo_c: i32,
    rafsi_c: i32,
    lujvo_score: i32,
    indices: &[[usize; 2]],
    mut tosmabru_type: Tosytype,
    tanru_len: usize,
    settings: &Settings,
) -> Option<(Tosytype, i32, i32, String, Vec<[usize; 2]>)> {
    let lujvo_f = strin!(lujvo, -1);
    let rafsi_i = strin!(rafsi, 0);
    if is_consonant(lujvo_f)
        && is_consonant(rafsi_i)
        && !if settings.allow_mz { &MZ_VALID } else { &VALID }
            .contains(&format!("{lujvo_f}{rafsi_i}").as_str())
        || BANNED_TRIPLES.contains(&format!("{lujvo_f}{}", strsl!(rafsi, 0..2)).as_str())
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
    } else if lujvo.len() == 5
        && rafsi_tarmi(strsl!(lujvo, 0..3)) == Tarmi::Ccv
        && strsl!(lujvo, 3..) == "'y"
    {
        return None;
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
            if INITIAL.contains(strsl!(rafsi, 2..4)) {
                return None;
            }
            tosmabru_type = Tosytype::Tosynone;
        } else if raftai1 == Tarmi::Cvc {
            if strin!(rafsi, -1) == 'y' {
                return None;
            }
        } else {
            tosmabru_type = Tosytype::Tosynone;
        }
    } else if tosmabru_type == Tosytype::Tosyhuhu && (rafsi_i != '\'' || contains_consonant(rafsi))
    {
        tosmabru_type = Tosytype::Tosynone;
    }
    let rafsi_start = lujvo.len() + hyphen.len() + (strin!(rafsi, 0) == '\'') as usize;
    let rafsi_end = rafsi_start + strip_hyphens(rafsi).len();
    let indices = indices
        .iter()
        .chain(&[[rafsi_start, rafsi_end]])
        .copied()
        .collect_vec();
    let mut new_c = rafsi_c;
    if !hyphen.is_empty() && "nr".contains(hyphen) {
        new_c = 2;
    } else if settings.consonants == ConsonantSetting::Cluster && rafsi_c != 2 {
        let mut i = lujvo.len() as isize - 1;
        while "'y".contains(strin!(lujvo, i)) {
            i -= 1;
        }
        let mut j = 0;
        while strin!(rafsi, j) == '\'' {
            j += 1;
        }
        new_c = (is_consonant(strin!(lujvo, i))
            && (is_consonant(strin!(rafsi, j)) || settings.glides && is_glide(strsl!(rafsi, j..))))
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
    let res = format!("{lujvo}{hyphen}{rafsi}");
    let score = lujvo_score + hyphen_score + score(rafsi) - tiebreak(&res);
    Some((tosmabru_type, total_c, score, res, indices))
}

type BestLujvoMap = IndexMap<String, (String, i32, Vec<[usize; 2]>)>;

/// Add a candidate to `current_best`
#[allow(clippy::type_complexity, clippy::missing_panics_doc)] // .unwrap()
pub fn update_current_best(
    candidate: Option<(Tosytype, i32, i32, String, Vec<[usize; 2]>)>,
    mut current_best: [[BestLujvoMap; 3]; 3],
) -> [[BestLujvoMap; 3]; 3] {
    if candidate.is_none() {
        return current_best;
    }
    let (tosmabru_type, num_consonants, res_score, res_lujvo, res_indices) = candidate.unwrap();
    let lujvo_f = strin!(&res_lujvo, -1);
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

/// Create the best lujvo for the tanru (list). Recommended to use [`get_lujvo_with_analytics`]
/// instead if you have a string
/// # Errors
/// if given less than two words, or if some part of the jvozba process fails
pub fn get_lujvo_from_list(
    valsi_list: &[String],
    settings: &Settings,
) -> Result<(String, i32, Vec<[usize; 2]>), Jvonunfli> {
    let rafsi_list_list = get_rafsi_list_list(
        valsi_list,
        &extract!(settings, y_hyphens, exp_rafsi, consonants, glides, allow_mz),
    );
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
        return Err(Jvonunfli::FakeTypeError("not enough words".to_string()));
    }
    for rafsi0 in &rafsi_list_list[0] {
        for rafsi1 in &rafsi_list_list[1] {
            let tosmabru_type =
                if tarmi_ignoring_hyphen(&rafsi0.0) == Tarmi::Cvc && !settings.generate_cmevla {
                    if strin!(&rafsi0.0, -1) == 'y' {
                        Tosytype::Tosyhuhu
                    } else {
                        Tosytype::Tosmabru
                    }
                } else {
                    Tosytype::Tosynone
                };
            let res = combine(
                &rafsi0.0,
                &rafsi1.0,
                rafsi0.1,
                rafsi1.1,
                score(&rafsi0.0),
                &[[0, strip_hyphens(&rafsi0.0).len()]],
                tosmabru_type,
                rafsi_list_list.len(),
                &extract!(
                    settings,
                    generate_cmevla,
                    y_hyphens,
                    consonants,
                    glides,
                    allow_mz
                ),
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
                        &previous_best[tosmabru_type as usize][num_consonants]
                    {
                        let res = combine(
                            &lujvo_and_score.0,
                            &rafsi.0,
                            num_consonants as i32,
                            rafsi.1,
                            lujvo_and_score.1,
                            &lujvo_and_score.2,
                            tosmabru_type,
                            rafsi_list_list.len(),
                            &extract!(
                                settings,
                                generate_cmevla,
                                y_hyphens,
                                consonants,
                                glides,
                                allow_mz
                            ),
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
        if (is_vowel(strin!(c, 0)) && !settings.generate_cmevla
            || is_consonant(strin!(c, 0)) && settings.generate_cmevla)
            && lujvo_and_score.1 < best_score
        {
            (best_lujvo, best_score, best_indices) = lujvo_and_score.clone();
        }
    }
    if best_lujvo.is_empty() {
        Err(Jvonunfli::NoLujvoFoundError(format!(
            "{{{}}} can't be turned into a lujvo",
            valsi_list.join(" ")
        )))
    } else {
        Ok((best_lujvo, best_score, best_indices))
    }
}

/// Create the best lujvo for the tanru (string)
/// # Errors
/// if given less than two words, or if some part of the jvozba process fails
pub fn get_lujvo_with_analytics(
    tanru: &str,
    settings: &Settings,
) -> Result<(String, i32, Vec<[usize; 2]>), Jvonunfli> {
    get_lujvo_from_list(&process_tanru(tanru), settings)
}
/// Create the best lujvo for the tanru (string). Doesns't output the score
/// # Errors
/// if given less than two words, or if some part of the jvozba process fails
pub fn get_lujvo(tanru: &str, settings: &Settings) -> Result<String, Jvonunfli> {
    Ok(get_lujvo_with_analytics(tanru, settings)?.0)
}
