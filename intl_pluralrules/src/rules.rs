#![allow(unused_variables, unused_parens)]
extern crate matches;
use super::operands::PluralOperands;
use super::PluralCategory;
type PluralRule = fn(PluralOperands) -> PluralCategory;
pub static CLDR_VERSION: usize = 33;
pub static LOCALES: &[&'static str] = &[
    "af", "ak", "am", "ar", "ars", "as", "asa", "ast", "az", "be", "bem", "bez", "bg", "bh", "bm",
    "bn", "bo", "br", "brx", "bs", "ca", "ce", "cgg", "chr", "ckb", "cs", "cy", "da", "de", "dsb",
    "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu", "fa", "ff", "fi", "fil", "fo", "fr",
    "fur", "fy", "ga", "gd", "gl", "gsw", "gu", "guw", "gv", "ha", "haw", "he", "hi", "hr", "hsb",
    "hu", "hy", "id", "ig", "ii", "in", "io", "is", "it", "iu", "iw", "ja", "jbo", "jgo", "ji",
    "jmc", "jv", "jw", "ka", "kab", "kaj", "kcg", "kde", "kea", "kk", "kkj", "kl", "km", "kn",
    "ko", "ks", "ksb", "ksh", "ku", "kw", "ky", "lag", "lb", "lg", "lkt", "ln", "lo", "lt", "lv",
    "mas", "mg", "mgo", "mk", "ml", "mn", "mo", "mr", "ms", "mt", "my", "nah", "naq", "nb", "nd",
    "ne", "nl", "nn", "nnh", "no", "nqo", "nr", "nso", "ny", "nyn", "om", "or", "os", "pa", "pap",
    "pl", "prg", "ps", "pt", "ptPT", "rm", "ro", "rof", "root", "ru", "rwk", "sah", "saq", "scn",
    "sd", "sdh", "se", "seh", "ses", "sg", "sh", "shi", "si", "sk", "sl", "sma", "smi", "smj",
    "smn", "sms", "sn", "so", "sq", "sr", "ss", "ssy", "st", "sv", "sw", "syr", "ta", "te", "teo",
    "th", "ti", "tig", "tk", "tl", "tn", "to", "tr", "ts", "tzm", "ug", "uk", "ur", "uz", "ve",
    "vi", "vo", "vun", "wa", "wae", "wo", "xh", "xog", "yi", "yo", "yue", "zh", "zu",
];
pub fn get_pr(lang_code: &str) -> Result<PluralRule, ()> {
    let lang: &str = &str::replace(&lang_code, "-", "");
    match lang {
        "af" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ak" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "am" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ar" => Ok(|po| {
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
        }),
        "ars" => Ok(|po| {
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
        }),
        "as" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "asa" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ast" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "az" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "be" => Ok(|po| {
            if (matches!(po.i, 2..=4) && matches!(po.i, 12..=14)) {
                PluralCategory::FEW
            } else if (po.i % 10 == 0) || (matches!(po.i, 5..=9)) || (matches!(po.i, 11..=14)) {
                PluralCategory::MANY
            } else if (po.i % 10 == 1 && po.i % 100 != 11) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "bem" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "bez" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "bg" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "bh" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "bm" => Ok(|po| PluralCategory::OTHER),
        "bn" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "bo" => Ok(|po| PluralCategory::OTHER),
        "br" => Ok(|po| {
            if ((po.i % 10 == 9 || matches!(po.i, 3..=4))
                && matches!(po.i, 10..=19)
                && matches!(po.i, 70..=79)
                && matches!(po.i, 90..=99))
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
        }),
        "brx" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "bs" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14))
                || (matches!(po.f % 10, 2..=4) && matches!(po.f % 100, 12..=14))
            {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ca" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ce" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "cgg" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "chr" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ckb" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "cs" => Ok(|po| {
            if (matches!(po.i, 2..=4) && po.v == 0) {
                PluralCategory::FEW
            } else if (po.v != 0) {
                PluralCategory::MANY
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "cy" => Ok(|po| {
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
        }),
        "da" => Ok(|po| {
            if (po.n == 1.0) || (po.t != 0 && (po.i == 0 || po.i == 1)) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "de" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "dsb" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (matches!(po.f % 100, 3..=4)) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "dv" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "dz" => Ok(|po| PluralCategory::OTHER),
        "ee" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "el" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "en" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "eo" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "es" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "et" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "eu" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "fa" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ff" => Ok(|po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "fi" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "fil" => Ok(|po| {
            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "fo" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "fr" => Ok(|po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "fur" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "fy" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ga" => Ok(|po| {
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
        }),
        "gd" => Ok(|po| {
            if (matches!(po.i, 3..=10) && po.f == 0 || matches!(po.i, 13..=19) && po.f == 0) {
                PluralCategory::FEW
            } else if (po.n == 1.0 || po.n == 11.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0 || po.n == 12.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "gl" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "gsw" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "gu" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "guw" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "gv" => Ok(|po| {
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
        }),
        "ha" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "haw" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "he" => Ok(|po| {
            if (po.v == 0 && matches!(po.i, 0..=10) && po.f == 0 && po.i % 10 == 0) {
                PluralCategory::MANY
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.i == 2 && po.v == 0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "hi" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "hr" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14))
                || (matches!(po.f % 10, 2..=4) && matches!(po.f % 100, 12..=14))
            {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "hsb" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (matches!(po.f % 100, 3..=4)) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "hu" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "hy" => Ok(|po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "id" => Ok(|po| PluralCategory::OTHER),
        "ig" => Ok(|po| PluralCategory::OTHER),
        "ii" => Ok(|po| PluralCategory::OTHER),
        "in" => Ok(|po| PluralCategory::OTHER),
        "io" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "is" => Ok(|po| {
            if (po.t == 0 && po.i % 10 == 1 && po.i % 100 != 11) || (po.t != 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "it" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "iu" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "iw" => Ok(|po| {
            if (po.v == 0 && matches!(po.i, 0..=10) && po.f == 0 && po.i % 10 == 0) {
                PluralCategory::MANY
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.i == 2 && po.v == 0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "ja" => Ok(|po| PluralCategory::OTHER),
        "jbo" => Ok(|po| PluralCategory::OTHER),
        "jgo" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ji" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "jmc" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "jv" => Ok(|po| PluralCategory::OTHER),
        "jw" => Ok(|po| PluralCategory::OTHER),
        "ka" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "kab" => Ok(|po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "kaj" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "kcg" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "kde" => Ok(|po| PluralCategory::OTHER),
        "kea" => Ok(|po| PluralCategory::OTHER),
        "kk" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "kkj" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "kl" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "km" => Ok(|po| PluralCategory::OTHER),
        "kn" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ko" => Ok(|po| PluralCategory::OTHER),
        "ks" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ksb" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ksh" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 0.0) {
                PluralCategory::ZERO
            } else {
                PluralCategory::OTHER
            }
        }),
        "ku" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "kw" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "ky" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "lag" => Ok(|po| {
            if ((po.i == 0 || po.i == 1) && po.n != 0.0) {
                PluralCategory::ONE
            } else if (po.n == 0.0) {
                PluralCategory::ZERO
            } else {
                PluralCategory::OTHER
            }
        }),
        "lb" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "lg" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "lkt" => Ok(|po| PluralCategory::OTHER),
        "ln" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "lo" => Ok(|po| PluralCategory::OTHER),
        "lt" => Ok(|po| {
            if (matches!(po.i, 2..=9) && matches!(po.i, 11..=19)) {
                PluralCategory::FEW
            } else if (po.f != 0) {
                PluralCategory::MANY
            } else if (po.i % 10 == 1 && matches!(po.i, 11..=19)) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "lv" => Ok(|po| {
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
        }),
        "mas" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "mg" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "mgo" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "mk" => Ok(|po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ml" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "mn" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "mo" => Ok(|po| {
            if (po.v != 0) || (po.n == 0.0) || (po.n != 1.0 && matches!(po.i, 1..=19)) {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "mr" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ms" => Ok(|po| PluralCategory::OTHER),
        "mt" => Ok(|po| {
            if (po.n == 0.0) || (matches!(po.i, 2..=10)) {
                PluralCategory::FEW
            } else if (matches!(po.i, 11..=19)) {
                PluralCategory::MANY
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "my" => Ok(|po| PluralCategory::OTHER),
        "nah" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "naq" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "nb" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "nd" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ne" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "nl" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "nn" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "nnh" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "no" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "nqo" => Ok(|po| PluralCategory::OTHER),
        "nr" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "nso" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ny" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "nyn" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "om" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "or" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "os" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "pa" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "pap" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "pl" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14)) {
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
        }),
        "prg" => Ok(|po| {
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
        }),
        "ps" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "pt" => Ok(|po| {
            if (matches!(po.i, 0..=1)) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ptPT" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "rm" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ro" => Ok(|po| {
            if (po.v != 0) || (po.n == 0.0) || (po.n != 1.0 && matches!(po.i, 1..=19)) {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "rof" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "root" => Ok(|po| PluralCategory::OTHER),
        "ru" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14)) {
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
        }),
        "rwk" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sah" => Ok(|po| PluralCategory::OTHER),
        "saq" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "scn" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sd" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sdh" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "se" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "seh" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ses" => Ok(|po| PluralCategory::OTHER),
        "sg" => Ok(|po| PluralCategory::OTHER),
        "sh" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14))
                || (matches!(po.f % 10, 2..=4) && matches!(po.f % 100, 12..=14))
            {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "shi" => Ok(|po| {
            if (matches!(po.i, 2..=10) && po.f == 0) {
                PluralCategory::FEW
            } else if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "si" => Ok(|po| {
            if (po.n == 0.0 || po.n == 1.0) || (po.i == 0 && po.f == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sk" => Ok(|po| {
            if (matches!(po.i, 2..=4) && po.v == 0) {
                PluralCategory::FEW
            } else if (po.v != 0) {
                PluralCategory::MANY
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sl" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (po.v != 0) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && po.i % 100 == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "sma" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "smi" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "smj" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "smn" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "sms" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        }),
        "sn" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "so" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sq" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sr" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14))
                || (matches!(po.f % 10, 2..=4) && matches!(po.f % 100, 12..=14))
            {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ss" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ssy" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "st" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sv" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "sw" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "syr" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ta" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "te" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "teo" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "th" => Ok(|po| PluralCategory::OTHER),
        "ti" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "tig" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "tk" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "tl" => Ok(|po| {
            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "tn" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "to" => Ok(|po| PluralCategory::OTHER),
        "tr" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ts" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "tzm" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) || (matches!(po.i, 11..=99) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ug" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "uk" => Ok(|po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14)) {
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
        }),
        "ur" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "uz" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "ve" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "vi" => Ok(|po| PluralCategory::OTHER),
        "vo" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "vun" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "wa" => Ok(|po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "wae" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "wo" => Ok(|po| PluralCategory::OTHER),
        "xh" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "xog" => Ok(|po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "yi" => Ok(|po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        "yo" => Ok(|po| PluralCategory::OTHER),
        "yue" => Ok(|po| PluralCategory::OTHER),
        "zh" => Ok(|po| PluralCategory::OTHER),
        "zu" => Ok(|po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        }),
        _ => Err(()),
    }
}
