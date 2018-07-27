//! A crate for generating plural rule operands from numberical input.
//!
//! This crate generates plural operands according to the specifications outlined at [Unicode's website](http://unicode.org/reports/tr35/tr35-numbers.html#Operands).
//!
//! Input is supported for int, float, and &str.
//!
//! # Examples
//!
//! From int
//!
//! ```
//! use intl_pluralrules::operands::*;
//! assert_eq!(Ok(PluralOperands {
//!    n: 2_f64,
//!    i: 2,
//!    v: 0,
//!    w: 0,
//!    f: 0,
//!    t: 0,
//! }), PluralOperands::from(2))
//! ```
//!
//! From float
//!
//! ```
//! use intl_pluralrules::operands::*;
//! assert_eq!(Ok(PluralOperands {
//!    n: 1234.567_f64,
//!    i: 1234,
//!    v: 3,
//!    w: 3,
//!    f: 567,
//!    t: 567,
//! }), PluralOperands::from("-1234.567"))
//! ```
//!
//! From &str
//!
//! ```
//! use intl_pluralrules::operands::*;
//! assert_eq!(Ok(PluralOperands {
//!    n: 123.45_f64,
//!    i: 123,
//!    v: 2,
//!    w: 2,
//!    f: 45,
//!    t: 45,
//! }), PluralOperands::from(123.45))
//! ```
#[macro_use]
extern crate matches;

/// A public AST module for plural rule representations.
pub mod operands;
mod rules;

use rules::*;

#[derive(Debug, Eq, PartialEq)]
pub enum PluralCategory {
    ZERO,
    ONE,
    TWO,
    FEW,
    MANY,
    OTHER,
}

pub fn get_cldr_version() -> usize {
    rules::CLDR_VERSION
}

pub fn get_locales<'a>(prt : PluralRuleType) -> &'a [&'static str] {
    rules::get_locales(prt)
}

pub fn get_locale_rules() -> Result<PluralRule, ()> {
    unimplemented!();
}

pub fn select<N: ToString>(lang: &str, number: N, prt : PluralRuleType) -> Result<PluralCategory, &'static str> {
    let f = rules::get_pr(lang, prt);
    match f {
        Ok(pr) => {
            let ops = operands::PluralOperands::from(number);
            match ops {
                Ok(ops) => Ok(pr(ops)),
                Err(_) => Err("Argument can not be parsed to operands."),
            }
        }
        Err(_) => Err("unknown locale"),
    }
}

#[cfg(test)]
mod tests {
    use super::{get_cldr_version, get_locales, select, PluralCategory};
    use rules::*;
    #[test]
    fn it_works() {
        assert_eq!(select("naq", 1, PluralRuleType::CARDINAL), Ok(PluralCategory::ONE));
        assert_eq!(select("naq", 2, PluralRuleType::CARDINAL), Ok(PluralCategory::TWO));
        assert_eq!(select("naq", 5, PluralRuleType::CARDINAL), Ok(PluralCategory::OTHER));
        assert_eq!(!select("quan", 5, PluralRuleType::CARDINAL).is_ok(), select("quan", 5, PluralRuleType::CARDINAL).is_err());
    }
    #[test]
    fn version_test() {
        assert_eq!(get_cldr_version(), 33);
    }
    #[test]
    fn locale_test() {
        static TEST_ARRAY: &[&'static str] = &[
            "af", "ak", "am", "ar", "ars", "as", "asa", "ast", "az", "be", "bem", "bez", "bg",
            "bh", "bm", "bn", "bo", "br", "brx", "bs", "ca", "ce", "cgg", "chr", "ckb", "cs", "cy",
            "da", "de", "dsb", "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu", "fa", "ff",
            "fi", "fil", "fo", "fr", "fur", "fy", "ga", "gd", "gl", "gsw", "gu", "guw", "gv", "ha",
            "haw", "he", "hi", "hr", "hsb", "hu", "hy", "id", "ig", "ii", "in", "io", "is", "it",
            "iu", "iw", "ja", "jbo", "jgo", "ji", "jmc", "jv", "jw", "ka", "kab", "kaj", "kcg",
            "kde", "kea", "kk", "kkj", "kl", "km", "kn", "ko", "ks", "ksb", "ksh", "ku", "kw",
            "ky", "lag", "lb", "lg", "lkt", "ln", "lo", "lt", "lv", "mas", "mg", "mgo", "mk", "ml",
            "mn", "mo", "mr", "ms", "mt", "my", "nah", "naq", "nb", "nd", "ne", "nl", "nn", "nnh",
            "no", "nqo", "nr", "nso", "ny", "nyn", "om", "or", "os", "pa", "pap", "pl", "prg",
            "ps", "pt", "pt-PT", "rm", "ro", "rof", "root", "ru", "rwk", "sah", "saq", "scn", "sd",
            "sdh", "se", "seh", "ses", "sg", "sh", "shi", "si", "sk", "sl", "sma", "smi", "smj",
            "smn", "sms", "sn", "so", "sq", "sr", "ss", "ssy", "st", "sv", "sw", "syr", "ta", "te",
            "teo", "th", "ti", "tig", "tk", "tl", "tn", "to", "tr", "ts", "tzm", "ug", "uk", "ur",
            "uz", "ve", "vi", "vo", "vun", "wa", "wae", "wo", "xh", "xog", "yi", "yo", "yue", "zh",
            "zu",
        ];

        assert_eq!(get_locales(PluralRuleType::CARDINAL), TEST_ARRAY);
    }
}
