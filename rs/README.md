Features not present in the Python/TypeScript versions:
- ability to convert a `Settings` to/from a string e.g. `cF1rgz`

If you run the tests please be sure to do so as `cargo test -r -- --nocapture --test-threads=1`

MSRV: 1.80.1 (tested on `x86_64-pc-windows-msvc`)

Test benchmarks:
```
                    debug    release
                    ------   -------
windows 11           1m11s       31s
fedora 42 (vm)         45s        6s
ubuntu 22.04 (wsl)   2m02s     1m27s
```