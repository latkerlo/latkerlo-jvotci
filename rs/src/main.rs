use latkerlo_jvotci::*;

fn main() {

    println!("{}", katna::get_veljvo("jvoka'a").join(" "));
    println!("{}", jvozba::get_lujvo("lujvo zbasu", false).unwrap().0);
}