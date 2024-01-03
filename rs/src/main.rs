use latkerlo_jvotci::*;

fn main() {
    println!("{:?}", jvozba::get_rafsi_list_list(vec!["-tadj-".to_string()], false));
    println!("{}", jvozba::get_lujvo("-tadj- tadji", false).unwrap().0);
    println!("{}", katna::get_veljvo("弱音ハク").join(" "));
}