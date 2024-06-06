use latkerlo_jvotci::*;
use tarmi::rafsi_tarmi;
// use rafsi::RAFSI;

fn main() {
    let settings = &Settings::default();
    println!("{:?}", rafsi_tarmi("lo'"));
    println!("{:?}", get_lujvo("djedi", settings));
}
