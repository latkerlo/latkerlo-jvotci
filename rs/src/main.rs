use latkerlo_jvotci::{get_lujvo, get_veljvo};

fn main() {
    println!(
        "{}",
        get_lujvo("lujvo co'e", true)
            .expect("can't make that lujvo")
            .0
    );
    println!("{}", get_veljvo("jvoco'e").expect("not a lujvo").join(" "));
}
