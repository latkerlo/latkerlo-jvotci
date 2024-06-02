use latkerlo_jvotci::*;
use rafsi::RAFSI;

fn main() {
    // things
    println!("{:?}", RAFSI.get("klesi"));
    println!("{:?}", get_lujvo("klesi klesi djedi", &Settings::default()));
}
