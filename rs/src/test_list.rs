#![cfg(test)]
// cargo test -- --nocapture --test-threads=1

use crate::*;
use data::HYPHENS;
use itertools::Itertools;
use katna::selrafsi_list_from_rafsi_list;
use std::fs;
use tarmi::{is_consonant, SETTINGS_ITERATOR};
use tools::{char, slice, slice_};

// ported from py/tests/test_other.py
fn check_conditions(cond: &str, settings: &Settings) -> bool {
    match cond {
        "" | "ELSE" => return true,
        "ALLOW_Y" => return settings.y_hyphens == AllowY,
        "FORCE_Y" => return settings.y_hyphens == ForceY,
        "MORE_RAF" => return settings.exp_rafsi,
        "TWO_CONSONANTS" => return settings.consonants == TwoConsonants,
        "ONE_CONSONANT" => return settings.consonants == OneConsonant,
        "GLIDES" => return settings.glides,
        "YES_MZ" => return settings.allow_mz,
        _ => (),
    }
    let mut i = 0;
    let left_string;
    if char(cond, i) == '(' {
        let mut depth = 1;
        while depth > 0 {
            i += 1;
            if char(cond, i) == '(' {
                depth += 1;
            } else if char(cond, i) == ')' {
                depth -= 1;
            }
        }
        left_string = slice(cond, 1, i);
        i += 1;
        if i as usize == cond.len() {
            return check_conditions(left_string, settings);
        }
    } else {
        while !"|&".contains(char(cond, i)) {
            i += 1;
        }
        left_string = slice(cond, 0, i).trim();
    }
    let operator = char(slice_(cond, i).trim(), 0);
    let right_string = slice_(cond, i + 1).trim();
    let left_side = check_conditions(left_string, settings);
    let right_side = check_conditions(right_string, settings);
    match operator {
        '|' => left_side || right_side,
        '&' => left_side && right_side,
        _ => panic!("universe broke"),
    }
}

fn both(test: Vec<&str>) {
    let settings = Settings {
        generate_cmevla: is_consonant(char(test[0], -1)),
        ..Settings::default()
    };
    let lujvo = test[0];
    let expect = test[1];
    println!("\n{lujvo}\nkatna - expect: {expect}");
    let tanru = get_veljvo(lujvo, &settings)
        .unwrap_or_else(|e| vec![e.to_string()])
        .join(" ");
    println!("        actual: {tanru}");
    assert_eq!(expect, tanru);
    let tanru = test[1];
    let expect = test[0];
    println!("{tanru}\nzbasu - expect: {expect}");
    let lujvo = get_lujvo(tanru, &settings).unwrap_or_else(|e| e.to_string());
    println!("        actual: {lujvo}");
    assert_eq!(expect, lujvo);
}
fn zba(tanru: &str, expect: &str, e_score: i32, e_indices: &str, settings: &Settings) {
    println!("\n{tanru}\nzbasu - expect: {expect}");
    let lujvo = get_lujvo(tanru, settings).unwrap_or_else(|e| e.to_string());
    println!("        actual: {lujvo}");
    assert_eq!(expect, lujvo);
    if e_score != 0 {
        println!("score - expect: {e_score}");
        let score = get_lujvo_with_analytics(tanru, settings).unwrap().1;
        println!("        actual: {score}");
        assert_eq!(e_score, score);
    }
    if !e_indices.is_empty() {
        println!("index - expect: {e_indices}");
        let indices = get_lujvo_with_analytics(tanru, settings)
            .unwrap()
            .2
            .iter()
            .map(|i| format!("{}-{}", i[0], i[1]))
            .join(",");
        println!("        actual: {indices}");
        assert_eq!(e_indices, indices);
    }
}
fn zba_f(tanru: &str, settings: &Settings) {
    println!("\n{tanru}\nzbasu - expect: Err(...)");
    let lujvo = get_lujvo(tanru, settings);
    println!("        actual: {lujvo:?}");
    assert!(lujvo.is_err());
}
fn kaha(lujvo: &str, expect: &str, e_btype: &str, e_rafsi: &str, settings: &Settings) {
    println!("\n{lujvo}\nkatna - expect: {expect}");
    let tanru = selrafsi_list_from_rafsi_list(analyze_brivla(lujvo, settings).unwrap().1, settings)
        .unwrap_or_else(|e| vec![e.to_string()])
        .join(" ");
    println!("        actual: {tanru}");
    assert_eq!(expect, tanru);
    if !e_btype.is_empty() {
        let e_btype = match e_btype {
            "GISMU" => "Gismu",
            "LUJVO" => "Lujvo",
            "CMEVLA" => "mevla",
            "ZIhEVLA" => "Zihevla",
            "EXTENDED" => "ExtendedLujvo",
            _ => panic!("found btype {{{e_btype}}}"),
        };
        println!("btype - expect: {e_btype}");
        let btype = analyze_brivla(lujvo, settings).unwrap().0;
        println!("        actual: {btype}");
        assert_eq!(e_btype, btype.to_string());
    }
    if !e_rafsi.is_empty() {
        println!("rafsi - expect: {e_rafsi}");
        let rafsi = analyze_brivla(lujvo, settings)
            .unwrap()
            .1
            .iter()
            .filter(|r| !HYPHENS.contains(&r.as_str()))
            .join(" ");
        println!("        actual: {rafsi}");
        assert_eq!(e_rafsi, rafsi);
    }
}
fn kaha_f(lujvo: &str, settings: &Settings) {
    println!("\n{lujvo}\nkatna - expect: Err(...)");
    let tanru = get_veljvo(lujvo, settings);
    println!("        actual: {tanru:?}");
    assert!(tanru.is_err());
}

#[test]
fn t_basic() {
    let file = fs::read_to_string("../tests/basic_test_list.tsv").unwrap();
    let tests = file.lines().map(|l| l.split('\t').collect_vec());
    for test in tests {
        if test.len() == 2 {
            both(test);
        } else if test.len() == 3 {
            if test[2] == "CMEVLA" {
                both(test);
            } else if test[2] == "JVOZBA" {
                let settings = Settings::default();
                if test[0] != "FAIL" {
                    zba(test[1], test[0], 0, "", &settings);
                } else {
                    zba_f(test[1], &settings);
                }
            } else if test[2] == "KATNA" {
                let settings = Settings {
                    generate_cmevla: is_consonant(char(test[0], -1)),
                    ..Settings::default()
                };
                if test[1] != "FAIL" {
                    kaha(test[0], test[1], "", "", &settings);
                } else {
                    kaha_f(test[0], &settings);
                }
            } else {
                panic!("found a test with instruction {}", test[2]);
            }
        } else if !test[0].starts_with('#') && !test[0].is_empty() {
            panic!("found a test with length {}", test.len());
        }
    }
}

#[test]
fn t_zba() {
    let file = fs::read_to_string("../tests/jvozba_test_list.tsv").unwrap();
    let tests = file
        .lines()
        .map(|l| l.split('\t').collect_vec())
        .collect_vec();
    let mut tests2 = vec![];
    for mut test in tests {
        if test[0].starts_with('#') || test[0].is_empty() {
            continue;
        }
        let tanru = test[0];
        let c = test[1];
        while test.len() > 3 {
            let cond = test.remove(2);
            let expect = test.remove(2);
            let score = if test.len() >= 3 && !test[2].starts_with("(") {
                test.remove(2)
            } else {
                ""
            };
            let indices = if test.len() >= 3 && !test[2].starts_with("(") {
                test.remove(2)
            } else {
                ""
            };
            tests2.push(vec![tanru, c, cond, expect, score, indices]);
        }
    }
    let mut last = "";
    let mut last_c = "";
    let mut non_else = vec![];
    for test in tests2 {
        let (tanru, c, cond, e_lujvo, e_score, e_indices) =
            test.iter().cloned().collect_tuple().unwrap();
        if tanru != last {
            last = tanru;
            last_c = c;
            non_else = vec![];
        }
        if c != last_c {
            last_c = c;
        }
        if cond != "ELSE" {
            non_else.push(cond);
        }
        for settings in SETTINGS_ITERATOR.iter() {
            if c == "C" && !settings.generate_cmevla || c.is_empty() && settings.generate_cmevla {
                continue;
            }
            if non_else.iter().any(|n| check_conditions(n, settings))
                || !check_conditions(cond, settings)
            {
                continue;
            }
            println!("\nsettings: {settings:?}\ncond: {c} / {cond}\nprev: {non_else:?}");
            if e_lujvo == "NONE" {
                zba_f(test[0], settings);
                continue;
            } else {
                zba(
                    tanru,
                    e_lujvo,
                    e_score.parse().unwrap_or(0),
                    e_indices,
                    settings,
                );
            }
        }
    }
}

#[test]
fn t_kaha() {
    let file = fs::read_to_string("../tests/katna_test_list.tsv").unwrap();
    let tests = file
        .lines()
        .map(|l| l.split('\t').collect_vec())
        .collect_vec();
    let mut tests2 = vec![];
    for mut test in tests {
        if test[0].starts_with('#') || test[0].is_empty() {
            continue;
        }
        let lujvo = test[0];
        while test.len() > 2 {
            let cond = test.remove(1);
            let btype = test.remove(1);
            let rafsi = if test.len() >= 2 && !test[1].starts_with("(") {
                test.remove(1)
            } else {
                ""
            };
            let tanru = if test.len() >= 2 && !test[1].starts_with("(") {
                test.remove(1)
            } else {
                ""
            };
            let indices = if test.len() >= 2 && !test[1].starts_with("(") {
                test.remove(1)
            } else {
                ""
            };
            tests2.push(vec![lujvo, cond, btype, rafsi, tanru, indices]);
        }
    }
    let mut last = "";
    let mut non_else = vec![];
    for test in tests2 {
        let (lujvo, cond, e_btype, e_rafsi, e_tanru, e_indices) =
            test.iter().cloned().collect_tuple().unwrap();
        if lujvo != last {
            last = lujvo;
            non_else = vec![];
        }
        if cond != "ELSE" {
            non_else.push(cond);
        }
        for settings in SETTINGS_ITERATOR.iter() {
            if (e_btype == "CMEVLA") ^ settings.generate_cmevla {
                continue;
            }
            if non_else.iter().any(|n| check_conditions(n, settings))
                || !check_conditions(cond, settings)
            {
                continue;
            }
            println!("\nsettings: {settings:?}\ncond: {cond}\nprev: {non_else:?}");
            if e_btype == "NONE" {
                kaha_f(test[0], settings);
                continue;
            } else {
                kaha(lujvo, e_tanru, e_btype, e_rafsi, settings);
            }
        }
    }
}
