// use latkerlo_jvotci::*;

use latkerlo_jvotci::*;

fn main() {
    println!("{:?}", katna::get_veljvo("jvoco'e"));
    println!("{}", jvozba::get_lujvo("lujvo co'e", true).unwrap().0);
}