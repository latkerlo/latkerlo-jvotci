# `latkerlo-jvotci` crate changelog

## 1.1.2509
- changed type of rafsi lists to `Vec<&str>`
- updated the rafsi lists

## 1.1.0
(originally released as v1.0.1 through v1.0.6)
- added git url to Cargo.toml
- changed the type of `RAFSI` to `HashMap<&'static str, Vec<String>>`
- formatted `jvoakaha2`'s error properly
- fixed `get_lujvo2`'s error of "non-Lojban character in {gry}" for *eiksy'aigryspe*
- made `jvokaha` `Err` if given something that isn't a lujvo at all
- added changelog
- made `get_veljvo` return a `Result` rather than `panic!`ing

## 1.0.0
- initial release
