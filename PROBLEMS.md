# oh no

(all of these are also in "Issues"land)

## 1½. initial `CCV'y'`

Rust accepts e.g. `*jvo'y'ismu` even though, like e.g. `jvo'yvla`, it is a slinku'i.

## 2. Rust has problems with `'y`

Specifically it doesn't seem to think the apostrophe is part of the hyphen; it emits errors like

```rs
Err(NonLojbanCharacterError("{smu'} ends in an apostrophe"))
```

Despite the name `NonLojbanCharacterError`, `is_only_lojban_characters()` *does* count `'` as a Lojban character.

This is not a problem for TS or Python, and `y'` (e.g. `valy'akti`) works in all three. *And so does `'y'`* which is interesting...

## 3. misidentifying slinku'i

In `Fr`, Rust thinks `u'ykerlo` is a slinku'i. It isn't because putting `pa` in front of it would be `pa.u'ykerlo`.

## 4. not identifying tosmabru before `y'`

Rust decomposes `lavlevlivy'ismu` into `lav-lev-liv-y'-ismu` rather than `la vle-vliv-y'-ismu`.

`lavlevy'ismu` works correctly everywhere (i.e. `la vlevy'ismu`).

## 5. I don't know what to call this one

Rust can decompose `ke'ery'u` in `r` (except `Fr`) and `ke'erytce` in non-`F`. ???
