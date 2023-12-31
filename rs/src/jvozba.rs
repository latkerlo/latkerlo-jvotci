use regex::Regex;
use crate::{tarmi::{tarmi_ignoring_hyphen, is_only_lojban_characters, is_valid_rafsi, is_gismu, Tarmi, rafsi_tarmi, is_consonant}, tools::{split_words, slice, char}, rafsi::RAFSI};

pub fn score(rafsi: &str) -> usize {
    1000 * rafsi.len() - 500 * rafsi.matches('\'').count() + 100 * rafsi.matches('y').count() - 10 * tarmi_ignoring_hyphen(rafsi).as_usize() - rafsi.matches(|c| "aeiou".contains(c)).count()
}

// wanna pass a string? trim & split_whitespace it
pub fn process_tanru(tanru: Vec<String>) -> Vec<String> {
    let valsi_list = tanru;
    let mut expanded = Vec::<String>::new();
    for v in valsi_list {
        expanded = [expanded, if v.contains('-') {vec![v]} else {split_words(&v)}].concat();
    }
    expanded
}

pub fn get_rafsi_list_list(valsi_list: Vec<String>, cmene: bool) -> Result<Vec<Vec<String>>, String> {
    let mut rafsi_list_list = Vec::new();
    for (i, &v) in valsi_list.iter().enumerate() {
        let v = v.clone();
        let mut rafsi_list = Vec::new();
        if slice(&v, -1, v.len() as isize) == "-" {
            Regex::new("^-+|-+$").unwrap().replace_all(&v, "");
            if !is_only_lojban_characters(&v) {
                return Err(format!("non-Lojban character in {v}"));
            }
            if !is_valid_rafsi(&v) {
                return Err(format!("invalid cluster in {v}"));
            }
            if is_gismu(&v) && i != valsi_list.len() - 1 {
                return Err(format!("non-final 5-letter rafsi {{{v}}}"));
            }
            if [Tarmi::Ccvc, Tarmi::Cvcc].contains(&rafsi_tarmi(&v)) {
                if cmene && i == valsi_list.len() - 1 {
                    rafsi_list.push(v);
                }
            } else {
                rafsi_list.push(v);
            }
            //                         fuck
            if is_consonant(char(&slice(v, -1, v.len() as isize), 0)) && !(cmene && i == valsi_list.len() - 1) {
                rafsi_list.push(format!("{v}y"));
            }
        } else {
            if !is_only_lojban_characters(&v) {
                return Err(format!("non-Lojban character in {v}"))
            }
            let cunrafsi_list = RAFSI.get(v.as_str());
            match cunrafsi_list {
                Some(thing) => {
                    for cunrafsi in *thing {
                        rafsi_list.push(cunrafsi.to_string());
                    }
                }
                _ => ()
            }
            if is_gismu(&v) {
                if !is_valid_rafsi(&v) {
                    return Err(format!("invalid cluster in {v}"));
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