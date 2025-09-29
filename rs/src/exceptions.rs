//! Categories of errors.

use std::fmt::Display;

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
use Jvonunfli::{
    DecompositionError, FakeTypeError, InvalidClusterError, NoLujvoFoundError,
    NonLojbanCharacterError, NotBrivlaError, NotZihevlaError,
};
impl Jvonunfli {
    pub fn text(self) -> String {
        match self {
            DecompositionError(e)
            | InvalidClusterError(e)
            | NoLujvoFoundError(e)
            | NonLojbanCharacterError(e)
            | NotBrivlaError(e)
            | NotZihevlaError(e)
            | FakeTypeError(e) => e,
        }
    }
}
impl Display for Jvonunfli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone().text())
    }
}
