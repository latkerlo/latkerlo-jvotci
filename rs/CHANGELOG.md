# `latkerlo-jvotci` crate changelog

## 1.0.9
- checks the first letter is a consonant for 3- or 4-letter rafsi. seriously how did i forget this

## 1.0.8
- no longer `Err`s when given < 2 words

## 1.0.7
- the type of `RAFSI` is now `HashMap<&'static str, Vec<&'static str>>`

## 1.0.6
- added git url to Cargo.toml

## 1.0.5
- the rafsi map is now constructed with `HashMap::from()`

## 1.0.4
- changed type of `RAFSI` from `HashMap<&'static str, &'static[&'static str]>` to `HashMap<&'static str, Vec<String>>`

## 1.0.3
- `jvoakaha2`'s error is formatted properly

## 1.0.2
- `get_lujvo2` no longer panics "non-Lojban character in {gry}" for *eiksy'aigryspe*
- `jvokaha` `Err`s if given something that isn't a lujvo at all
- added changelog

## 1.0.1
- `get_veljvo` now returns a `Result` rather than `panic!`ing

## 1.0.0
- initial release