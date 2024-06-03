use latkerlo_jvotci::*;
use rafsi::RAFSI;

fn main() {
    // things
    println!("{:?}", RAFSI.get("condi"));
    println!("{:?}", grill("condi djedi", &Settings::default()));
    println!("{:?}", get_lujvo("condi djedi", &Settings::default()));
    println!("{:?}", get_veljvo("zi'evlyjvo", &Settings::default()))
}
