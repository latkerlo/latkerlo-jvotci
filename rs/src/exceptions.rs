use crate::auto_to_string;
#[derive(Debug, Clone)]
pub enum Jvonunfli {
    DecompositionError(String),
    InvalidClusterError(String),
    NoLujvoFoundError(String),
    NonLojbanCharacterError(String),
    NotBrivlaError(String),
    NotZihevlaError(String),
}
auto_to_string!(Jvonunfli);
