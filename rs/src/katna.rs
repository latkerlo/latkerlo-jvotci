//! Functions for decomposing a lujvo into **rafsi**.

use itertools::Itertools as _;

use crate::{
    data::{HYPHENS, INITIAL, MZ_VALID, VALID},
    exceptions::Jvonunfli::{
        self, DecompositionError, FakeTypeError, InvalidClusterError, NoLujvoFoundError,
        NotBrivlaError,
    },
    extract,
    jvozba::{get_lujvo_from_list, score, tiebreak},
    rafsi::RAFSI,
    strin, strsl,
    tarmi::{
        BrivlaType::{Cmevla, ExtendedLujvo, Lujvo},
        Settings,
        Tarmi::{Ccv, Ccvc, Ccvcv, Cvc, Cvcc, Cvccv, Cvhv, Cvv},
        YHyphenSetting::{ForceY, Standard},
        is_consonant, is_vowel, rafsi_tarmi,
    },
    tools::{analyze_brivla, is_brivla},
};

/// Returns the word with the given rafsi, if one exists.
pub fn search_selrafsi_from_rafsi(r: &str) -> Option<String> {
    if r != "brod" && r.len() == 4 && !r.chars().any(|c| c == '\'') {
        "aeiou"
            .chars()
            .map(|c| format!("{r}{c}"))
            .find(|gismu| RAFSI.contains_key(gismu.as_str()))
            .or_else(|| RAFSI.iter().find_map(|(v, rl)| rl.contains(&r).then(|| (*v).to_string())))
    } else {
        RAFSI.iter().find_map(|(v, rl)| rl.contains(&r).then(|| (*v).to_string()))
    }
}
/// Creates a list of selrafsi (source words) and formatted unassigned rafsi.
/// # Errors
/// None, hopefully. If you somehow do encounter one, please report it as an
/// issue in [the GitHub repository][github].
///
/// [github]: https://github.com/latkerlo/latkerlo-jvotci
#[allow(clippy::missing_panics_doc)] // .unwrap()
pub fn selrafsi_list_from_rafsi_list(
    rafsi_list: &[String],
    settings: &Settings,
) -> Result<Vec<String>, Jvonunfli> {
    let mut res = rafsi_list
        .iter()
        .map(|r| if HYPHENS.contains(&r.as_str()) { String::new() } else { r.clone() })
        .collect_vec();
    let selrafsi_list = res.iter().map(|r| search_selrafsi_from_rafsi(r)).collect_vec();
    for (i, _) in res.clone().iter().enumerate() {
        if res[i].is_empty() {
            continue;
        }
        if selrafsi_list[i].is_some() {
            res[i] = selrafsi_list[i].clone().unwrap();
        } else if rafsi_list.len() >= 2
            && i < rafsi_list.len() - 2
            && strin!(&rafsi_list[i + 1], 0) == 'y'
            && is_brivla(&format!("{}a", res[i]), &extract!(settings; y_hyphens, allow_mz))
        {
            res[i] = format!("{}-", res[i]);
        } else if is_brivla(&res[i], &extract!(settings; y_hyphens, allow_mz)) {
            // do nothing
        } else if i == rafsi_list.len() - 1
            && is_brivla(&format!("{}a", res[i]), &extract!(settings; y_hyphens, allow_mz))
        {
            res[i] = format!("{}-", res[i]);
        } else {
            res[i] = format!("-{}-", res[i]);
        }
    }
    Ok(res.iter().filter(|r| !r.is_empty()).cloned().collect_vec())
}

/// Checks if `corr` and `other` represent the same lujvo. `other` may have
/// unnecessary hyphens.
#[must_use]
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
            && [Cvv, Cvhv].contains(&rafsi_tarmi(&other[i - 1]))
            && (i > 1 || [Ccvcv, Ccvc, Ccv].contains(&rafsi_tarmi(&other[i + 1])))
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

/// Decomposes a lujvo into rafsi and hyphens.
/// # Errors
/// Any errors from [`jvokaha2`] (if the actual decomposing part fails) or
/// [`get_lujvo_from_list`] (if there are issues when re-assembling it) are
/// forwarded. A [`FakeTypeError`] is also returned if there are less than two
/// words given, and a [`DecompositionError`] is returned if the lujvo has a
/// problem that is fixable (TODO: list).
#[allow(clippy::missing_panics_doc)] // .unwrap()
pub fn jvokaha(lujvo: &str, settings: &Settings) -> Result<Vec<String>, Jvonunfli> {
    let arr = jvokaha2(lujvo, &extract!(settings; y_hyphens, allow_mz))?;
    let rafsi_tanru = arr.iter().filter(|r| r.len() > 2).map(|r| format!("-{r}-")).collect_vec();
    if rafsi_tanru.len() == 1 {
        return Err(FakeTypeError("not enough rafsi".to_string()));
    }
    let correct_lujvo = get_lujvo_from_list(&rafsi_tanru, &Settings {
        generate_cmevla: is_consonant(strin!(&arr[arr.len() - 1], -1)),
        ..extract!(settings; y_hyphens, consonants, glides, allow_mz)
    });
    if let Err(e) = correct_lujvo {
        match e {
            NoLujvoFoundError(m) => return Err(DecompositionError(m)),
            _ => return Err(e),
        }
    }
    let correct_lujvo = correct_lujvo.unwrap().0;
    let cool_and_good = if settings.y_hyphens == ForceY {
        correct_lujvo == lujvo
    } else {
        compare_lujvo_pieces(&jvokaha2(&correct_lujvo, &extract!(settings; allow_mz))?, &arr)
    };
    if cool_and_good {
        Ok(arr)
    } else {
        Err(DecompositionError(format!(
            "{{{lujvo}}} is malformed and should be {{{correct_lujvo}}}"
        )))
    }
}

/// The actual decomposing work part of [`jvokaha`].
/// # Errors
/// An [`InvalidClusterError`] is returned if the lujvo has any invalid
/// clusters.
///
/// A [`NotBrivlaError`] is returned if the lujvo starts with a CCV rafsi +
/// *'y*, because it would be a slinku'i: *uajvo* is a valid zi'evla, and we
/// want zi'evla to be able to go at the start of lujvo, so e.g. *jvo'yfliba*
/// can't be a word since you could put a *ua* in front of it to get "another"
/// lujvo.
///
/// A [`DecompositionError`] is returned if something else goes wrong. This is
/// not user-facing if you are using [`analyze_brivla`] or [`get_veljvo`].
pub fn jvokaha2(lujvo: &str, settings: &Settings) -> Result<Vec<String>, Jvonunfli> {
    let orig = lujvo;
    let mut lujvo = lujvo;
    let mut res: Vec<&str> = vec![];
    loop {
        if lujvo.is_empty() {
            return Ok(res.iter().copied().map(String::from).collect_vec());
        }
        if !res.is_empty() && res[res.len() - 1].len() != 1 {
            if strin!(lujvo, 0) == 'y'
                || settings.y_hyphens != ForceY
                    && (strsl!(lujvo, 0..2) == "nr"
                        || strin!(lujvo, 0) == 'r'
                            && lujvo.len() >= 2
                            && is_consonant(strin!(lujvo, 1)))
            {
                res.push(strsl!(lujvo, 0..1));
                lujvo = strsl!(lujvo, 1..);
                continue;
            } else if settings.y_hyphens != Standard && strsl!(lujvo, 0..2) == "'y" {
                res.push(strsl!(lujvo, 0..2));
                lujvo = strsl!(lujvo, 2..);
                continue;
            }
        }
        if rafsi_tarmi(strsl!(lujvo, 0..3)) == Cvv
            && ["ai", "ei", "oi", "au"].contains(&strsl!(lujvo, 1..3))
        {
            res.push(strsl!(lujvo, 0..3));
            lujvo = strsl!(lujvo, 3..);
            continue;
        }
        if rafsi_tarmi(strsl!(lujvo, 0..4)) == Cvhv {
            res.push(strsl!(lujvo, 0..4));
            lujvo = strsl!(lujvo, 4..);
            continue;
        }
        if [Cvcc, Ccvc].contains(&rafsi_tarmi(strsl!(lujvo, 0..4))) {
            if is_vowel(strin!(lujvo, 1)) {
                if !if settings.allow_mz { &MZ_VALID } else { &VALID }
                    .contains(&strsl!(lujvo, 2..4))
                {
                    return Err(InvalidClusterError(format!(
                        "{{{orig}}} contains an invalid cluster",
                    )));
                }
            } else if !INITIAL.contains(&strsl!(lujvo, 0..2)) {
                return Err(InvalidClusterError(format!(
                    "{{{orig}}} starts with an invalid cluster",
                )));
            }
            if lujvo.len() == 4 || strin!(lujvo, 4) == 'y' {
                res.push(strsl!(lujvo, 0..4));
                if strin!(lujvo, 4) == 'y' {
                    res.push("y");
                }
                lujvo = strsl!(lujvo, 5..);
                continue;
            }
        }
        if [Cvccv, Ccvcv].contains(&rafsi_tarmi(lujvo)) {
            res.push(lujvo);
            return Ok(res.iter().copied().map(String::from).collect_vec());
        }
        if rafsi_tarmi(strsl!(lujvo, 0..3)) == Cvc {
            res.push(strsl!(lujvo, 0..3));
            lujvo = strsl!(lujvo, 3..);
            continue;
        }
        if rafsi_tarmi(strsl!(lujvo, 0..3)) == Ccv {
            if !INITIAL.contains(&strsl!(lujvo, 0..2)) {
                return Err(InvalidClusterError(format!(
                    "{{{orig}}} starts with an invalid cluster",
                )));
            }
            if lujvo == orig && strsl!(lujvo, 3..5) == "'y" {
                return Err(NotBrivlaError(format!(
                    "{{{orig}}} starts with CCV'y, making it a slinku'i"
                )));
            }
            res.push(strsl!(lujvo, 0..3));
            lujvo = strsl!(lujvo, 3..);
            continue;
        }
        return Err(DecompositionError(format!("{{{orig}}} can't be decomposed")));
    }
}

/// Calculates the score for a lujvo.
/// # Errors
/// Errors are forwarded from [`get_veljvo`] and [`analyze_brivla`].
pub fn score_lujvo(lujvo: &str, settings: &Settings) -> Result<i32, Jvonunfli> {
    get_veljvo(lujvo, settings)?;
    let decomp = analyze_brivla(lujvo, settings)?.1;
    Ok(decomp
        .iter()
        .map(|r| {
            if ["y", "n", "r", ""].contains(&r.as_str()) { 1100 * r.len() as i32 } else { score(r) }
        })
        .sum::<i32>()
        - tiebreak(lujvo))
}

/// Gets the selrafsi (source tanru) and formatted unassigned rafsi for this
/// lujvo.
/// # Errors
/// Errors are forwarded from [`analyze_brivla`]. Additionally a
/// [`DecompositionError`] is returned if given something other than a lujvo.
pub fn get_veljvo(lujvo: &str, settings: &Settings) -> Result<Vec<String>, Jvonunfli> {
    let (b_type, rafsi_list) = analyze_brivla(
        lujvo,
        &extract!(
            settings;
            y_hyphens,
            exp_rafsi,
            consonants,
            generate_cmevla,
            allow_mz
        ),
    )?;
    if ![Lujvo, ExtendedLujvo, Cmevla].contains(&b_type) {
        return Err(DecompositionError(format!(
            "{{{lujvo}}} is a {}, not a lujvo or decomposable cmevla",
            b_type.to_string().to_lowercase()
        )));
    }
    selrafsi_list_from_rafsi_list(&rafsi_list, &extract!(settings; y_hyphens, allow_mz))
}
