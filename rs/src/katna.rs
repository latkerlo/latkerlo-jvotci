//! Tools for breaking lujvo.

use crate::{rafsi::RAFSI, tools::{char, slice}, tarmi::{is_consonant, is_vowel, rafsi_tarmi, Tarmi}, data::{VALID, INITIAL}, jvozba::get_lujvo2};

/// Gets `Some(`the word with this rafsi`)` or `None` if none exists.
pub fn search_selrafsi_from_rafsi(rafsi: &str) -> Option<String> {
    if rafsi.len() == 5 && RAFSI.contains_key(rafsi) {
         return Some(rafsi.to_string());
    }
    if rafsi != "brod" && rafsi.len() == 4 && !rafsi.contains('\'') {
        for u in 0..5 {
            let candidate = format!("{rafsi}{}", char("aeiou", u));
            if RAFSI.contains_key(candidate.as_str()) {
                return Some(candidate);
            }
        }
    }
    for (v, r) in RAFSI.iter() {
        if r.contains(&rafsi) {
            return Some(v.to_string());
        }
    }
    None
}

/// Tries to break a lujvo; returns a list of rafsi and hyphens. Returns an Err if the lujvo is invalid.
pub fn jvokaha(lujvo: &str) -> Result<Vec<String>, String> {
    let arr = jvokaha2(lujvo);
    arr.as_ref()?;
    let arr = arr.unwrap();
    let rafsi_tanru = arr.iter().filter(|x| x.len() > 1).map(|x| format!("-{x}-")).collect();
    let correct_lujvo = get_lujvo2(rafsi_tanru, is_consonant(char(&arr[arr.len() - 1], arr[arr.len() - 1].len() - 1)));
    if correct_lujvo.is_ok() && lujvo == correct_lujvo.clone().unwrap().0 {
        Ok(arr)
    } else if correct_lujvo.is_ok() {
        Err(format!("malformed lujvo {{{lujvo}}}, should be {{{}}}", correct_lujvo.unwrap().0))
    } else {
        Err(format!("{{{lujvo}}} is not a lujvo"))
    }
}

/// Splits a lujvo into rafsi and hyphens, regardless of morphological validity. Returns an Err on invalid consonant clusters or if it's not a lujvo.
pub fn jvokaha2(lujvo: &str) -> Result<Vec<String>, String> {
    let original = lujvo;
    let mut lujvo = lujvo.to_string();
    let mut res = Vec::<String>::new();
    loop {
        if lujvo.is_empty() {
            break Ok(res);
        }
        if !res.is_empty() && res.last().unwrap().len() != 1 && (char(&lujvo, 0) == 'y' || slice(&lujvo, 0, 2) == "nr" || char(&lujvo, 0) == 'r' && is_consonant(char(&lujvo, 1))) {
            res.push(char(&lujvo, 0).to_string());
            lujvo = slice(&lujvo, 1, lujvo.len() as isize);
            continue;
        }
        if rafsi_tarmi(&slice(&lujvo, 0, 3)) == Tarmi::Cvv && ["ai", "ei", "oi", "au"].contains(&slice(&lujvo, 1, 3).as_str()) {
            res.push(slice(&lujvo, 0, 3));
            lujvo = slice(&lujvo, 3, lujvo.len() as isize);
            continue;
        }
        if rafsi_tarmi(&slice(&lujvo, 0, 4)) == Tarmi::Cvhv {
            res.push(slice(&lujvo, 0, 4));
            lujvo = slice(&lujvo, 4, lujvo.len() as isize);
            continue;
        }
        if [Tarmi::Cvcc, Tarmi::Ccvc].contains(&rafsi_tarmi(&slice(&lujvo, 0, 4))) {
            if is_vowel(char(&lujvo, 1)) {
                if !VALID.contains(&slice(&lujvo, 2, 4).as_str()) {
                    break Err(format!("invalid cluster {{{}}} in {{{}}}", slice(&lujvo, 2, 4), original));
                }
            } else if !INITIAL.contains(&slice(&lujvo, 0, 2).as_str()) {
                break Err(format!("invalid initial cluster {{{}}} in {{{}}}", slice(&lujvo, 0, 2), original));
            }
            if lujvo.len() == 4 || char(&lujvo, 4) == 'y' {
                res.push(slice(&lujvo, 0, 4).to_string());
                if char(&lujvo, 4) == 'y' {
                    res.push("y".to_string());
                }
                lujvo = slice(&lujvo, 5, lujvo.len() as isize);
                continue;
            }
        }
        if [Tarmi::Cvccv, Tarmi::Ccvcv].contains(&rafsi_tarmi(&lujvo)) {
            res.push(lujvo.to_string());
            break Ok(res);
        }
        if [Tarmi::Cvc, Tarmi::Ccv].contains(&rafsi_tarmi(&slice(&lujvo, 0, 3))) {
            res.push(slice(&lujvo, 0, 3).to_string());
            lujvo = slice(&lujvo, 3, lujvo.len() as isize);
            continue;
        }
        break Err(format!("failed to decompose {{{original}}}"));
    }
}

/// Returns the words (and unassigned rafsi if there are any, e.g. *-log-*) making up the lujvo, or an Err if `jvokaha` fails.
pub fn get_veljvo(lujvo: &str) -> Result<Vec<String>, String> {
    let rafsi_list = jvokaha(lujvo)?.iter().filter(|&x| x.len() > 1).cloned().collect::<Vec<String>>();
    let selrafsi_list = rafsi_list.iter().map(|x| search_selrafsi_from_rafsi(x).unwrap_or(format!("-{x}-"))).collect::<Vec<String>>();
    Ok(selrafsi_list)
}