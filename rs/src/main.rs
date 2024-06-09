use humantime::format_duration;
use latkerlo_jvotci::*;
use std::{collections::HashMap, time::Instant};

fn main() {
    let settings = &Settings::default();
    let mut the = HashMap::new();
    let start = Instant::now();
    const END: u32 = 10000; // takes ~10m with `--release`
    for i in 0..END {
        println!(
            "{i}    {}",
            format_duration(start.elapsed() / if i != 0 { i } else { 1 } * (END - i))
        );
        let k = get_lujvo("condi djedi", settings).unwrap();
        if the.contains_key(&k) {
            the.insert(k.clone(), the.get(&k).unwrap() + 1);
        } else {
            the.insert(k, 1);
        }
    }
    println!("{the:?}");
    println!("{:?}", get_veljvo("jvo'ytci", settings)); // bad
    println!("{:?}", get_veljvo("lujvo'ytci", settings)); // ok
    println!("{:?}", get_veljvo("uajvo'ytci", settings)); // ok
    println!("{:?}", get_veljvo("latratyraty'ismu", settings)); // bad, la falls off
    println!("{:?}", get_veljvo("tratraty'ismu", settings)); // ok
}
