use latkerlo_jvotci::*;

fn main() {
    // things
    let settings = Settings {
        generate_cmevla: false,
        y_hyphens: YHyphenSetting::AllowY,
        exp_rafsi: true,
        consonants: ConsonantSetting::Cluster,
        glides: false,
        allow_mz: true,
    };
    println!("{:?}", get_lujvo("sakprtlfmsnge'a co'e", &settings))
}
