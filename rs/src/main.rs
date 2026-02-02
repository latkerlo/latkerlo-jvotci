use std::{
    env,
    io::{Write as _, stdin, stdout},
    process::exit,
};

use itertools::Itertools as _;
use latkerlo_jvotci::{
    ConsonantSetting::{Cluster, OneConsonant, TwoConsonants},
    RAFSI, Settings,
    YHyphenSetting::{AllowY, ForceY, Standard},
    analyze_brivla,
    cli_docs::{BOLD, CLI_INSTRUCTIONS, CYAN, GREEN, PINK, RED, RESET, TUI_INSTRUCTIONS},
    data::HYPHENS,
    get_lujvo, get_lujvo_with_analytics, get_veljvo,
    katna::{search_selrafsi_from_rafsi, selrafsi_list_from_rafsi_list},
    normalize, score_lujvo, strin,
};

#[allow(clippy::too_many_lines)]
fn main() {
    let mut settings = Settings::default();
    macro_rules! toggle {
        ($field:ident, $on:ident) => {
            settings.$field = if settings.$field == $on { Settings::default().$field } else { $on }
        };
    }
    let mut input = String::new();
    let mut lanli = false;
    // args
    let mut used_cli = false;
    let args = env::args().skip(1).collect_vec();
    let mut arginput = vec![];
    for (i, arg) in args.iter().enumerate() {
        if i == 0 && arg.starts_with('-') {
            if arg.contains('h') || arg == "--help" {
                println!("{}", *CLI_INSTRUCTIONS);
                return;
            }
            if arg.contains('L') {
                lanli = true;
            }
            if arg.contains('-') {
                // noöp
            }
            let flags = arg.chars().filter(|&c| !"LZh-".contains(c)).collect::<String>();
            if let Ok(s) = flags.parse() {
                settings = s;
            } else {
                println!("{RED}invalid flags, see {BOLD}-h{RESET}");
                exit(1);
            }
        } else {
            if arg.starts_with('/') {
                println!(
                    "{RED}flags starting with {BOLD}/{RESET}{RED} can only be used in interactive \
                     mode{RESET}"
                );
                exit(1);
            }
            if (i == 0 || i == 1 && args[0].starts_with('-')) && args[i..].len() == 1 {
                lanli = true;
                let arg = normalize(arg);
                if let Some(selrafsi) = search_selrafsi_from_rafsi(&arg) {
                    println!("{PINK}{{{arg}}} is a rafsi of {{{selrafsi}}}{RESET}");
                }
                if let Some(rafsi) = RAFSI.iter().find(|(s, _)| *s == &arg).map(|(_, r)| r) {
                    let rafsi = rafsi.iter().join(" ");
                    println!("{PINK}{{{arg}}} has rafsi {{{rafsi}}}{RESET}");
                }
            }
            arginput.push(arg);
        }
    }
    if !arginput.is_empty() {
        input = arginput.clone().into_iter().join(" ");
        used_cli = true;
    }
    // interactive
    loop {
        if !used_cli {
            input.clear();
            print!(
                "\n{CYAN}{} {settings}{RESET}\nenter a {}: {BOLD}{}",
                if lanli { "lanli" } else { "zbasu" },
                if lanli { "brivla" } else { "tanru" },
                if used_cli { input.clone() + "\n" } else { String::new() }
            );
            stdout().flush().unwrap();
            stdin().read_line(&mut input).expect("failed to read stdin");
        }
        print!("{RESET}");
        stdout().flush().unwrap();
        input = input.trim().to_string();
        if let Some(arg) = input.strip_prefix('/') {
            if arg.contains('q') {
                return;
            }
            if arg.contains('h') {
                println!("{}", *TUI_INSTRUCTIONS);
                continue;
            }
            if arg.len() == 1 {
                match strin!(arg, 0) {
                    'Z' => lanli = false,
                    'L' => lanli = true,
                    'c' => settings.generate_cmevla ^= true,
                    'r' => settings.exp_rafsi ^= true,
                    'g' => settings.glides ^= true,
                    'z' => settings.allow_mz ^= true,
                    'S' => settings.y_hyphens = Standard,
                    'A' => toggle!(y_hyphens, AllowY),
                    'F' => toggle!(y_hyphens, ForceY),
                    'C' => settings.consonants = Cluster,
                    '2' => toggle!(consonants, TwoConsonants),
                    '1' => toggle!(consonants, OneConsonant),
                    _ => println!("{RED}invalid single flag, see {BOLD}/h{RESET}"),
                }
                continue;
            }
            if arg.contains('L') {
                lanli = true;
            } else if arg.contains('Z') {
                lanli = false;
            }
            let flags = arg.chars().filter(|&c| !"LZh-".contains(c)).collect::<String>();
            if let Ok(s) = flags.parse() {
                settings = s;
            } else {
                println!("{RED}invalid flags, see {BOLD}/h{RESET}");
            }
            continue;
        }
        if lanli {
            let res = analyze_brivla(&input, &settings);
            if let Err(e) = res {
                println!("{RED}{e}{RESET}");
                if used_cli {
                    exit(1);
                }
                continue;
            }
            let (analysis, hyphens) = res.unwrap();
            println!(
                "{CYAN}{}\n{}{}{GREEN}{}{RESET}",
                analysis.to_string().to_lowercase().replace("dl", "d l"),
                if hyphens.join(" ") == input { String::new() } else { hyphens.join(" ") + "\n" },
                score_lujvo(&input, &settings)
                    .map_or_else(|_| String::new(), |score| score.to_string() + "\n"),
                selrafsi_list_from_rafsi_list(&hyphens, &settings).unwrap().into_iter().join(" ")
            );
            let veljvo = get_veljvo(&input, &settings);
            if let Err(e) = veljvo {
                println!("{RED}{e}{RESET}");
                if used_cli {
                    exit(1);
                }
                continue;
            }
            let veljvo = veljvo.unwrap().join(" ");
            let best = get_lujvo_with_analytics(&veljvo, &settings);
            if let Ok((best_lujvo, best_score, _)) = best
                && normalize(&input) != best_lujvo
            {
                let best_hyphens =
                    analyze_brivla(&best_lujvo, &settings).map(|(_, h)| h).unwrap_or_default();
                let input_hyphens = hyphens.clone();
                print!("{CYAN}best: {GREEN}");
                let mut m = 0;
                let mut b = 0;
                let mut diverged = false;
                while m < input_hyphens.len() && b < best_hyphens.len() {
                    let mabla_curr = input_hyphens.get(m).map_or("", String::as_str);
                    let best_curr = best_hyphens.get(b).map_or("", String::as_str);
                    if HYPHENS.contains(&mabla_curr) {
                        if !HYPHENS.contains(&best_curr) {
                            if best_curr == input_hyphens.get(m + 1).map_or("", String::as_str)
                                && !diverged
                            {
                                print!("{RED}-{GREEN}");
                            }
                            m += 1;
                            continue;
                        }
                        if mabla_curr == best_curr {
                            print!("{best_curr}");
                            m += 1;
                            b += 1;
                            continue;
                        }
                    } else if HYPHENS.contains(&best_curr) {
                        print!("{RED}{best_curr}{GREEN}");
                        b += 1;
                        diverged = true;
                        continue;
                    }
                    if mabla_curr == best_curr {
                        print!("{best_curr}");
                        diverged = false;
                    } else {
                        print!("{RED}{best_curr}{GREEN}");
                        diverged = true;
                    }
                    m += 1;
                    b += 1;
                }
                while b < best_hyphens.len() {
                    print!("{}", best_hyphens[b]);
                    b += 1;
                }
                println!("{RESET} {CYAN}({best_score}){RESET}");
                if score_lujvo(&input, &settings).unwrap() < best_score {
                    println!(
                        "{RED}hm, the 'best' lujvo actually has a higher score? that can't be \
                         good...{RESET}"
                    );
                    exit(1);
                }
            }
        } else {
            let res = get_lujvo(&input, &settings);
            println!(
                "{}{}{RESET}",
                if res.is_err() { RED } else { GREEN },
                res.clone().unwrap_or_else(|e| e.to_string())
            );
            if res.is_err() && used_cli {
                exit(1);
            }
        }
        if used_cli && !arginput.is_empty() {
            return;
        }
        used_cli = false;
    }
}
