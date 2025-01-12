use crate::auto_to_string;
#[derive(Debug, Clone)]
pub enum Jvonunfli {
    /// Something is not decomposable
    DecompositionError(String),
    /// Something contains a prohibited consonant cluster
    InvalidClusterError(String),
    /// Lujvo creation fails
    NoLujvoFoundError(String),
    /// There are characters that aren't Lojban letters
    NonLojbanCharacterError(String),
    /// Something isn't a brivla
    NotBrivlaError(String),
    /// Something isn't a zi'evla
    NotZihevlaError(String),
    /// Something happened that in e.g. the TS implementation would throw a `TypeError`
    FakeTypeError(String),
}
auto_to_string!(Jvonunfli);
