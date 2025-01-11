// excluded pedantic lints
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
pub mod data;
pub mod exceptions;
pub mod jvozba;
pub mod katna;
pub mod rafsi;
pub mod tarmi;
mod test_list;
pub mod tools;
pub use jvozba::{get_lujvo, get_lujvo_with_analytics, grll};
pub use katna::{get_veljvo, score_lujvo};
pub use tarmi::{
    ConsonantSetting::{self, *},
    Settings,
    YHyphenSetting::{self, *},
    SETTINGS_ITERATOR,
};
pub use tools::{analyze_brivla, is_brivla, normalize};
