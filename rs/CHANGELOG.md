# `latkerlo-jvotci` crate changelog

## 2.3.1
- fix message about *y* followed by non-glide vowel sequence
- add some `#[must_use]`s

## 2.3.0
- export `RAFSI`

## 2.2.0
- updated rafsi list
- unmigrated back to rust 2021
- documented msrv
- minor internal stuff
- added more metadata for cargo

## 2.1.0
- latkerlo-jvotci is about 5x faster :3
- migrated to rust 2024

## 2.0.0
> [!WARNING]
> lots of breaking changes! see [docs.rs](https://docs.rs/latkerlo-jvotci/) for everything you can do now
- can now put zi'evla in lujvo!
- new settings to control various things
- fixed a typo in the changelog entry for 1.0.3 lol

## 1.0.7
- the type of `RAFSI` is now `HashMap<&'static str, Vec<&'static str>>`

## 1.0.6
- added git url to Cargo.toml

## 1.0.5
- the rafsi map is now constructed with `HashMap::from()`

## 1.0.4
- changed type of `RAFSI` from `HashMap<&'static str, &'static[&'static str]>` to `HashMap<&'static str, Vec<String>>`

## 1.0.3
- `jvokaha2`'s error is formatted properly

## 1.0.2
- `get_lujvo2` no longer panics "non-Lojban character in {gry}" for *eiksy'aigryspe*
- `jvokaha` `Err`s if given something that isn't a lujvo at all
- added changelog

## 1.0.1
- `get_veljvo` now returns a `Result` rather than `panic!`ing

## 1.0.0
- initial release
