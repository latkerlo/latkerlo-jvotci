use itertools::Itertools as _;

use crate::{
    data::{HYPHENS, INITIAL, MZ_VALID, VALID},
    exceptions::Jvonunfli,
    extract,
    jvozba::{get_lujvo_from_list, score, tiebreak},
    rafsi::RAFSI,
    tarmi::{is_consonant, is_vowel, rafsi_tarmi, BrivlaType, Settings, Tarmi, YHyphenSetting},
    tools::{analyze_brivla, char, is_brivla, slice, slice_},
};

/// Return the word with this rafsi if one exists
pub fn search_selrafsi_from_rafsi(r: &str) -> Option<String> {
    if r != "brod" && r.len() == 4 && !r.chars().any(|c| c == '\'') {
        "aeiou"
            .chars()
            .map(|c| format!("{r}{c}"))
            .find(|gismu| RAFSI.contains_key(gismu.as_str()))
            .or_else(|| {
                RAFSI
                    .iter()
                    .find_map(|(v, rl)| rl.contains(&r).then(|| (*v).to_string()))
            })
    } else {
        RAFSI
            .iter()
            .find_map(|(v, rl)| rl.contains(&r).then(|| (*v).to_string()))
    }
}
/// Create a list of selrafsi and formatted unassigned rafsi
/// # Errors
/// if any rafsi corresponds to an invalid brivla
#[allow(clippy::missing_panics_doc)] // .unwrap()
pub fn selrafsi_list_from_rafsi_list(
    rafsi_list: &[String],
    settings: &Settings,
) -> Result<Vec<String>, Jvonunfli> {
    let mut res = rafsi_list
        .iter()
        .map(|r| {
            if HYPHENS.contains(&r.as_str()) {
                String::new()
            } else {
                r.to_string()
            }
        })
        .collect_vec();
    let selrafsi_list = res
        .iter()
        .map(|r| search_selrafsi_from_rafsi(r))
        .collect_vec();
    for (i, _) in res.clone().iter().enumerate() {
        if res[i].is_empty() {
            continue;
        }
        if selrafsi_list[i].is_some() {
            res[i] = selrafsi_list[i].clone().unwrap();
        } else if rafsi_list.len() >= 2
            && i < rafsi_list.len() - 2
            && char(&rafsi_list[i + 1], 0) == 'y'
            && is_brivla(
                &format!("{}a", res[i]),
                &extract!(settings, y_hyphens, allow_mz),
            )
        {
            res[i] = format!("{}-", res[i]);
        } else if is_brivla(&res[i], &extract!(settings, y_hyphens, allow_mz)) {
            // do nothing
        } else if i == rafsi_list.len() - 1
            && is_brivla(
                &format!("{}a", res[i]),
                &extract!(settings, y_hyphens, allow_mz),
            )
        {
            res[i] = format!("{}-", res[i]);
        } else {
            res[i] = format!("-{}-", res[i]);
        }
    }
    Ok(res.iter().filter(|r| !r.is_empty()).cloned().collect_vec())
}

/// Check if `corr` and `other` represent the same lujvo. `other` may have unnecessary hyphens
pub fn compare_lujvo_pieces(corr: &[String], other: &[String]) -> bool {
    let mut i = 0;
    for part in corr {
        if part == &other[i] {
            i += 1;
            continue;
        }
        if 0 < i
            && i < other.len() - 1
            && "rn".contains(&other[i])
            && [Tarmi::Cvv, Tarmi::Cvhv].contains(&rafsi_tarmi(&other[i - 1]))
            && (i > 1
                || [Tarmi::Ccvcv, Tarmi::Ccvc, Tarmi::Ccv].contains(&rafsi_tarmi(&other[i + 1])))
        {
            i += 1;
        }
        if part == &other[i] {
            i += 1;
        } else {
            return false;
        }
    }
    i == other.len()
}

/// Decompose a lujvo into rafsi and hyphens.
/// # Errors
/// if the lujvo is malformed
#[allow(clippy::missing_panics_doc)] // .unwrap()
pub fn jvokaha(lujvo: &str, settings: &Settings) -> Result<Vec<String>, Jvonunfli> {
    let arr = jvokaha2(lujvo, &extract!(settings, y_hyphens, allow_mz))?;
    let rafsi_tanru = arr
        .iter()
        .filter(|r| r.len() > 2)
        .map(|r| format!("-{r}-"))
        .collect_vec();
    if rafsi_tanru.len() == 1 {
        return Err(Jvonunfli::FakeTypeError("not enough rafsi".to_string()));
    }
    let correct_lujvo = get_lujvo_from_list(
        &rafsi_tanru,
        &Settings {
            generate_cmevla: is_consonant(char(&arr[arr.len() - 1], -1)),
            ..extract!(settings, y_hyphens, consonants, glides, allow_mz)
        },
    );
    if let Err(e) = correct_lujvo {
        match e {
            Jvonunfli::NoLujvoFoundError(m) => return Err(Jvonunfli::DecompositionError(m)),
            _ => return Err(e),
        }
    }
    let correct_lujvo = correct_lujvo.unwrap().0;
    let cool_and_good = if settings.y_hyphens == YHyphenSetting::ForceY {
        correct_lujvo == lujvo
    } else {
        compare_lujvo_pieces(
            &jvokaha2(
                &correct_lujvo,
                &Settings {
                    y_hyphens: YHyphenSetting::Standard,
                    allow_mz: settings.allow_mz,
                    ..Settings::default()
                },
            )?,
            &arr,
        )
    };
    if cool_and_good {
        Ok(arr)
    } else {
        Err(Jvonunfli::DecompositionError(format!(
            "{{{lujvo}}} is malformed and should be {{{correct_lujvo}}}"
        )))
    }
}

/// Decompose a lujvo into rafsi and hyphens.
/// # Errors
/// if not decomposable.
pub fn jvokaha2(lujvo: &str, settings: &Settings) -> Result<Vec<String>, Jvonunfli> {
    let orig = lujvo;
    let mut lujvo = lujvo;
    let mut res: Vec<&str> = vec![];
    loop {
        if lujvo.is_empty() {
            return Ok(res.iter().copied().map(String::from).collect_vec());
        }
        if !res.is_empty() && res[res.len() - 1].len() != 1 {
            if char(lujvo, 0) == 'y'
                || settings.y_hyphens != YHyphenSetting::ForceY
                    && (slice(lujvo, 0, 2) == "nr"
                        || char(lujvo, 0) == 'r'
                            && lujvo.len() >= 2
                            && is_consonant(char(lujvo, 1)))
            {
                res.push(slice(lujvo, 0, 1));
                lujvo = slice_(lujvo, 1);
                continue;
            } else if settings.y_hyphens != YHyphenSetting::Standard && slice(lujvo, 0, 2) == "'y" {
                res.push(slice(lujvo, 0, 2));
                lujvo = slice_(lujvo, 2);
                continue;
            }
        }
        if rafsi_tarmi(slice(lujvo, 0, 3)) == Tarmi::Cvv
            && ["ai", "ei", "oi", "au"].contains(&slice(lujvo, 1, 3))
        {
            res.push(slice(lujvo, 0, 3));
            lujvo = slice_(lujvo, 3);
            continue;
        }
        if rafsi_tarmi(slice(lujvo, 0, 4)) == Tarmi::Cvhv {
            res.push(slice(lujvo, 0, 4));
            lujvo = slice_(lujvo, 4);
            continue;
        }
        if [Tarmi::Cvcc, Tarmi::Ccvc].contains(&rafsi_tarmi(slice(lujvo, 0, 4))) {
            if is_vowel(char(lujvo, 1)) {
                if !if settings.allow_mz { &MZ_VALID } else { &VALID }.contains(&slice(lujvo, 2, 4))
                {
                    return Err(Jvonunfli::InvalidClusterError(format!(
                        "{{{orig}}} contains an invalid cluster",
                    )));
                }
            } else if !INITIAL.contains(&slice(lujvo, 0, 2)) {
                return Err(Jvonunfli::InvalidClusterError(format!(
                    "{{{orig}}} starts with an invalid cluster",
                )));
            }
            if lujvo.len() == 4 || char(lujvo, 4) == 'y' {
                res.push(slice(lujvo, 0, 4));
                if char(lujvo, 4) == 'y' {
                    res.push("y");
                }
                lujvo = slice_(lujvo, 5);
                continue;
            }
        }
        if [Tarmi::Cvccv, Tarmi::Ccvcv].contains(&rafsi_tarmi(lujvo)) {
            res.push(lujvo);
            return Ok(res.iter().copied().map(String::from).collect_vec());
        }
        if rafsi_tarmi(slice(lujvo, 0, 3)) == Tarmi::Cvc {
            res.push(slice(lujvo, 0, 3));
            lujvo = slice_(lujvo, 3);
            continue;
        }
        if rafsi_tarmi(slice(lujvo, 0, 3)) == Tarmi::Ccv {
            if !INITIAL.contains(&slice(lujvo, 0, 2)) {
                return Err(Jvonunfli::InvalidClusterError(format!(
                    "{{{orig}}} starts with an invalid cluster",
                )));
            }
            if lujvo == orig && slice(lujvo, 3, 5) == "'y" {
                return Err(Jvonunfli::NotBrivlaError(format!(
                    "{{{orig}}} starts with CCV'y, making it a slinku'i"
                )));
            }
            res.push(slice(lujvo, 0, 3));
            lujvo = slice_(lujvo, 3);
            continue;
        }
        return Err(Jvonunfli::DecompositionError(format!(
            "{{{orig}}} can't be decomposed"
        )));
    }
}

/// Calculate the score for a lujvo
/// # Errors
/// if not given a lujvo or cmejvo
pub fn score_lujvo(lujvo: &str, settings: &Settings) -> Result<i32, Jvonunfli> {
    get_veljvo(lujvo, settings)?;
    let decomp = analyze_brivla(lujvo, settings)?.1;
    Ok(decomp
        .iter()
        .map(|r| {
            if ["y", "n", "r", ""].contains(&r.as_str()) {
                1100 * r.len() as i32
            } else {
                score(r)
            }
        })
        .sum::<i32>()
        - tiebreak(lujvo))
}

/// Get the selrafsi (source tanru) and formatted unassigned rafsi for this lujvo
/// # Errors
/// if not given a lujvo or if decomposing fails
pub fn get_veljvo(lujvo: &str, settings: &Settings) -> Result<Vec<String>, Jvonunfli> {
    let (b_type, rafsi_list) = analyze_brivla(
        lujvo,
        &extract!(
            settings,
            y_hyphens,
            exp_rafsi,
            consonants,
            generate_cmevla,
            allow_mz
        ),
    )?;
    if ![
        BrivlaType::Lujvo,
        BrivlaType::ExtendedLujvo,
        BrivlaType::Cmevla,
    ]
    .contains(&b_type)
    {
        return Err(Jvonunfli::DecompositionError(format!(
            "{{{lujvo}}} is a {}, not a lujvo or decomposable cmevla",
            b_type.to_string().to_lowercase()
        )));
    }
    selrafsi_list_from_rafsi_list(&rafsi_list, &extract!(settings, y_hyphens, allow_mz))
}
