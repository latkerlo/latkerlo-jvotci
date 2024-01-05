use latkerlo_jvotci::*;

fn main() {
    println!("{}", get_lujvo("lujvo co'e", true).unwrap().0);
    println!("{}", get_veljvo("jvoco'e").join(" "));
}