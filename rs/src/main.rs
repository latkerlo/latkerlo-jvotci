use latkerlo_jvotci::*;

fn main() {
    println!("{}", jvozba::get_lujvo("lujvo co'e", true).unwrap().0);
    println!("{}", katna::get_veljvo("jvoco'e").join(" "));
}