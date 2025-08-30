//! Categories of errors.

use crate::auto_to_string;

#[derive(Debug, Clone)]
/// An error. The specific variants/messages that are returned are discussed
/// per-function.
pub enum Jvonunfli {
    /// Something is not decomposable.
    DecompositionError(String),
    /// Something contains a prohibited consonant cluster.
    InvalidClusterError(String),
    /// Lujvo creation has failed.
    NoLujvoFoundError(String),
    /// There are characters that aren't Lojban letters.
    NonLojbanCharacterError(String),
    /// Something isn't a brivla.
    NotBrivlaError(String),
    /// Something isn't a zi'evla.
    NotZihevlaError(String),
    /// Something happened that would throw a `TypeError` in TypeScript.
    FakeTypeError(String),
}
auto_to_string!(Jvonunfli);
