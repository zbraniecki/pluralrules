#![allow(unused_variables, unused_parens)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::float_cmp))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::nonminimal_bool))]
use super::operands::PluralOperands;
use super::PluralCategory;
use matches::matches;
pub type PluralRule = fn(&PluralOperands) -> PluralCategory;
pub static CLDR_VERSION: usize = 36;
pub const LOCALES_CARDINAL: &[&str] = &[
    "af", "ak", "am", "an", "ar", "ars", "as", "asa", "ast", "az", "be", "bem", "bez", "bg", "bho",
    "bm", "bn", "bo", "br", "brx", "bs", "ca", "ce", "ceb", "cgg", "chr", "ckb", "cs", "cy", "da",
    "de", "dsb", "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu", "fa", "ff", "fi", "fil",
    "fo", "fr", "fur", "fy", "ga", "gd", "gl", "gsw", "gu", "guw", "gv", "ha", "haw", "he", "hi",
    "hr", "hsb", "hu", "hy", "ia", "id", "ig", "ii", "in", "io", "is", "it", "iu", "iw", "ja",
    "jbo", "jgo", "ji", "jmc", "jv", "jw", "ka", "kab", "kaj", "kcg", "kde", "kea", "kk", "kkj",
    "kl", "km", "kn", "ko", "ks", "ksb", "ksh", "ku", "kw", "ky", "lag", "lb", "lg", "lkt", "ln",
    "lo", "lt", "lv", "mas", "mg", "mgo", "mk", "ml", "mn", "mo", "mr", "ms", "mt", "my", "nah",
    "naq", "nb", "nd", "ne", "nl", "nn", "nnh", "no", "nqo", "nr", "nso", "ny", "nyn", "om", "or",
    "os", "osa", "pa", "pap", "pl", "prg", "ps", "pt", "pt-PT", "rm", "ro", "rof", "root", "ru",
    "rwk", "sah", "saq", "sc", "scn", "sd", "sdh", "se", "seh", "ses", "sg", "sh", "shi", "si",
    "sk", "sl", "sma", "smi", "smj", "smn", "sms", "sn", "so", "sq", "sr", "ss", "ssy", "st", "su",
    "sv", "sw", "syr", "ta", "te", "teo", "th", "ti", "tig", "tk", "tl", "tn", "to", "tr", "ts",
    "tzm", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "vun", "wa", "wae", "wo", "xh", "xog", "yi",
    "yo", "yue", "zh", "zu",
];
pub const LOCALES_ORDINAL: &[&str] = &[
    "af", "am", "an", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "ce", "cs", "cy", "da", "de",
    "dsb", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "fy", "ga", "gd", "gl", "gsw",
    "gu", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "id", "in", "is", "it", "iw", "ja", "ka",
    "kk", "km", "kn", "ko", "kw", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mo", "mr", "ms", "my",
    "nb", "ne", "nl", "or", "pa", "pl", "prg", "ps", "pt", "ro", "root", "ru", "sc", "scn", "sd",
    "sh", "si", "sk", "sl", "sq", "sr", "sv", "sw", "ta", "te", "th", "tk", "tl", "tr", "uk", "ur",
    "uz", "vi", "yue", "zh", "zu",
];
#[cfg_attr(tarpaulin, skip)]
pub const PRS_CARDINAL: &[PluralRule] = &[
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 3..=10)) {
            PluralCategory::FEW
        } else if (matches!(po.i, 11..=99)) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else if (po.n == 0.0) {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 3..=10)) {
            PluralCategory::FEW
        } else if (matches!(po.i, 11..=99)) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else if (po.n == 0.0) {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 2..=4) && !matches!(po.i, 12..=14)) {
            PluralCategory::FEW
        } else if (po.i % 10 == 0) || (matches!(po.i, 5..=9)) || (matches!(po.i, 11..=14)) {
            PluralCategory::MANY
        } else if (po.i % 10 == 1 && po.i % 100 != 11) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if ((po.i % 10 == 9 || matches!(po.i, 3..=4))
            && !matches!(po.i, 10..=19)
            && !matches!(po.i, 70..=79)
            && !matches!(po.i, 90..=99))
        {
            PluralCategory::FEW
        } else if (po.n != 0.0 && po.i % 1000000 == 0) {
            PluralCategory::MANY
        } else if (po.i % 10 == 1 && po.i % 100 != 11 && po.i % 100 != 71 && po.i % 100 != 91) {
            PluralCategory::ONE
        } else if (po.i % 10 == 2 && po.i % 100 != 12 && po.i % 100 != 72 && po.i % 100 != 92) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && matches!(po.i % 10, 2..=4) && !matches!(po.i % 100, 12..=14))
            || (matches!(po.f % 10, 2..=4) && !matches!(po.f % 100, 12..=14))
        {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
            || (po.f % 10 == 1 && po.f % 100 != 11)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
            || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
            || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 2..=4) && po.v == 0) {
            PluralCategory::FEW
        } else if (po.v != 0) {
            PluralCategory::MANY
        } else if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 3.0) {
            PluralCategory::FEW
        } else if (po.n == 6.0) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else if (po.n == 0.0) {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) || (po.t != 0 && (po.i == 0 || po.i == 1)) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (matches!(po.f % 100, 3..=4)) {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
            PluralCategory::ONE
        } else if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0 || po.i == 1) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
            || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
            || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0 || po.i == 1) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 3..=6) && po.f == 0) {
            PluralCategory::FEW
        } else if (matches!(po.i, 7..=10) && po.f == 0) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 3..=10) && po.f == 0 || matches!(po.i, 13..=19) && po.f == 0) {
            PluralCategory::FEW
        } else if (po.n == 1.0 || po.n == 11.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0 || po.n == 12.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0
            && (po.i % 100 == 0
                || po.i % 100 == 20
                || po.i % 100 == 40
                || po.i % 100 == 60
                || po.i % 100 == 80))
        {
            PluralCategory::FEW
        } else if (po.v != 0) {
            PluralCategory::MANY
        } else if (po.v == 0 && po.i % 10 == 1) {
            PluralCategory::ONE
        } else if (po.v == 0 && po.i % 10 == 2) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && !matches!(po.i, 0..=10) && po.f == 0 && po.i % 10 == 0) {
            PluralCategory::MANY
        } else if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else if (po.i == 2 && po.v == 0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && matches!(po.i % 10, 2..=4) && !matches!(po.i % 100, 12..=14))
            || (matches!(po.f % 10, 2..=4) && !matches!(po.f % 100, 12..=14))
        {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
            || (po.f % 10 == 1 && po.f % 100 != 11)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (matches!(po.f % 100, 3..=4)) {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
            PluralCategory::ONE
        } else if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0 || po.i == 1) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.t == 0 && po.i % 10 == 1 && po.i % 100 != 11) || (po.t != 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && !matches!(po.i, 0..=10) && po.f == 0 && po.i % 10 == 0) {
            PluralCategory::MANY
        } else if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else if (po.i == 2 && po.v == 0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 0 || po.i == 1) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 0.0) {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i % 100 == 3
            || po.i % 100 == 23
            || po.i % 100 == 43
            || po.i % 100 == 63
            || po.i % 100 == 83)
        {
            PluralCategory::FEW
        } else if (po.n != 1.0
            && (po.i % 100 == 1
                || po.i % 100 == 21
                || po.i % 100 == 41
                || po.i % 100 == 61
                || po.i % 100 == 81))
        {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.i % 100 == 2
            || po.i % 100 == 22
            || po.i % 100 == 42
            || po.i % 100 == 62
            || po.i % 100 == 82)
            || (po.i % 1000 == 0
                && (po.i % 100000 == 40000
                    || po.i % 100000 == 60000
                    || po.i % 100000 == 80000
                    || matches!(po.i, 1000..=20000)))
            || (po.n != 0.0 && po.i % 1000000 == 100000)
        {
            PluralCategory::TWO
        } else if (po.n == 0.0) {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if ((po.i == 0 || po.i == 1) && po.n != 0.0) {
            PluralCategory::ONE
        } else if (po.n == 0.0) {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (matches!(po.i, 2..=9) && !matches!(po.i, 11..=19)) {
            PluralCategory::FEW
        } else if (po.f != 0) {
            PluralCategory::MANY
        } else if (po.i % 10 == 1 && !matches!(po.i, 11..=19)) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i % 10 == 1 && po.i % 100 != 11)
            || (po.v == 2 && po.f % 10 == 1 && po.f % 100 != 11)
            || (po.v != 2 && po.f % 10 == 1)
        {
            PluralCategory::ONE
        } else if (po.i % 10 == 0)
            || (matches!(po.i, 11..=19))
            || (po.v == 2 && matches!(po.f % 100, 11..=19))
        {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) || (po.f % 10 == 1 && po.f % 100 != 11)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v != 0) || (po.n == 0.0) || (matches!(po.i, 2..=19)) {
            PluralCategory::FEW
        } else if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 0.0) || (matches!(po.i, 2..=10)) {
            PluralCategory::FEW
        } else if (matches!(po.i, 11..=19)) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && matches!(po.i % 10, 2..=4) && !matches!(po.i % 100, 12..=14)) {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i != 1 && matches!(po.i % 10, 0..=1))
            || (po.v == 0 && matches!(po.i % 10, 5..=9))
            || (po.v == 0 && matches!(po.i % 100, 12..=14))
        {
            PluralCategory::MANY
        } else if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i % 10 == 1 && po.i % 100 != 11)
            || (po.v == 2 && po.f % 10 == 1 && po.f % 100 != 11)
            || (po.v != 2 && po.f % 10 == 1)
        {
            PluralCategory::ONE
        } else if (po.i % 10 == 0)
            || (matches!(po.i, 11..=19))
            || (po.v == 2 && matches!(po.f % 100, 11..=19))
        {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 0..=1)) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v != 0) || (po.n == 0.0) || (matches!(po.i, 2..=19)) {
            PluralCategory::FEW
        } else if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.v == 0 && matches!(po.i % 10, 2..=4) && !matches!(po.i % 100, 12..=14)) {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 10 == 0)
            || (po.v == 0 && matches!(po.i % 10, 5..=9))
            || (po.v == 0 && matches!(po.i % 100, 11..=14))
        {
            PluralCategory::MANY
        } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.v == 0 && matches!(po.i % 10, 2..=4) && !matches!(po.i % 100, 12..=14))
            || (matches!(po.f % 10, 2..=4) && !matches!(po.f % 100, 12..=14))
        {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
            || (po.f % 10 == 1 && po.f % 100 != 11)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 2..=10) && po.f == 0) {
            PluralCategory::FEW
        } else if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 0.0 || po.n == 1.0) || (po.i == 0 && po.f == 1) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 2..=4) && po.v == 0) {
            PluralCategory::FEW
        } else if (po.v != 0) {
            PluralCategory::MANY
        } else if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (po.v != 0) {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 100 == 1) {
            PluralCategory::ONE
        } else if (po.v == 0 && po.i % 100 == 2) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && matches!(po.i % 10, 2..=4) && !matches!(po.i % 100, 12..=14))
            || (matches!(po.f % 10, 2..=4) && !matches!(po.f % 100, 12..=14))
        {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
            || (po.f % 10 == 1 && po.f % 100 != 11)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
            || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
            || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) || (matches!(po.i, 11..=99) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.v == 0 && matches!(po.i % 10, 2..=4) && !matches!(po.i % 100, 12..=14)) {
            PluralCategory::FEW
        } else if (po.v == 0 && po.i % 10 == 0)
            || (po.v == 0 && matches!(po.i % 10, 5..=9))
            || (po.v == 0 && matches!(po.i % 100, 11..=14))
        {
            PluralCategory::MANY
        } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (matches!(po.i, 0..=1) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i == 1 && po.v == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i == 0) || (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
];
pub const PRS_ORDINAL: &[PluralRule] = &[
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 4.0) {
            PluralCategory::FEW
        } else if (po.n == 6.0) {
            PluralCategory::MANY
        } else if (po.n == 1.0
            || po.n == 5.0
            || po.n == 7.0
            || po.n == 8.0
            || po.n == 9.0
            || po.n == 10.0)
        {
            PluralCategory::ONE
        } else if (po.n == 2.0 || po.n == 3.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i % 10 == 3 || po.i % 10 == 4)
            || (po.i % 1000 == 100
                || po.i % 1000 == 200
                || po.i % 1000 == 300
                || po.i % 1000 == 400
                || po.i % 1000 == 500
                || po.i % 1000 == 600
                || po.i % 1000 == 700
                || po.i % 1000 == 800
                || po.i % 1000 == 900)
        {
            PluralCategory::FEW
        } else if (po.i == 0)
            || (po.i % 10 == 6)
            || (po.i % 100 == 40 || po.i % 100 == 60 || po.i % 100 == 90)
        {
            PluralCategory::MANY
        } else if (po.i % 10 == 1
            || po.i % 10 == 2
            || po.i % 10 == 5
            || po.i % 10 == 7
            || po.i % 10 == 8)
            || (po.i % 100 == 20 || po.i % 100 == 50 || po.i % 100 == 70 || po.i % 100 == 80)
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if ((po.i % 10 == 2 || po.i % 10 == 3) && po.i % 100 != 12 && po.i % 100 != 13) {
            PluralCategory::FEW
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 4.0) {
            PluralCategory::FEW
        } else if (po.n == 6.0) {
            PluralCategory::MANY
        } else if (po.n == 1.0
            || po.n == 5.0
            || po.n == 7.0
            || po.n == 8.0
            || po.n == 9.0
            || po.n == 10.0)
        {
            PluralCategory::ONE
        } else if (po.n == 2.0 || po.n == 3.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 4.0) {
            PluralCategory::FEW
        } else if (po.n == 1.0 || po.n == 3.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 3.0 || po.n == 4.0) {
            PluralCategory::FEW
        } else if (po.n == 5.0 || po.n == 6.0) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0) {
            PluralCategory::TWO
        } else if (po.n == 0.0 || po.n == 7.0 || po.n == 8.0 || po.n == 9.0) {
            PluralCategory::ZERO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i % 10 == 3 && po.i % 100 != 13) {
            PluralCategory::FEW
        } else if (po.i % 10 == 1 && po.i % 100 != 11) {
            PluralCategory::ONE
        } else if (po.i % 10 == 2 && po.i % 100 != 12) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 3.0 || po.n == 13.0) {
            PluralCategory::FEW
        } else if (po.n == 1.0 || po.n == 11.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0 || po.n == 12.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 4.0) {
            PluralCategory::FEW
        } else if (po.n == 6.0) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0 || po.n == 3.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 4.0) {
            PluralCategory::FEW
        } else if (po.n == 6.0) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0 || po.n == 3.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0 || po.n == 5.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 11.0 || po.n == 8.0 || po.n == 80.0 || po.n == 800.0) {
            PluralCategory::MANY
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i == 0)
            || (po.i % 100 == 40
                || po.i % 100 == 60
                || po.i % 100 == 80
                || matches!(po.i % 100, 2..=20))
        {
            PluralCategory::MANY
        } else if (po.i == 1) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.i % 10 == 6) || (po.i % 10 == 9) || (po.i % 10 == 0 && po.n != 0.0) {
            PluralCategory::MANY
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 5.0) || (po.i % 100 == 5) {
            PluralCategory::MANY
        } else if (matches!(po.i, 1..=4) && po.f == 0)
            || (matches!(po.i, 1..=4)
                || matches!(po.i, 21..=24)
                || matches!(po.i, 41..=44)
                || matches!(po.i, 61..=64)
                || matches!(po.i, 81..=84))
        {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if ((po.i % 10 == 7 || po.i % 10 == 8) && po.i % 100 != 17 && po.i % 100 != 18) {
            PluralCategory::MANY
        } else if (po.i % 10 == 1 && po.i % 100 != 11) {
            PluralCategory::ONE
        } else if (po.i % 10 == 2 && po.i % 100 != 12) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 4.0) {
            PluralCategory::FEW
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else if (po.n == 2.0 || po.n == 3.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (matches!(po.i, 1..=4) && po.f == 0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 4.0) {
            PluralCategory::FEW
        } else if (po.n == 6.0) {
            PluralCategory::MANY
        } else if (po.n == 1.0 || po.n == 5.0 || matches!(po.i, 7..=9) && po.f == 0) {
            PluralCategory::ONE
        } else if (po.n == 2.0 || po.n == 3.0) {
            PluralCategory::TWO
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 11.0 || po.n == 8.0 || po.n == 80.0 || po.n == 800.0) {
            PluralCategory::MANY
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 11.0 || po.n == 8.0 || po.n == 80.0 || po.n == 800.0) {
            PluralCategory::MANY
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i % 10 == 4 && po.i % 100 != 14) {
            PluralCategory::MANY
        } else if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if ((po.i % 10 == 1 || po.i % 10 == 2) && po.i % 100 != 11 && po.i % 100 != 12) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i % 10 == 6 || po.i % 10 == 9) || (po.n == 10.0) {
            PluralCategory::FEW
        } else {
            PluralCategory::OTHER
        }
    },
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| {
        if (po.i % 10 == 3 && po.i % 100 != 13) {
            PluralCategory::FEW
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| {
        if (po.n == 1.0) {
            PluralCategory::ONE
        } else {
            PluralCategory::OTHER
        }
    },
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
    |po| PluralCategory::OTHER,
];
