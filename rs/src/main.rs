use latkerlo_jvotci::*;

fn main() {
    println!(
        "{}",
        get_lujvo("lujvo co'e", true)
            .expect("can't make that lujvo")
            .0
    );
    println!("{}", get_veljvo("iismynei").expect("not a lujvo").join(" "));
}
