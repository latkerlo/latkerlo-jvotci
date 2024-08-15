use humantime::format_duration;
use latkerlo_jvotci::*;
use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let settings = &Settings::default();
    let mut the = HashMap::new();
    let start = Instant::now();
    const END: u32 = 100000; // takes ~10m with `--release`
    for i in 0..END {
        let eta = if i != 0 {
            &format_duration(start.elapsed() / i * (END - i)).to_string()
        } else {
            ""
        };
        println!("{i}    {}", eta);
        fs::write("eta.txt", eta).unwrap();
        let k = get_lujvo("condi djedi", settings).unwrap();
        if the.contains_key(&k) {
            the.insert(k.clone(), the.get(&k).unwrap() + 1);
        } else {
            the.insert(k, 1);
        }
    }
    println!("{the:?}");
    println!("{}\n", format_duration(start.elapsed()));
}
