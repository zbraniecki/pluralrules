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

pub use rules::get_locales;
pub use rules::CLDR_VERSION;

#[derive(Debug, Clone, PartialEq)]
pub struct IntlPluralRules {
    function: PluralRule,
}

impl IntlPluralRules {
    pub fn create(lang: &str, prt: PluralRuleType) -> Result<Self, &'static str> {
        let returned_rule = rules::get_pr(lang, prt);
        match returned_rule {
            Ok(returned_rule) => Ok(Self {
                function: returned_rule,
            }),
            Err(_) => Err("unknown locale"),
        }
    }

    pub fn select<N: ToString>(&self, number: N) -> Result<PluralCategory, &'static str> {
        let ops = operands::PluralOperands::from(number);
        let pr = self.function;
        match ops {
            Ok(ops) => Ok(pr(ops)),
            Err(_) => Err("Argument can not be parsed to operands."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_locales, IntlPluralRules, PluralCategory, CLDR_VERSION};
    use rules::*;
    #[test]
    fn it_works() {
        let pr_naq = IntlPluralRules::create("naq", PluralRuleType::CARDINAL).unwrap();
        assert_eq!(pr_naq.select(1), Ok(PluralCategory::ONE));
        assert_eq!(pr_naq.select(2), Ok(PluralCategory::TWO));
        assert_eq!(pr_naq.select(5), Ok(PluralCategory::OTHER));

        let pr_broken = IntlPluralRules::create("test", PluralRuleType::CARDINAL);
        assert_eq!(pr_broken.is_err(), !pr_broken.is_ok());
    }
    #[test]
    fn version_test() {
        assert_eq!(CLDR_VERSION, 33);
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
