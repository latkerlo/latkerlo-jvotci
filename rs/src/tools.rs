use crate::{
    katna::jvokaha2,
    tarmi::{is_consonant, is_gismu},
};
use regex::Regex;

pub fn char(s: &str, n: usize) -> char {
    s.chars().nth(n).unwrap_or_default()
}
/// (supports negative indices)
pub fn slice(s: &str, a: isize, b: isize) -> String {
    let mut a = if a >= 0 {
        a as usize
    } else {
        s.len() - (-a as usize)
    };
    let mut b = if b >= 0 {
        b as usize
    } else {
        s.len() - (-b as usize)
    };
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    let x = s
        .chars()
        .enumerate()
        .filter(|&(i, _)| a <= i && i < b)
        .map(|(_, c)| c)
        .collect::<String>();
    x
}

pub fn is_gismu_or_lujvo(s: &str) -> bool {
    if is_gismu(s) {
        true
    } else {
        jvokaha2(s).is_ok()
    }
}

/// Splits off a cmavo (particle). Assumes it does need to be split and the remainder is valid Lojban. Returns an Err on non-Lojban characters.
pub fn split_one_cmavo(s: &str) -> Result<[&str; 2], String> {
    let mut i = 0;
    let mut will_end = false;
    while i < s.len() {
        if i + 2 < s.len()
            && ["ai", "ei", "oi", "au"].contains(&&s[i..i + 2])
            && !"aeiouy".contains(char(s, i + 2))
        {
            i += 2;
            will_end = true;
        } else if i + 1 < s.len() && "iu".contains(char(s, i)) && "aeiouy".contains(char(s, i + 1))
        {
            if will_end {
                break;
            }
            i += 2;
            will_end = true;
        } else if "aeiouy".contains(char(s, i)) {
            i += 1;
            will_end = true;
        } else if char(s, i) == '\'' {
            i += 1;
            will_end = false;
        } else if "bcdfgjklmnprstvxz".contains(char(s, i)) {
            if i == 0 {
                i += 1;
            } else {
                break;
            }
        } else {
            return Err(format!(
                "non-Lojban character {{{}}} in {{{}}} at index {{{}}}",
                char(s, i),
                s,
                i
            ));
        }
    }
    Ok([&s[0..i], &s[i..]])
}

/// Chops all the cmavo off the front and returns a list of the resulting words
pub fn split_words(s: &str) -> Vec<String> {
    if s.is_empty() {
        return vec![];
    }
    if is_consonant(s.chars().last().unwrap()) {
        return vec![s.to_string()];
    }
    let first5 = slice(&s.replace(['y', '\''], ""), 0, 5);
    let cluster = Regex::new("[bcdfgjklmnprstvxz]{2}").unwrap().find(&first5).map_or(-1, |m| m.start() as isize);
    if cluster == -1 {
        let [cmavo, remainder] = split_one_cmavo(s).unwrap();
        return [
            vec![cmavo.to_string()],
            split_words(remainder),
        ]
        .concat();
    }
    if is_gismu_or_lujvo(s) {
        return vec![s.to_string()];
    }
    let mut i = 0;
    for c in s.chars() {
        if i >= cluster {
            break;
        }
        if !['y', '\''].contains(&c) {
            i += 1;
        }
    }
    if is_gismu_or_lujvo(&slice(s, i, s.len() as isize)) {
        vec![slice(s, 0, i), slice(s, i, s.len() as isize)]
    } else {
        vec![s.to_string()]
    }
}
