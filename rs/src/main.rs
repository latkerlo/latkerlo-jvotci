use itertools::Itertools;
use latkerlo_jvotci::*;
use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

fn main() {
    let mut settings = Settings::default();
    let mut settings_str = String::new();
    let mut input = String::new();
    let mut katna = false;
    loop {
        input.clear();
        print!(
            "\n\x1b[96m{} {settings}\x1b[m\nenter a {}: \x1b[93m",
            if katna { "katna" } else { "zbasu" },
            if katna { "lujvo" } else { "tanru" }
        );
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("failed to read stdin");
        print!("\x1b[m");
        stdout().flush().unwrap();
        input = input.trim().to_string();
        if input == "/katna" {
            katna = true;
        } else if input == "/zbasu" {
            katna = false;
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
            settings = Settings::from_str(&settings_str).unwrap();
            settings_str = settings.to_string();
        } else if katna {
            let res = get_veljvo(&input, &settings);
            println!(
                "{}{}\x1b[m",
                if res.is_err() { "\x1b[91m" } else { "\x1b[92m" },
                res.unwrap_or_else(|e| vec![e.to_string()]).join(" ")
            );
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
