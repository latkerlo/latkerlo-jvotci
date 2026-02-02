//! Lojban lujvo generation and analysis.
//!
//! `latkerlo_jvotci` accepts more words as valid lujvo than CLL does. The
//! primary differences are that:
//! - unnecessary hyphens are allowed by default (making e.g. *zi'ervla* a lujvo
//!   rather than a zi'evla), though [`Settings`] can fix this
//! - zi'evla can go inside lujvo (making e.g. *itku'ilybau* a lujvo)
//!
//! # Quick start
//! ```
//! use latkerlo_jvotci::*;
//!
//! # fn main() -> Result<(), Jvonunfli> {
//! let settings = Settings::default();
//! let tanru = "blanu zdani";
//! let lujvo = get_lujvo(tanru, &settings)?; // -> "blazda".to_string()
//! let veljvo = get_veljvo(&lujvo, &settings)?; // -> vec!["blanu", "zdani"]
//! assert_eq!(veljvo.join(" "), tanru);
//! # Ok(())
//! # }
//! ```

// excluded lints
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::reversed_empty_ranges)]

pub mod cli_docs;
pub mod data;
pub mod exceptions;
pub mod jvozba;
pub mod katna;
pub mod rafsi;
pub mod tarmi;
mod test_list;
pub mod tools;

pub use exceptions::Jvonunfli;
pub use jvozba::{get_lujvo, get_lujvo_with_analytics, grll};
pub use katna::{get_veljvo, score_lujvo};
pub use rafsi::RAFSI;
pub use tarmi::{
    ConsonantSetting::{self, *},
    SETTINGS_ITERATOR, Settings,
    YHyphenSetting::{self, *},
};
pub use tools::{analyze_brivla, is_brivla, normalize};
