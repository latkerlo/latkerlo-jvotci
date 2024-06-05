use latkerlo_jvotci::*;
use tarmi::rafsi_tarmi;
// use rafsi::RAFSI;

fn main() {
    let settings = &Settings::default();
    println!("{:?}", rafsi_tarmi("jesy"));
    println!("{:?}", get_lujvo("condi djedi", settings));
}
