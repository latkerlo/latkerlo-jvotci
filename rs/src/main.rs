use itertools::Itertools;
use katna::selrafsi_list_from_rafsi_list;
use latkerlo_jvotci::*;
use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

fn main() {
    let mut settings = Settings::default();
    let mut settings_str = String::new();
    let mut input = String::new();
    let mut lanli = false;
    loop {
        input.clear();
        print!(
            "\n\x1b[96m{} {settings}\x1b[m\nenter a {}: \x1b[93m",
            if lanli { "lanli" } else { "zbasu" },
            if lanli { "brivla" } else { "tanru" }
        );
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("failed to read stdin");
        print!("\x1b[m");
        stdout().flush().unwrap();
        input = input.trim().to_string();
        if input == "/lanli" {
            lanli = true;
        } else if input == "/zbasu" {
            lanli = false;
        } else if input == "/help" {
            let instructions = "\x1b[96mchange mode:\n".to_string()
                + "  /lanli - analyzes a brivla, converts to tanru if possible\n"
                + "  /zbasu - converts tanru to lujvo\n"
                + "flags (default is off):\n"
                + "  c - cmevla\n"
                + "  r - experimental rafsi\n"
                + "  g - treat glides as consonants\n"
                + "  z - allow `mz`\n"
                + "hyphens:\n"
                + "  S - [default] CLL hyphen rules\n"
                + "  A - allow `'y` etc\n"
                + "  F - force `'y` etc (no `r` or `n` hyphens)\n"
                + "consonants:\n"
                + "  C - [default] require a consonant cluster\n"
                + "  2 - min. 2 consonants\n"
                + "  1 - min. 1 consonant\n"
                + "multiple settings can be set at once, e.g. `/czF`\n"
                + "/quit to stop\x1b[m";
            println!("{instructions}");
        } else if input == "/quit" {
            return;
        } else if input.starts_with('/') {
            input = input[1..].to_string();
            #[allow(unused_assignments)]
            if input == "default" {
                input = settings_str.clone();
            }
            input = input.chars().sorted().dedup().collect();
            let mut new = String::new();
            for c in (input.clone() + &settings_str).chars().sorted().dedup() {
                if (input.contains(c) ^ settings_str.contains(c))
                    && !("SAF".chars().any(|x| x == c)
                        && settings_str.contains(c)
                        && input.chars().any(|i| "SAF".chars().any(|x| x == i)))
                    && !("C21".chars().any(|x| x == c)
                        && settings_str.contains(c)
                        && input.chars().any(|i| "C21".chars().any(|x| x == i)))
                {
                    new += &c.to_string();
                }
            }
            settings_str = new;
            if let Ok(s) = Settings::from_str(&settings_str) {
                settings = s;
            } else {
                println!("\x1b[91minvalid settings, see `/help`\x1b[m");
            }
            settings_str = settings.to_string();
        } else if lanli {
            let res = analyze_brivla(&input, &settings);
            if let Err(e) = res {
                println!("\x1b[91m{e}\x1b[m");
            } else {
                let hyphens = res.clone().unwrap().1;
                println!(
                    "\x1b[96m{}\n{}\n\x1b[92m{}\x1b[m",
                    res.unwrap()
                        .0
                        .to_string()
                        .to_lowercase()
                        .replace("dl", "d l"),
                    hyphens.join(" "),
                    selrafsi_list_from_rafsi_list(hyphens, &settings)
                        .unwrap()
                        .into_iter()
                        .join(" ")
                );
            }
        } else {
            let res = get_lujvo(&input, &settings);
            println!(
                "{}{}\x1b[m",
                if res.is_err() { "\x1b[91m" } else { "\x1b[92m" },
                res.unwrap_or_else(|e| e.to_string())
            );
        }
    }
}
