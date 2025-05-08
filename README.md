# latkerlo jvotci

A set of tools for creating and decomposing Lojban lujvo. Available in Python,
JavaScript, and Rust. You can also try it out 
[here](https://latkerlo.github.io/latkerlo-jvotci/).

In general, the distinction between morphologically valid brivla and other 
strings is intended to mirror the results produced by 
[Camxes: Standard](http://lojban.github.io/ilmentufa/camxes.html) and the "add 
valsi" feature on [JVS](https://jbovlaste.lojban.org). However, a small number
of dissenting design choices are documented in [differences.md](differences.md).

The recent addition of zi'evla lujvo support is latkerlo's original work,
but the core CLL lujvo functionality is based on these three projects:

- [p-lujvo](https://codeberg.org/tb148/p-lujvo) by Miao Liang
- [jvozba](https://github.com/uakci/jvozba/tree/v3) by uakci
- [sozysozbot jvozba](https://github.com/sozysozbot/sozysozbot_jvozba) by sozysozbot

la latkerlo jvotci is licensed under the MIT license. See 
[LICENSE.md](LICENSE.md) for license details.

## Installation

### Python

Use pip:

```sh
pip install latkerlo-jvotci
```

### JavaScript

Copy and include all these files from the js/docs folder:

```html
<script src="data.js"></script>
<script src="rafsi.js"></script>
<script src="exceptions.js"></script>
<script src="tarmi.js"></script>
<script src="tools.js"></script>
<script src="jvozba.js"></script>
<script src="katna.js"></script>
```

### Rust

```sh
cargo add latkerlo_jvotci
```

## The Basics

**Rust is covered [here](https://docs.rs/latkerlo-jvotci/) - the rest of this readme only applies to Python and JS**

To create a lujvo, use the function `get_lujvo` / `getLujvo`. Provide a veljvo
as a list/array of strings, or a single string with words separated by spaces.
The result is a string.

```python
# python
get_lujvo("mlatu kerlo")
get_lujvo(["mlatu", "kerlo"])
# -> "latkerlo"

get_lujvo("mlatu kerlo", generate_cmevla=True)
get_lujvo(["mlatu", "kerlo"], generate_cmevla=True)
# -> "latker"
```

```javascript
// javascript
getLujvo("mlatu kerlo");
getLujvo(["mlatu", "kerlo"]);
// -> "latkerlo"

getLujvo("mlatu kerlo", {generateCmevla: true});
getLujvo(["mlatu", "kerlo"], {generateCmevla: true});
// -> "latker"
// (Note that optional arguments are provided as an object.)
```

To decompose a lujvo, use the function `get_veljvo` / `getVeljvo`.
Provide a lujvo as a string. The result is a list/array of strings.

```python
# python
get_veljvo("latkerlo")  # -> ["mlatu", "kerlo"]

```

```javascript
// javascript
getVeljvo("latkerlo");  // -> ["mlatu", "kerlo"]

```

## Other Useful Functions

`is_brivla` / `isBrivla` returns True if the provided string is a valid brivla.

```python
# python
is_brivla("latkerlo")  # -> True
is_brivla("bisladru")  # -> False
is_brivla("latker")    # -> False
```

```javascript
// javascript
isBrivla("latkerlo");  // -> true
isBrivla("bisladru");  // -> false
isBrivla("latker");    // -> false
```
\
Under the hood, `get_veljvo` and `is_brivla` call the function `analyse_brivla`
/ `analyseBrivla`. If the provided string is a brivla or decomposable cmevla,
the function provides two pieces of information about it: the type of word
(a string), and its raw decomposition (a string list/array). If it is some
other kind of word, an exception/error is raised/thrown.

```python
# python
analyse_brivla("tcanyja'a")  # -> ("LUJVO", ["tcan", "y", "ja'a"])
analyse_brivla("re'ertren")  # -> ("CMEVLA", ["re'e", "r", "tren"])
analyse_brivla("mlatu")      # -> ("GISMU", ["mlatu"])
analyse_brivla("latkello")   # -> raises NotBrivlaError
```
```javascript
// javascript
analyseBrivla("tcanyja'a");  // -> ["LUJVO", ["tcan", "y", "ja'a"]]
analyseBrivla("re'ertren");  // -> ["CMEVLA", ["re'e", "r", "tren"]]
analyseBrivla("mlatu");      // -> ["GISMU", ["mlatu"]]
analyseBrivla("latkello");   // -> throws NotBrivlaError
```

The five possible word types that can be returned are `"GISMU"`, `"ZIhEVLA"`, 
`"LUJVO"`, `"EXTENDED_LUJVO"`, and `"CMEVLA"`. LUJVO are lujvo as defined in 
the CLL, and EXTENDED_LUJVO are forms that are allowed by newer and/or 
experimental rules. For many applications, this distinction may not be 
important.

Note that cmevla not composed of (two or more) rafsi will result in an error, 
but decomposable cmevla will be processed even though they are, in fact, not 
brivla.

If you want to get the type and decomposition of a word, it may be more 
efficient to use `analyse_brivla` and then process the results rather than, 
for example, using `get_veljvo` and `is_brivla` separately.
\
\
\
The function `get_lujvo_with_analytics` / `getLujvoWithAnalytics` provides the
lujvo along with its score and a list of pairs of start and end indices marking
where each word in the input is represented in the final result.

```python
# python
get_lujvo_with_analytics("mlatu kerlo")
# -> ["latkerlo", 7937, [(0, 3), (3, 8)]]

get_lujvo_with_analytics("tcana jatna")
# -> ["tcanyja'a", 8597, [(0, 4), (5, 9)]]
```

```javascript
// javascript
getLujvoWithAnalytics("mlatu kerlo");
// -> ["latkerlo", 7937, [[0, 3], [3, 8]]]

getLujvoWithAnalytics("tcana jatna");
// -> ["tcanyja'a", 8597, [[0, 4], [5, 9]]]
```
\
`get_rafsi_indices` / `getRafsiIndices` can be used to get the corresponding
index-pair list from the decomposition result of `analyse_brivla`.

```python
# python
get_rafsi_indices(["lat", "kerlo"])
# -> [(0, 3), (3, 8)]

b_type, decomp = analyse_brivla("tcanyja'a")
get_rafsi_indices(decomp)
# -> [(0, 4), (5, 9)]
```

```javascript
// javascript
getRafsiIndices(["lat", "kerlo"]);
// -> [[0, 3], [3, 8]]

let [bType, decomp] = analyseBrivla("tcanyja'a");
getRafsiIndices(decomp);
// -> [[0, 4], [5, 9]]
```

## Advanced Options

Support is provided for a variety of proposed and experimental modifications
to Lojban's morphology. As a CLL sympathiser, la latkerlo does not necessarily
personally approve of any of them. Nonetheless, there may be some value in 
using them experimentally, and the code is already written, so it might as well
be published. :)

The following five modifications are available as options in all functions
described above, except for `getRafsiIndices`.

### y_hyphens / yHyphens

A proposal exists to require all cmavo ending in y to be followed by a pause.
This allows `'y` to be used as a hyphen to attach CVV and CVhV rafsi at the 
beginning of a lujvo. The three settings are `"STANDARD"` (default), 
`"ALLOW_Y"`, and `"FORCE_Y"`. 

STANDARD:<br>
No extra y-hyphens.<br>
rai'ymlatu -> rai'y mlatu (two words)

ALLOW_Y:<br>
y-hyphens are allowed, and r- & n-hyphens can still be in lujvo<br>
rai'ymlatu -> LUJVO<br>
re'enre'e -> LUJVO

FORCE_Y:<br>
y-hyphens are always used instead of r- & n-hyphens 
(which create zi'evla instead)<br>
rai'ymlatu -> LUJVO<br>
re'enre'e -> ZIhEVLA

### exp_rafsi_shapes / expRafsiShapes

Experimental rafsi shapes extend rafsi to all cmavo shapes, as long as they do
not contain y. The two settings are `False` (default) and `True`.

False (standard rafsi shapes):<br>
Only CLL short rafsi shapes. zi'evla minus final vowels are also allowed.<br>
mlatyfa'u'u -> NOT BRIVLA

True (experimental rafsi shapes):<br>
Any cmavo shape without a y can be a "short" rafsi.<br>
mlatyfa'u'u -> LUJVO<br>

### consonants

Relaxation of the requirement that brivla contain a consonant cluster.<br>
Some background: When a pause is required after any cmavo ending in y (either
ALLOW_Y or FORCE_Y), there are some strings (e.g. `nei'ynei`) that cannot fall
apart into multiple words, cannot combine into other words, and also do not 
break any of the language's overarching morphological rules. Changing the
requirement from a consonant cluster to any two consonants allows these strings
to be used as words, sometimes resulting in better-scoring lujvo. In
combination with experimental rafsi shapes, this can also be reduced to a
single non-initial consonant.

The three settings are `"CLUSTER"` (default), `"TWO_CONSONANTS"`, and 
`"ONE_CONSONANT"`.

(Examples assume ALLOW_Y and experimental rafsi.)

CLUSTER:<br>
A consonant cluster must appear in every brivla (may be separated by y).<br>
lojycpa -> LUJVO<br>
nei'ynei -> NOT BRIVLA<br>
a'ynei -> NOT BRIVLA

TWO_CONSONANTS:<br>
2 consonants are required (need not be adjacent).<br>
lojycpa -> LUJVO<br>
nei'ynei -> EXTENDED LUJVO<br>
a'ynei -> NOT BRIVLA

ONE_CONSONANT:<br>
Only 1 consonant is required (cannot be initial).<br>
lojycpa -> LUJVO<br>
nei'ynei -> EXTENDED LUJVO<br>
a'ynei -> EXTENDED LUJVO

### glides

Treats glides as consonants. Has no effect unless used with experimental rafsi
shapes. The two settings are `False` (default) and `True`.

False (glides are ***not*** consonants):<br>
latyia -> NOT BRIVLA

True (glides ***are*** consonants):<br>
latyia -> EXTENDED LUJVO

Note: In combination with relaxed consonant rules and either ALLOW_Y or 
FORCE_Y, this may produce lujvo with no actual consonants.<br>
ia'yia -> EXTENDED LUJVO

### allow_mz / allowMZ

Allows `mz` as a valid consonant cluster. The two settings are `False`
(default) and `True`.

False (mz is ***not*** valid):<br>
gimzu -> NOT BRIVLA<br>
-tam- zabna -> tamyza'a

True (mz ***is*** valid):<br>
gimzu -> GISMU<br>
-tam- zabna -> tamza'a

### Example Usage

```python
# python
get_veljvo("le'e'e'ygimzu", 
           y_hyphens="ALLOW_Y", 
           exp_rafsi_shapes=True, 
           consonants="TWO_CONSONANTS", 
           glides=True, 
           allow_mz=True)
get_veljvo("le'e'e'ygimzu", "FORCE_Y", False, "CLUSTER", False, True)
get_veljvo("le'e'e'ygimzu", allow_mz=True)
```

```javascript
// javascript
getVeljvo("le'e'e'ygimzu", {
  yHyphens: "ALLOW_Y", 
  expRafsiShapes: true, 
  consonants: "TWO_CONSONANTS", 
  glides: true, 
  allowMZ: true
});
getVeljvo("le'e'e'ygimzu", {allowMZ: true});
```
