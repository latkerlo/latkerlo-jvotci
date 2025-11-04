//! Various lists of things like consonant clusters. The rafsi list is stored in
//! [`rafsi`][`crate::rafsi`] instead.

use std::{collections::HashSet, sync::LazyLock};

/// The consonant clusters permitted by CLL.
pub static VALID: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "bd", "bg", "bj", "bl", "bm", "bn", "br", "bv", "bz", "cf", "ck", "cl", "cm", "cn", "cp",
        "cr", "ct", "db", "dg", "dj", "dl", "dm", "dn", "dr", "dv", "dz", "fc", "fk", "fl", "fm",
        "fn", "fp", "fr", "fs", "ft", "fx", "gb", "gd", "gj", "gl", "gm", "gn", "gr", "gv", "gz",
        "jb", "jd", "jg", "jl", "jm", "jn", "jr", "jv", "kc", "kf", "kl", "km", "kn", "kp", "kr",
        "ks", "kt", "lb", "lc", "ld", "lf", "lg", "lj", "lk", "lm", "ln", "lp", "lr", "ls", "lt",
        "lv", "lx", "lz", "mb", "mc", "md", "mf", "mg", "mj", "mk", "ml", "mn", "mp", "mr", "ms",
        "mt", "mv", "mx", "nb", "nc", "nd", "nf", "ng", "nj", "nk", "nl", "nm", "np", "nr", "ns",
        "nt", "nv", "nx", "nz", "pc", "pf", "pk", "pl", "pm", "pn", "pr", "ps", "pt", "px", "rb",
        "rc", "rd", "rf", "rg", "rj", "rk", "rl", "rm", "rn", "rp", "rs", "rt", "rv", "rx", "rz",
        "sf", "sk", "sl", "sm", "sn", "sp", "sr", "st", "sx", "tc", "tf", "tk", "tl", "tm", "tn",
        "tp", "tr", "ts", "tx", "vb", "vd", "vg", "vj", "vl", "vm", "vn", "vr", "vz", "xf", "xl",
        "xm", "xn", "xp", "xr", "xs", "xt", "zb", "zd", "zg", "zl", "zm", "zn", "zr", "zv",
    ])
});

#[allow(clippy::too_long_first_doc_paragraph)]
/// The CLL consonant clusters ([`VALID`]) + *mz*, which CLL forbids in order to
/// spite the inventor of Loglan. You can control whether this list of clusters
/// is used via `allow_mz` in [`Settings`][`crate::Settings`].
pub static MZ_VALID: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut clusters = VALID.iter().copied().collect::<HashSet<_>>();
    clusters.insert("mz");
    clusters
});

/// The consonant clusters permitted *word-initially* by CLL.
pub static INITIAL: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "bl", "br", "cf", "ck", "cl", "cm", "cn", "cp", "cr", "ct", "dj", "dr", "dz", "fl", "fr",
        "gl", "gr", "jb", "jd", "jg", "jm", "jv", "kl", "kr", "ml", "mr", "pl", "pr", "sf", "sk",
        "sl", "sm", "sn", "sp", "sr", "st", "tc", "tr", "ts", "vl", "vr", "xl", "xr", "zb", "zd",
        "zg", "zm", "zv",
    ])
});

pub static ZIHEVLA_INITIAL: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        "bl", "br", "dr", "fl", "fr", "gl", "gr", "kl", "kr", "ml", "mr", "pl", "pr", "tr", "vl",
        "vr",
    ])
});
/// The set of consonant triples banned by CLL: *nts*, *ntc*, *ndz*, *ndj*.
/// These are banned because they sound too similar to *ns*, *nc*, *nz*, *nj*.
pub static BANNED_TRIPLES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["ndj", "ndz", "ntc", "nts"]));

/// Single vowels and falling diphthongs. These syllables always require a
/// pronounced glottal stop before them.
pub static START_VOWEL_CLUSTERS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["a", "e", "i", "o", "u", "au", "ai", "ei", "oi"]));

/// Syllables starting with glides (*i*/*u*). These syllables can sometimes not
/// be preceded by a pronounced glottal stop.
pub static FOLLOW_VOWEL_CLUSTERS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "ia", "ie", "ii", "io", "iu", "iau", "iai", "iei", "ioi", "ua", "ue", "ui", "uo", "uu",
        "uau", "uai", "uei", "uoi",
    ])
});

/// The set of lujvo hyphens, used between zi'evla and to prevent cmavo-shaped
/// rafsi from falling of the start of a lujvo.
pub static HYPHENS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["r", "n", "y", "'y", "y'", "'y'"]));
