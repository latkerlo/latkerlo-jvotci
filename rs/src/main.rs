use latkerlo_jvotci::*;

fn main() {
    // things
    println!(
        "{:?}",
        jvozba::get_rafsi_list_list(
            vec!["sanmi".to_string(), "bukpu".to_string()],
            &Settings {
                y_hyphens: ForceY,
                consonants: TwoConsonants,
                ..Settings::default()
            }
        )
    );
}
