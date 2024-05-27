use itertools::Itertools;
use lazy_static::lazy_static;

pub const VALID: [&str; 179] = [
    "bd", "bg", "bj", "bl", "bm", "bn", "br", "bv", "bz", "cf", "ck", "cl", "cm", "cn", "cp", "cr",
    "ct", "db", "dg", "dj", "dl", "dm", "dn", "dr", "dv", "dz", "fc", "fk", "fl", "fm", "fn", "fp",
    "fr", "fs", "ft", "fx", "gb", "gd", "gj", "gl", "gm", "gn", "gr", "gv", "gz", "jb", "jd", "jg",
    "jl", "jm", "jn", "jr", "jv", "kc", "kf", "kl", "km", "kn", "kp", "kr", "ks", "kt", "lb", "lc",
    "ld", "lf", "lg", "lj", "lk", "lm", "ln", "lp", "lr", "ls", "lt", "lv", "lx", "lz", "mb", "mc",
    "md", "mf", "mg", "mj", "mk", "ml", "mn", "mp", "mr", "ms", "mt", "mv", "mx", "nb", "nc", "nd",
    "nf", "ng", "nj", "nk", "nl", "nm", "np", "nr", "ns", "nt", "nv", "nx", "nz", "pc", "pf", "pk",
    "pl", "pm", "pn", "pr", "ps", "pt", "px", "rb", "rc", "rd", "rf", "rg", "rj", "rk", "rl", "rm",
    "rn", "rp", "rs", "rt", "rv", "rx", "rz", "sf", "sk", "sl", "sm", "sn", "sp", "sr", "st", "sx",
    "tc", "tf", "tk", "tl", "tm", "tn", "tp", "tr", "ts", "tx", "vb", "vd", "vg", "vj", "vl", "vm",
    "vn", "vr", "vz", "xf", "xl", "xm", "xn", "xp", "xr", "xs", "xt", "zb", "zd", "zg", "zl", "zm",
    "zn", "zr", "zv",
];
lazy_static! {
    pub static ref MZ_VALID: Vec<&'static str> = VALID.iter().chain([&"mz"]).cloned().collect_vec();
}

pub const INITIAL: [&str; 48] = [
    "bl", "br", "cf", "ck", "cl", "cm", "cn", "cp", "cr", "ct", "dj", "dr", "dz", "fl", "fr", "gl",
    "gr", "jb", "jd", "jg", "jm", "jv", "kl", "kr", "ml", "mr", "pl", "pr", "sf", "sk", "sl", "sm",
    "sn", "sp", "sr", "st", "tc", "tr", "ts", "vl", "vr", "xl", "xr", "zb", "zd", "zg", "zm", "zv",
];
pub const ZIHEVLA_INITIAL: [&str; 16] = [
    "bl", "br", "dr", "fl", "fr", "gl", "gr", "kl", "kr", "ml", "mr", "pl", "pr", "tr", "vl", "vr",
];

pub const BANNED_TRIPLES: [&str; 4] = ["ndj", "ndz", "ntc", "nts"];

pub const START_VOWEL_CLUSTERS: [&str; 9] = ["a", "e", "i", "o", "u", "au", "ai", "ei", "oi"];
pub const FOLLOW_VOWEL_CLUSTERS: [&str; 18] = [
    "ia", "ie", "ii", "io", "iu", "iau", "iai", "iei", "ioi", "ua", "ue", "ui", "uo", "uu", "uau",
    "uai", "uei", "uoi",
];

pub const HYPHENS: [&str; 6] = ["r", "n", "y", "'y", "y'", "'y'"];
