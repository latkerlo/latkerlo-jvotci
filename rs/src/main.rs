use std::{
    env,
    io::{Write as _, stdin, stdout},
    process::exit,
    str::FromStr as _,
};

use itertools::Itertools as _;
use latkerlo_jvotci::{
    ConsonantSetting::{Cluster, OneConsonant, TwoConsonants},
    RAFSI, Settings,
    YHyphenSetting::{AllowY, ForceY, Standard},
    analyze_brivla, get_lujvo,
    katna::{search_selrafsi_from_rafsi, selrafsi_list_from_rafsi_list},
    normalize, score_lujvo, strin,
};

const VERSION: &str = "\x1b[93;1mlatkerlo-jvotci v2.4.2510\x1b[m";

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
                let instructions = VERSION.to_string()
                    + "\x1b[;96m cli edition\n"
                    + "usage: \x1b[;1mjvotci \x1b[m(\x1b[1;3mflags\x1b[m) \
                       (\x1b[1;3minput\x1b[m...)\n"
                    + "\x1b[;96mflags:\x1b[m (\x1b[92m*\x1b[m = default)\n"
                    + "  \x1b[95mmodes:\x1b[m\n"
                    + "    \x1b[1m-Z\x1b[;92m* >1 word\x1b[m convert tanru/rafsi to lujvo\n"
                    + "    \x1b[1m-L\x1b[;92m*  1 word\x1b[m analyze brivla, decompose if \
                       possible\n"
                    + "    \x1b[1m-h\x1b[m  print this help text\n"
                    + "  you may need to pass \x1b[1m-Z\x1b[m explicitly if the input starts with \
                       a formatted rafsi like {-vla-},\n"
                    + "  since otherwise it will get interpreted as a flag.\n"
                    + "  alternatively, use \x1b[1m--\x1b[m to separate flags from input\n"
                    + "  \x1b[95mtoggles:\x1b[m\n"
                    + "    \x1b[1m-c\x1b[m  generate a cmevla\n"
                    + "    \x1b[1m-r\x1b[m  allow any cmavo to be a rafsi\n"
                    + "    \x1b[1m-g\x1b[m  treat glides as consonants\n"
                    + "    \x1b[1m-z\x1b[m  allow {mz}\n"
                    + "  \x1b[95mhyphens:\x1b[m (up to 1 option allowed)\n"
                    + "    \x1b[1m-S\x1b[;92m*\x1b[m CLL hyphen rules + {'y} etc for zi'evla\n"
                    + "    \x1b[1m-A\x1b[m  allow {'y} etc hyphens in place of {r} or {n}\n"
                    + "    \x1b[1m-F\x1b[m  force {'y} and treat words with {r} or {n} as \
                       zi'evla\n"
                    + "  hyphens that are present when they don't need to be are always \
                       permitted\n"
                    + "  \x1b[95mconsonants:\x1b[m (up to 1 option allowed)\n"
                    + "    \x1b[1m-C\x1b[;92m*\x1b[m require a consonant cluster\n"
                    + "    \x1b[1m-2\x1b[m  minimum 2 consonants\n"
                    + "    \x1b[1m-1\x1b[m  minimum 1 consonant\n"
                    + "multiple flags \x1b[3mmust\x1b[m be grouped together if present, and they \
                       can be set in any order, e.g. \x1b[1m-Zgc1rA\x1b[m\n"
                    + "\x1b[96minput:\x1b[m a string to do things to\n"
                    + "if it's multiple words they will get concatenated.\n"
                    + "if a rafsi is entered, its selrafsi will be shown, and vice versa.\n"
                    + "if no input is provided, an interactive mode is used. help for that can be \
                       printed with \x1b[1m/h\x1b[m";
                println!("{instructions}");
                return;
            }
            if arg.contains('L') {
                lanli = true;
            }
            if arg.contains('-') {
                // noöp
            }
            let flags =
                arg.chars().filter(|&c| !"LZh-".chars().any(|f| f == c)).collect::<String>();
            if let Ok(s) = Settings::from_str(&flags) {
                settings = s;
            } else {
                println!("\x1b[91minvalid flags, see `-h`\x1b[m");
                exit(1);
            }
        } else {
            if arg.starts_with('/') {
                println!(
                    "\x1b[91mflags starting with / can only be used in interactive mode\x1b[m"
                );
                exit(1);
            }
            if (i == 0 || i == 1 && args[0].starts_with('-')) && args[i..].len() == 1 {
                lanli = true;
                let arg = normalize(arg);
                if let Some(selrafsi) = search_selrafsi_from_rafsi(&arg) {
                    println!("\x1b[95m{{{arg}}} is a rafsi of {{{selrafsi}}}\x1b[m");
                }
                if let Some(rafsi) = RAFSI.iter().filter(|(s, _)| *s == &arg).map(|(_, r)| r).next()
                {
                    let rafsi = rafsi.iter().join(" ");
                    println!("\x1b[95m{{{arg}}} has rafsi {{{rafsi}}}\x1b[m");
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
                "\n\x1b[96m{} {settings}\x1b[m\nenter a {}: \x1b[1m{}",
                if lanli { "lanli" } else { "zbasu" },
                if lanli { "brivla" } else { "tanru" },
                if used_cli { input.clone() + "\n" } else { String::new() }
            );
            stdout().flush().unwrap();
            stdin().read_line(&mut input).expect("failed to read stdin");
        }
        print!("\x1b[m");
        stdout().flush().unwrap();
        input = input.trim().to_string();
        if let Some(arg) = input.strip_prefix('/') {
            if arg.contains('q') {
                return;
            }
            if arg.contains('h') {
                let instructions = VERSION.to_string()
                    + "\x1b[96m interactive edition\n"
                    + "\x1b[96mflags:\x1b[m (\x1b[92m*\x1b[m = default)\n"
                    + "  \x1b[95mmodes:\x1b[m\n"
                    + "    \x1b[1m/Z\x1b[;92m*\x1b[m (\x1b[96mzbasu\x1b[m) convert tanru/rafsi to \
                       lujvo\n"
                    + "    \x1b[1m/L\x1b[m  (\x1b[96mlanli\x1b[m) analyze brivla, decompose if \
                       possible\n"
                    + "    \x1b[1m/h\x1b[m  print this help text\n"
                    + "    \x1b[1m/q\x1b[m  quit\n"
                    + "  \x1b[95mtoggles:\x1b[m\n"
                    + "    \x1b[1m/c\x1b[m  generate a cmevla\n"
                    + "    \x1b[1m/r\x1b[m  allow any cmavo to be a rafsi\n"
                    + "    \x1b[1m/g\x1b[m  treat glides as consonants\n"
                    + "    \x1b[1m/z\x1b[m  allow {mz}\n"
                    + "  \x1b[95mhyphens:\x1b[m (up to 1 option allowed)\n"
                    + "    \x1b[1m/S\x1b[;92m*\x1b[m CLL hyphen rules + {'y} etc for zi'evla\n"
                    + "    \x1b[1m/A\x1b[m  allow {'y} etc hyphens in place of {r} or {n}\n"
                    + "    \x1b[1m/F\x1b[m  force {'y} and treat words with {r} or {n} as \
                       zi'evla\n"
                    + "  hyphens that are present when they don't need to be are always \
                       permitted\n"
                    + "  \x1b[95mconsonants:\x1b[m (up to 1 option allowed)\n"
                    + "    \x1b[1m/C\x1b[;92m*\x1b[m require a consonant cluster\n"
                    + "    \x1b[1m/2\x1b[m  minimum 2 consonants\n"
                    + "    \x1b[1m/1\x1b[m  minimum 1 consonant\n"
                    + "single-character flags only toggle themselves.\n"
                    + "multiple flags can be set at once and in any order, e.g. \
                       \x1b[1m/Zgc1rA\x1b[m.\n"
                    + "to return to the default settings, use \x1b[1m/\x1b[m";
                println!("{instructions}");
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
                    _ => println!("\x1b[91minvalid single flag, see \x1b[1m/h\x1b[m"),
                }
                continue;
            }
            if arg.contains('L') {
                lanli = true;
            } else if arg.contains('Z') {
                lanli = false;
            }
            let flags =
                arg.chars().filter(|&c| !"LZh-".chars().any(|f| f == c)).collect::<String>();
            if let Ok(s) = Settings::from_str(&flags) {
                settings = s;
            } else {
                println!("\x1b[91minvalid flags, see \x1b[1m/h\x1b[m");
            }
            continue;
        }
        if lanli {
            let res = analyze_brivla(&input, &settings);
            if let Err(e) = res {
                println!("\x1b[91m{e}\x1b[m");
                if used_cli {
                    exit(1);
                }
            } else {
                let hyphens = res.clone().unwrap().1;
                println!(
                    "\x1b[96m{}\n{}{}\x1b[92m{}\x1b[m",
                    res.unwrap().0.to_string().to_lowercase().replace("dl", "d l"),
                    if hyphens.join(" ") == input {
                        String::new()
                    } else {
                        hyphens.join(" ") + "\n"
                    },
                    score_lujvo(&input, &settings)
                        .map_or_else(|_| String::new(), |score| score.to_string() + "\n"),
                    selrafsi_list_from_rafsi_list(&hyphens, &settings)
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
