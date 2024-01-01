use latkerlo_jvotci::*;

fn main() {
    println!("{}", katna::get_veljvo("jvoco'e").join(" "));
    println!("{}", jvozba::get_lujvo("lujvo co'e", true).unwrap().0);
}