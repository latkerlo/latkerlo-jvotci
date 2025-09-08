# latkerlo-jvotci
A library and CLI for creating and decomposing lujvo in Lojban.

Docs for the library are available at [docs.rs](https://docs.rs/latkerlo_jvotci).
If you run the tests (in the [Github repo](https://github.com/latkerlo/latkerlo-jvotci) please be sure to do so as `cargo test -r -- --nocapture --test-threads=1`

MSRV: 1.88.0

## Installing the binary
```
$ cargo install latkerlo-jvotci
```
## Using the binary
**Creating lujvo from tanru:**
```
$ jvotci lujvo tutci
jvotci
```

**Analyzing existing lujvo:**
```
$ jvotci -L latkerlo
lujvo
lat kerlo
7937
mlatu kerlo
```

**Interactive mode:**
```
jvotci
```
`/h` for help, `/q` to quit.

**Show help text:**
```
jvotci -h
```


