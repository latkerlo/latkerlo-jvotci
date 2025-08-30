#![allow(clippy::format_push_string)]
#![cfg(test)]

use std::{
    fs::{self, OpenOptions},
    io::Write as _,
    sync::LazyLock,
};

use itertools::Itertools as _;
use regex::Regex;

use crate::{
    data::HYPHENS,
    katna::selrafsi_list_from_rafsi_list,
    tarmi::{SETTINGS_ITERATOR, is_consonant},
    tools::{get_rafsi_indices, regex_replace_all},
    *,
};

const PRINT: bool = false;

// ported from py/tests/test_other.py
fn check_conditions(cond: &str, settings: Settings) -> bool {
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
    if strin!(cond, i) == '(' {
        let mut depth = 1;
        while depth > 0 {
            i += 1;
            if strin!(cond, i) == '(' {
                depth += 1;
            } else if strin!(cond, i) == ')' {
                depth -= 1;
            }
        }
        left_string = strsl!(cond, 1..i);
        i += 1;
        if i as usize == cond.len() {
            return check_conditions(left_string, settings);
        }
    } else {
        while !"|&".contains(strin!(cond, i)) {
            i += 1;
        }
        left_string = strsl!(cond, 0..i).trim();
    }
    let operator = strin!(strsl!(cond, i..).trim(), 0);
    let right_string = strsl!(cond, i + 1..).trim();
    let left_side = check_conditions(left_string, settings);
    let right_side = check_conditions(right_string, settings);
    match operator {
        '|' => left_side || right_side,
        '&' => left_side && right_side,
        _ => panic!("universe broke"),
    }
}

static STRIP_ANSI: LazyLock<Regex> = LazyLock::new(|| Regex::new("\x1b\\[\\d*m").unwrap());

fn both(test: &[&str]) -> i32 {
    assert!(test.len() > 1);
    let settings =
        Settings { generate_cmevla: is_consonant(strin!(test[0], -1)), ..Settings::default() };
    let lujvo = test[0];
    let expect = test[1];
    let mut output = format!("\n\x1b[1m{lujvo}\x1b[m");
    let tanru = get_veljvo(lujvo, &settings)
        .unwrap_or_else(|e| vec![format!("{:?}", Err::<Vec<String>, _>(e))])
        .join(" ");
    output += &if expect == tanru {
        format!("\nkatna    - \x1b[92m{tanru}\x1b[m")
    } else {
        format!("\nkatna    - \x1b[91m{tanru}\x1b[m\nexpected - {expect}")
    };
    let tanru = test[1];
    let expect = test[0];
    let lujvo =
        get_lujvo(tanru, &settings).unwrap_or_else(|e| format!("{:?}", Err::<String, _>(e)));
    output += &format!("\nzbasu    - \x1b[9{}m{lujvo}\x1b[m", (expect == lujvo) as u8 + 1);
    let ohno = output.contains("[91m");
    if PRINT || ohno {
        println!("{output}");
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(if ohno { "test_diagnostics/bad.txt" } else { "test_diagnostics/good.txt" })
        .unwrap();
    file.write_all(regex_replace_all(&STRIP_ANSI, &(output + "\n"), "").as_bytes()).unwrap();
    ohno as i32
}
fn zba(tanru: &str, expect: &str, e_score: i32, e_indices: &str, settings: Settings) -> i32 {
    let mut output = format!("\n\x1b[1m{tanru}\x1b[m");
    let lujvo = get_lujvo(tanru, &settings);
    if lujvo.is_err() {
        output += &format!(
            "\nzbasu    - \x1b[91m{lujvo:?}\x1b[m\nexpected - {expect}{}",
            if Settings::default() == settings {
                String::new()
            } else {
                format!("\nsettings - {settings}")
            }
        );
        println!("{output}");
        let mut file =
            OpenOptions::new().append(true).create(true).open("test_diagnostics/bad.txt").unwrap();
        file.write_all(regex_replace_all(&STRIP_ANSI, &(output + "\n"), "").as_bytes()).unwrap();
        return 1;
    }
    let lujvo = lujvo.unwrap_or_else(|e| format!("{:?}", Err::<String, _>(e)));
    output += &if expect == lujvo {
        format!("\nzbasu    - \x1b[92m{lujvo}\x1b[m")
    } else {
        format!("\nzbasu    - \x1b[91m{lujvo}\x1b[m\nexpected - {expect}")
    };
    if e_score != 0 {
        let score = get_lujvo_with_analytics(tanru, &settings).unwrap().1;
        output += &if e_score == score {
            format!("\nscore    - \x1b[92m{score}\x1b[m")
        } else {
            format!("\nscore    - \x1b[91m{score}\x1b[m\nexpected - {e_score}")
        };
    }
    if !e_indices.is_empty() {
        let indices = get_lujvo_with_analytics(tanru, &settings)
            .unwrap()
            .2
            .iter()
            .map(|i| format!("{}-{}", i[0], i[1]))
            .join(",");
        output += &if e_indices == indices {
            format!("\nindices  - \x1b[92m{indices}\x1b[m")
        } else {
            format!("\nindices  - \x1b[91m{indices}\x1b[m\nexpected - {e_indices}")
        }
    }
    if Settings::default() != settings {
        output += &format!("\nsettings - {settings}");
    }
    let ohno = output.contains("[91m");
    if PRINT || ohno {
        println!("{output}");
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(if ohno { "test_diagnostics/bad.txt" } else { "test_diagnostics/good.txt" })
        .unwrap();
    file.write_all(regex_replace_all(&STRIP_ANSI, &(output + "\n"), "").as_bytes()).unwrap();
    ohno as i32
}
fn zba_f(tanru: &str, settings: Settings) -> i32 {
    let mut output = format!("\n\x1b[1m{tanru}\x1b[m");
    let lujvo = get_lujvo(tanru, &settings);
    output += &if lujvo.is_err() {
        format!("\nzbasu    - \x1b[93m{lujvo:?}\x1b[m")
    } else {
        format!("\nzbasu    - \x1b[91m{lujvo:?}\x1b[m")
    };
    if Settings::default() != settings {
        output += &format!("\nsettings - {settings}");
    }
    let ohno = output.contains("[91m");
    if PRINT || ohno {
        println!("{output}");
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(if ohno { "test_diagnostics/bad.txt" } else { "test_diagnostics/check.txt" })
        .unwrap();
    file.write_all(regex_replace_all(&STRIP_ANSI, &(output + "\n"), "").as_bytes()).unwrap();
    ohno as i32
}
fn kaha(
    lujvo: &str,
    expect: &str,
    e_btype: &str,
    e_rafsi: &str,
    e_indices: &str,
    settings: Settings,
) -> i32 {
    let mut output = format!("\n\x1b[1m{lujvo}\x1b[m");
    let pre_tanru = analyze_brivla(lujvo, &settings);
    if pre_tanru.is_err() {
        output += &format!(
            "\nkatna    - \x1b[91m{pre_tanru:?}\x1b[m\nexpected - {expect}{}",
            if Settings::default() == settings {
                String::new()
            } else {
                format!("\nsettings - {settings}")
            }
        );
        println!("{output}");
        let mut file =
            OpenOptions::new().append(true).create(true).open("test_diagnostics/bad.txt").unwrap();
        file.write_all(regex_replace_all(&STRIP_ANSI, &(output + "\n"), "").as_bytes()).unwrap();
        return 1;
    }
    let tanru = selrafsi_list_from_rafsi_list(&pre_tanru.as_ref().unwrap().1.clone(), &settings)
        .unwrap_or_else(|e| vec![e.to_string()])
        .join(" ");
    output += &if expect == tanru {
        format!("\nkatna    - \x1b[92m{tanru}\x1b[m")
    } else {
        format!("\nkatna    - \x1b[91m{tanru}\x1b[m\nexpected - {expect}")
    };
    if !e_btype.is_empty() {
        let e_btype = match e_btype {
            "GISMU" => "gismu",
            "LUJVO" => "lujvo",
            "CMEVLA" => "cmevla",
            "ZIhEVLA" => "zi'evla",
            "EXTENDED" => "extended lujvo",
            _ => panic!("found btype {{{e_btype}}}"),
        };
        let btype = &normalize(&pre_tanru.as_ref().unwrap().0.to_string()).replace("dl", "d l");
        output += &if e_btype == btype {
            format!("\nbrivtype - \x1b[92m{btype}\x1b[m")
        } else {
            format!("\nbrivtype - \x1b[91m{btype}\x1b[m\nexpected - {e_btype}")
        };
    }
    if !e_rafsi.is_empty() {
        let rafsi = pre_tanru
            .as_ref()
            .unwrap()
            .1
            .iter()
            .filter(|r| !HYPHENS.contains(&r.as_str()))
            .join(" ");
        output += &if e_rafsi == rafsi {
            format!("\nrafsi    - \x1b[92m{rafsi}\x1b[m")
        } else {
            format!("\nrafsi    - \x1b[91m{rafsi}\x1b[m\nexpected - {e_rafsi}")
        };
    }
    if !e_indices.is_empty() {
        let indices =
            get_rafsi_indices(&pre_tanru.unwrap().1.iter().map(String::as_str).collect_vec())
                .iter()
                .map(|i| format!("{}-{}", i[0], i[1]))
                .join(",");
        output += &if e_indices == indices {
            format!("\nindices  - \x1b[92m{indices}\x1b[m")
        } else {
            format!("\nindices  - \x1b[91m{indices}\x1b[m\nexpected - {e_indices}")
        };
    }
    if Settings::default() != settings {
        output += &format!("\nsettings - {settings}");
    }
    let ohno = output.contains("[91m");
    if PRINT || ohno {
        println!("{output}");
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(if ohno { "test_diagnostics/bad.txt" } else { "test_diagnostics/good.txt" })
        .unwrap();
    file.write_all(regex_replace_all(&STRIP_ANSI, &(output + "\n"), "").as_bytes()).unwrap();
    ohno as i32
}
fn kaha_f(lujvo: &str, settings: Settings) -> i32 {
    let mut output = format!("\n\x1b[1m{lujvo}\x1b[m");
    let tanru = get_veljvo(lujvo, &settings);
    output += &if tanru.is_err() {
        format!("\nkatna    - \x1b[93m{tanru:?}\x1b[m")
    } else {
        format!(
            "\nkatna    - \x1b[91m{tanru:?}\x1b[m\nraf+hyph - \x1b[91m{}\x1b[m",
            analyze_brivla(lujvo, &settings).unwrap().1.join(" ")
        )
    };
    if Settings::default() != settings {
        output += &format!("\nsettings - {settings}");
    }
    let ohno = output.contains("[91m");
    if PRINT || ohno {
        println!("{output}");
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(if ohno { "test_diagnostics/bad.txt" } else { "test_diagnostics/check.txt" })
        .unwrap();
    file.write_all(regex_replace_all(&STRIP_ANSI, &(output + "\n"), "").as_bytes()).unwrap();
    ohno as i32
}

#[test]
fn t_basic() {
    fs::create_dir_all("test_diagnostics").unwrap();
    let file = fs::read_to_string("../tests/basic_test_list.tsv").unwrap();
    let tests = file.lines().map(|l| l.split('\t').collect_vec()).collect_vec();
    let mut ohnos = 0;
    for test in tests.clone() {
        if test.len() == 2 {
            ohnos += both(&test);
        } else if test.len() == 3 {
            if test[2] == "CMEVLA" {
                ohnos += both(&test);
            } else if test[2] == "JVOZBA" {
                let settings = Settings::default();
                if test[0] == "FAIL" {
                    ohnos += zba_f(test[1], settings);
                } else {
                    ohnos += zba(test[1], test[0], 0, "", settings);
                }
            } else if test[2] == "KATNA" {
                let settings = Settings {
                    generate_cmevla: is_consonant(strin!(test[0], -1)),
                    ..Settings::default()
                };
                if test[1] == "FAIL" {
                    ohnos += kaha_f(test[0], settings);
                } else {
                    ohnos += kaha(test[0], test[1], "", "", "", settings);
                }
            } else {
                panic!("found a test with instruction {}", test[2]);
            }
        } else if !test[0].starts_with('#') && !test[0].is_empty() {
            panic!("found a test with length {}", test.len());
        }
    }
    if ohnos > 0 {
        println!();
        panic!(
            "\x1b[92m{:5}\x1b[m/{} tests passed (see check.txt to ensure the NONE tests failed \
             for the right reasons)\n\x1b[91m{ohnos:5}\x1b[m/{1} tests failed",
            tests.len() - ohnos as usize,
            tests.len()
        );
    }
}

#[test]
fn t_zba() {
    fs::create_dir_all("test_diagnostics").unwrap();
    let file = fs::read_to_string("../tests/jvozba_test_list.tsv").unwrap();
    let tests = file.lines().map(|l| l.split('\t').collect_vec()).collect_vec();
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
            let score =
                if test.len() >= 3 && !test[2].starts_with('(') { test.remove(2) } else { "" };
            let indices =
                if test.len() >= 3 && !test[2].starts_with('(') { test.remove(2) } else { "" };
            tests2.push(vec![tanru, c, cond, expect, score, indices]);
        }
    }
    let mut last = "";
    let mut last_c = "";
    let mut non_else = vec![];
    let mut ohnos = 0;
    let mut i = 0;
    for test in tests2.clone() {
        let (tanru, c, cond, e_lujvo, e_score, e_indices) =
            test.iter().copied().collect_tuple().unwrap();
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
        for &settings in SETTINGS_ITERATOR.iter() {
            if (c == "C") ^ settings.generate_cmevla
                || non_else.iter().enumerate().any(|(i, n)| {
                    !(cond != "ELSE" && i == non_else.len() - 1) && check_conditions(n, settings)
                })
                || !check_conditions(cond, settings)
            {
                continue;
            }
            i += 1;
            if e_lujvo == "NONE" {
                ohnos += zba_f(test[0], settings);
            } else {
                ohnos += zba(tanru, e_lujvo, e_score.parse().unwrap_or(0), e_indices, settings);
            }
        }
    }
    if ohnos > 0 {
        println!();
        panic!(
            "\x1b[92m{:5}\x1b[m/{i} tests passed (see check.txt to ensure the NONE tests failed \
             for the right reasons)\n\x1b[91m{ohnos:5}\x1b[m/{i} tests failed",
            i - ohnos,
        );
    }
}

#[test]
fn t_kaha() {
    fs::create_dir_all("test_diagnostics").unwrap();
    let file = fs::read_to_string("../tests/katna_test_list.tsv").unwrap();
    let tests = file.lines().map(|l| l.split('\t').collect_vec()).collect_vec();
    let mut tests2 = vec![];
    for mut test in tests {
        if test[0].starts_with('#') || test[0].is_empty() {
            continue;
        }
        let lujvo = test[0];
        while test.len() > 2 {
            let cond = test.remove(1);
            let btype = test.remove(1);
            let rafsi =
                if test.len() >= 2 && !test[1].starts_with('(') { test.remove(1) } else { "" };
            let tanru =
                if test.len() >= 2 && !test[1].starts_with('(') { test.remove(1) } else { "" };
            let indices =
                if test.len() >= 2 && !test[1].starts_with('(') { test.remove(1) } else { "" };
            tests2.push(vec![lujvo, cond, btype, rafsi, tanru, indices]);
        }
    }
    let mut last = "";
    let mut non_else = vec![];
    let mut ohnos = 0;
    let mut i = 0;
    for test in tests2 {
        let (lujvo, cond, e_btype, e_rafsi, e_tanru, e_indices) =
            test.iter().copied().collect_tuple().unwrap();
        if lujvo != last {
            last = lujvo;
            non_else = vec![];
        }
        if cond != "ELSE" {
            non_else.push(cond);
        }
        for &settings in SETTINGS_ITERATOR.iter() {
            if (e_btype == "CMEVLA") ^ settings.generate_cmevla
                || non_else.iter().enumerate().any(|(i, n)| {
                    !(cond != "ELSE" && i == non_else.len() - 1) && check_conditions(n, settings)
                })
                || !check_conditions(cond, settings)
            {
                continue;
            }
            i += 1;
            if e_btype == "NONE" {
                ohnos += kaha_f(test[0], settings);
            } else {
                ohnos += kaha(lujvo, e_tanru, e_btype, e_rafsi, e_indices, settings);
            }
        }
    }
    if ohnos > 0 {
        println!();
        panic!(
            "\x1b[92m{:5}\x1b[m/{i} tests passed (see check.txt to ensure the NONE tests failed \
             for the right reasons)\n\x1b[91m{ohnos:5}\x1b[m/{i} tests failed",
            i - ohnos,
        );
    }
}

#[test]
fn init() {
    fs::remove_dir_all("test_diagnostics").unwrap();
    fs::create_dir_all("test_diagnostics").unwrap();
}
