use latkerlo_jvotci::*;

fn main() {

    println!("{}y", tools::slice("01234", 0, -1));
    println!("{:?}", rafsi::RAFSI.get("blaci"));
    println!("{:?}", jvozba::get_rafsi_list_list(vec!["blaci".to_string()], false));
    println!("{:?}", katna::get_veljvo("jvoco'e").join(" "));
    println!("{:?}", jvozba::get_lujvo("lujvo co'e", true).unwrap().0);
}