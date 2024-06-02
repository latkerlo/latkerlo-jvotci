#![feature(effects, const_trait_impl)]
pub mod data;
pub mod exceptions;
pub mod jvozba;
pub mod katna;
pub mod rafsi;
pub mod tarmi;
mod test_list;
pub mod tools;
pub use jvozba::{get_lujvo, get_lujvo_with_analytics, grill};
pub use katna::get_veljvo;
pub use tarmi::{
    ConsonantSetting::{self, *},
    Settings,
    YHyphenSetting::{self, *},
    SETTINGS_ITERATOR,
};
pub use tools::{analyze_brivla, is_brivla, normalize};
