use crate::{rafsi::RAFSI, tools::{char, slice}, tarmi::{is_consonant, is_vowel, rafsi_tarmi, Tarmi}, data::{VALID, INITIAL}};

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

//

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
        break Err("failed to decompose {original}".to_string());
    }
}