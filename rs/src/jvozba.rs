//! Tools for making lujvo.

use std::collections::HashMap;
use regex::Regex;
use crate::{tarmi::{tarmi_ignoring_hyphen, is_only_lojban_characters, is_valid_rafsi, is_gismu, Tarmi, rafsi_tarmi, is_consonant, is_vowel}, tools::{split_words, slice, char}, rafsi::RAFSI, data::{VALID, INITIAL}};

/// Returns the score for the given rafsi.
pub fn score(rafsi: &str) -> usize {
    1000 * rafsi.len() - 500 * rafsi.matches('\'').count() + 100 * rafsi.matches('y').count() - 10 * tarmi_ignoring_hyphen(rafsi).as_usize() - rafsi.matches(|c| "aeiou".contains(c)).count()
}

pub fn process_tanru(tanru: Vec<String>) -> Vec<String> {
    let valsi_list = tanru;
    let mut expanded = Vec::<String>::new();
    for v in valsi_list {
        expanded = [expanded, if v.contains('-') {vec![v]} else {split_words(&v)}].concat();
    }
    expanded
}

/// Takes a list of words and whether the lujvo is intended to be a name (end in a consonant); returns rafsi for all of the words. Returns an Err on invalid clusters/letters or nonfinal 5-letter rafsi
pub fn get_rafsi_list_list(valsi_list: Vec<String>, cmene: bool) -> Result<Vec<Vec<String>>, String> {
    let mut rafsi_list_list = Vec::new();
    for (i, v) in valsi_list.iter().enumerate() {
        let mut v = v.clone();
        let mut rafsi_list = Vec::new();
        if char(&v, v.len() - 1) == '-' {
            v = Regex::new("^-+|-+$").unwrap().replace_all(&v, "").to_string();
            if !is_only_lojban_characters(&v) {
                return Err(format!("non-Lojban character in {{{v}}}"));
            }
            if !is_valid_rafsi(&v) {
                return Err(format!("invalid cluster in {{{v}}}"));
            }
            if is_gismu(&v) && i != valsi_list.len() - 1 {
                return Err(format!("non-final 5-letter rafsi {{{v}}}"));
            }
            if [Tarmi::Ccvc, Tarmi::Cvcc].contains(&rafsi_tarmi(&v)) {
                if cmene && i == valsi_list.len() - 1 {
                    rafsi_list.push(v.clone());
                }
            } else {
                rafsi_list.push(v.clone());
            }
            if is_consonant(char(&v, v.len() - 1)) && !(cmene && i == valsi_list.len() - 1) {
                rafsi_list.push(format!("{}y", &v));
            }
        } else {
            if !is_only_lojban_characters(&v) {
                return Err(format!("non-Lojban character in {v}"))
            }
            let cunrafsi_list = RAFSI.get(v.as_str());
            if let Some(thing) = cunrafsi_list {
                for cunrafsi in thing.iter() {
                    rafsi_list.push(cunrafsi.to_string());
                    if is_consonant(char(cunrafsi, cunrafsi.len() - 1)) {
                        rafsi_list.push(format!("{cunrafsi}y"));
                    }
                }
            }
            if is_gismu(&v) {
                if !is_valid_rafsi(&v) {
                    return Err(format!("invalid cluster in {{{v}}}"));
                }
                if i == valsi_list.len() - 1 {
                    if cmene {
                        rafsi_list.push(slice(&v, 0, -1));
                    } else {
                        rafsi_list.push(v);
                    }
                } else {
                    rafsi_list.push(format!("{}y", slice(&v, 0, -1)))
                }
            }
        }
        rafsi_list_list.push(rafsi_list);
    }
    Ok(rafsi_list_list)
}

/// Appends a rafsi to a lujvo-in-progress
pub fn combine(lujvo: &str, rafsi: &str, score: usize, mut tosmabru: bool, cmene: bool, tanru_len: usize) -> Option<(usize, usize, String)> {
    let lujvo_final = char(lujvo, lujvo.len() - 1);
    let rafsi_initial = char(rafsi, 0);
    if is_consonant(lujvo_final) && is_consonant(rafsi_initial) && !VALID.contains(&(lujvo_final.to_string() + &rafsi_initial.to_string()).as_str()) {
        return None;
    }
    if ["ndj", "ndz", "ntc", "nts"].contains(&(lujvo_final.to_string() + &slice(rafsi, 0, 2)).as_str()) {
        return None;
    }
    let raftai1 = tarmi_ignoring_hyphen(rafsi);
    let mut hyphen = "";
    if lujvo.len() <= 5 && !cmene {
        let raftai0 = tarmi_ignoring_hyphen(lujvo);
        if [Tarmi::Cvhv, Tarmi::Cvv].contains(&raftai0) {
            if rafsi_initial == 'r' {
                hyphen = "n";
            } else {
                hyphen = "r";
            }
        }
        if tanru_len == 2 && raftai1 == Tarmi::Ccv {
            hyphen = "";
        }
    }
    if tosmabru {
        if !INITIAL.contains(&(lujvo_final.to_string() + &rafsi_initial.to_string()).as_str()) {
            tosmabru = false;
        } else if raftai1 == Tarmi::Cvccv {
            if INITIAL.contains(&slice(rafsi, 2, 4).as_str()) {
                return None;
            }
            tosmabru = false;
        } else if raftai1 == Tarmi::Cvc {
            if char(rafsi, rafsi.len() - 1) == 'y' {
                return None;
            }
        } else {
            tosmabru = false;
        }
    }
    Some((tosmabru as usize, score + 1100 * hyphen.len() + self::score(rafsi), lujvo.to_owned() + hyphen + rafsi))
}

pub type BestLujvoMap = HashMap<String, (String, usize)>;

pub fn update_current_best(result: Option<(usize, usize, String)>, mut current_best: [BestLujvoMap; 2]) -> [BestLujvoMap; 2] {
    if let Some((tosmabru, res_score, res_lujvo)) = result {
        let lujvo_final = char(&res_lujvo, res_lujvo.len() - 1).to_string();
        if !current_best[tosmabru].contains_key(&lujvo_final) || current_best[tosmabru].get(&lujvo_final).unwrap().1 > res_score {
            current_best[tosmabru].insert(lujvo_final, (res_lujvo, res_score));
        }
    }
    current_best
}

/// Makes a lujvo! `cmene` is whether or not it should end in a consonant
pub fn get_lujvo(tanru: &str, cmene: bool) -> Result<(String, usize), String> {
    get_lujvo2(process_tanru(tanru.to_string().split_whitespace().map(String::from).collect()), cmene)
}

pub fn get_lujvo2(valsi_list: Vec<String>, cmene: bool) -> Result<(String, usize), String> {
    let rafsi_list_list = get_rafsi_list_list(valsi_list.clone(), cmene)?;
    let mut current_best = [BestLujvoMap::new(), BestLujvoMap::new()];
    for rafsi0 in &rafsi_list_list[0] {
        for rafsi1 in &rafsi_list_list[1] {
            let tosmabru = if tarmi_ignoring_hyphen(rafsi0) == Tarmi::Cvc && char(rafsi0, rafsi0.len() - 1) != 'y' && !cmene {1} else {0};
            let result = combine(rafsi0, rafsi1, score(rafsi0), tosmabru != 0, cmene, rafsi_list_list.len());
            current_best = update_current_best(result, current_best);
        }
    }
    let mut previous_best = current_best;
    for rafsi_list in &rafsi_list_list[2..] {
        current_best = [BestLujvoMap::new(), BestLujvoMap::new()];
        for rafsi in rafsi_list {
            for (tosmabru, _) in previous_best.iter().enumerate() {
                for lujvo_and_score in previous_best[tosmabru].values() {
                    let result = combine(&lujvo_and_score.0, rafsi, lujvo_and_score.1, tosmabru != 0, cmene, 0);
                    current_best = update_current_best(result, current_best);
                }
            }
        }
        previous_best = current_best;
    }
    let mut best_lujvo = "".to_string();
    let mut best_score = usize::MAX;
    for (lerfu, lujvo_and_score) in &previous_best[0] {
        let lerfu = char(lerfu, 0);
        if is_vowel(lerfu) ^ cmene && lujvo_and_score.1 < best_score {
            (best_lujvo, best_score) = lujvo_and_score.clone();
        }
    }
    if best_lujvo.is_empty() {
        Err(format!("no lujvo found for {{{}}}", valsi_list.join(" ")))
    } else {
        Ok((best_lujvo, best_score))
    }
}