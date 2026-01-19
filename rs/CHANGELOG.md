# `latkerlo-jvotci` crate changelog

## a.b.yymm (any new patch release)
- updated the rafsi list

## 2.5.2601
- fixed *mlongen- tsiju*
- stopped rewarding apostrophes; now the generated lujvo always has the fewest possible syllables
- made the CLI print `-` for unnecessary hyphens less

## 2.4.2510
- improved some docs
- made the CLI return exit codes more consistently

## 2.4.0
- fixed *bastryvla* and *toii'ysmu*
- improved error message consistency for tosmabru
- added a `cargo install`able CLI (see the readme / [crates.io](https://crates.io/crates/latkerlo-jvotci))
- rewrote the changelog to consistently use past tense

## 2.3.1
- fixed message about *y* followed by non-glide vowel sequence
- added some `#[must_use]`s

## 2.3.0
- made `RAFSI` a public export

## 2.2.0
- updated rafsi list
- unmigrated back to rust 2021
- documented msrv
- did minor internal stuff
- added more metadata for cargo

## 2.1.0
- made the tests about 5x faster!
- migrated to rust 2024

## 2.0.0
> [!WARNING]
> lots of breaking changes! see [docs.rs](https://docs.rs/latkerlo-jvotci/) for everything you can do now
- made it possible to put zi'evla in lujvo now!
- added new settings to control various things
- fixed a typo in the changelog entry for 1.0.3 lol

## 1.0.7
- changed the type of `RAFSI` to `HashMap<&'static str, Vec<&'static str>>`

## 1.0.6
- added git url to Cargo.toml

## 1.0.5
- switched to constructing the rafsi map with `HashMap::from()`

## 1.0.4
- changed the type of `RAFSI` from `HashMap<&'static str, &'static[&'static str]>` to `HashMap<&'static str, Vec<String>>`

## 1.0.3
- formatted `jvokaha2`'s error properly

## 1.0.2
- fixed `get_lujvo2`'s error of "non-Lojban character in {gry}" for *eiksy'aigryspe*
- made `jvokaha` `Err` if given something that isn't a lujvo at all
- added a changelog

## 1.0.1
- made `get_veljvo` return a `Result` rather than `panic!`ing

## 1.0.0
- initial release
