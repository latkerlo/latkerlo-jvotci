# oh no

(all of these are also in "Issues"land)

## 1. initial CCV'y should not be allowed

`uajvo` is a valid zi'evla. Therefore `uajvyvla` should be a valid lujvo (it is), and so should `uajvo'yvla` (it is).

This means `*jvo'yvla` should be invalid, because you could put `ua` in front of it and get a valid lujvo (so if you really wanted the `'y` you'd need to do `lujvo'yvla`).

**Python** and **TS** decomposes `*jvo'yvla` just fine into `lujvo valsi`.

In **Rust** it doesn't work (good) for the wrong reason (bad), because

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

`lavlevy'ismu` works correctly everywhere.

## 5. I don't know what to call this one

Rust can decompose `ke'ery'u` in `r` (except `Fr`) and `ke'erytce` in non-`F`. ???

## 6. `coidje` and friends are probabilistic

Rust outputs `coidje` or `cnodei` inconsistently for `condi djedi`, and similarly with `klesi` in place of `condi` and/or when the order is reversed.

Results of generating `condi djedi` 10,000 times:

```
