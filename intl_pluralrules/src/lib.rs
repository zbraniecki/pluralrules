//! A crate for generating plural rule operands from numberical input.
//!
//! This crate generates plural operands according to the specifications outlined at [Unicode's website](http://unicode.org/reports/tr35/tr35-numbers.html#Operands).
//!
//! Input is supported for int, float, and &str.
//!
//! # Examples
//!
//! ```
//! extern crate intl_pluralrules;
//! use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
//!
//! let permanent: &'static str = "naq";
//! let locale_code = &permanent;
//!  
//! assert!(IntlPluralRules::get_locales(PluralRuleType::CARDINAL).contains(&permanent));
//!
//! let pr_naq = IntlPluralRules::create(locale_code, PluralRuleType::CARDINAL).unwrap();
//! assert_eq!(pr_naq.select(1), Ok(PluralCategory::ONE));
//! assert_eq!(pr_naq.select("2"), Ok(PluralCategory::TWO));
//! assert_eq!(pr_naq.select(5.0), Ok(PluralCategory::OTHER));
//!
//! assert_eq!(pr_naq.get_locale(), "naq");
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

pub use rules::PluralRuleType;
/// CLDR_VERSION is the version of CLDR extracted from the file used to generate rules.rs.
pub use rules::CLDR_VERSION;

/// The main structure for selecting plural rules.
///
/// # Examples
///
/// ```
/// extern crate intl_pluralrules;
/// use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
///
/// let pr_naq = IntlPluralRules::create("naq", PluralRuleType::CARDINAL).unwrap();
/// assert_eq!(pr_naq.select(1), Ok(PluralCategory::ONE));
/// assert_eq!(pr_naq.select("2"), Ok(PluralCategory::TWO));
/// assert_eq!(pr_naq.select(5.0), Ok(PluralCategory::OTHER));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct IntlPluralRules {
    locale: String,
    function: PluralRule,
}

impl IntlPluralRules {
    /// Returns an instance of IntlPluralRules.
    ///
    /// # Examples
    /// ```
    /// extern crate intl_pluralrules;
    /// use intl_pluralrules::{IntlPluralRules, PluralRuleType};
    ///
    /// let pr_naq = IntlPluralRules::create("naq", PluralRuleType::CARDINAL);
    /// assert_eq!(pr_naq.is_ok(), !pr_naq.is_err());
    ///
    /// let pr_broken = IntlPluralRules::create("test", PluralRuleType::CARDINAL);
    /// assert_eq!(pr_broken.is_err(), !pr_broken.is_ok());
    /// ```
    pub fn create(lang: &str, prt: PluralRuleType) -> Result<Self, &'static str> {
        let returned_rule = rules::get_pr(lang, prt);
        match returned_rule {
            Ok(returned_rule) => Ok(Self {
                locale: lang.to_string(),
                function: returned_rule,
            }),
            Err(_) => Err("unknown locale"),
        }
    }

    /// Returns a result of the plural category for the given input.
    ///
    /// If the input is not numeric.
    ///
    /// # Examples
    /// ```
    /// extern crate intl_pluralrules;
    /// use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
    ///
    /// let pr_naq = IntlPluralRules::create("naq", PluralRuleType::CARDINAL).unwrap();
    /// assert_eq!(pr_naq.select(1), Ok(PluralCategory::ONE));
    /// assert_eq!(pr_naq.select(2), Ok(PluralCategory::TWO));
    /// assert_eq!(pr_naq.select(5), Ok(PluralCategory::OTHER));
    /// ```
    pub fn select<N: ToString>(&self, number: N) -> Result<PluralCategory, &'static str> {
        let ops = operands::PluralOperands::from(number);
        let pr = self.function;
        match ops {
            Ok(ops) => Ok(pr(ops)),
            Err(_) => Err("Argument can not be parsed to operands."),
        }
    }

    /// Returns a list of the available locales.
    ///
    /// # Examples
    /// ```
    /// extern crate intl_pluralrules;
    /// use intl_pluralrules::{IntlPluralRules, PluralRuleType};
    ///
    /// assert_eq!(
    ///     IntlPluralRules::get_locales(PluralRuleType::CARDINAL).is_empty(),
    ///     false
    /// );
    /// ```
    pub fn get_locales(prt: PluralRuleType) -> &'static [&'static str] {
        rules::get_locales(prt)
    }

    /// Returns the locale name for this PluralRule instance.
    ///
    /// # Examples
    /// ```
    /// extern crate intl_pluralrules;
    /// use intl_pluralrules::{IntlPluralRules, PluralRuleType};
    ///
    /// let pr_naq = IntlPluralRules::create("naq", PluralRuleType::CARDINAL).unwrap();
    /// assert_eq!(pr_naq.get_locale(), "naq");
    /// ```
    pub fn get_locale(&self) -> &str {
        &self.locale
    }
}

#[cfg(test)]
mod tests {
    use super::{IntlPluralRules, PluralCategory, PluralRuleType, CLDR_VERSION};

    #[test]
    fn cardinals_test() {
        let pr_naq = IntlPluralRules::create("naq", PluralRuleType::CARDINAL).unwrap();
        assert_eq!(pr_naq.select(1), Ok(PluralCategory::ONE));
        assert_eq!(pr_naq.select(2), Ok(PluralCategory::TWO));
        assert_eq!(pr_naq.select(5), Ok(PluralCategory::OTHER));

        let pr_broken = IntlPluralRules::create("test", PluralRuleType::CARDINAL);
        assert_eq!(pr_broken.is_err(), !pr_broken.is_ok());
    }

    #[test]
    fn ordinals_rules() {
        let pr_naq = IntlPluralRules::create("uk", PluralRuleType::ORDINAL).unwrap();
        assert_eq!(pr_naq.select(33), Ok(PluralCategory::FEW));
        assert_eq!(pr_naq.select(113), Ok(PluralCategory::OTHER));
    }

    #[test]
    fn version_test() {
        assert_eq!(CLDR_VERSION, 33);
    }

    #[test]
    fn locale_test() {
        assert_eq!(
            IntlPluralRules::get_locales(PluralRuleType::CARDINAL).is_empty(),
            false
        );
    }
}
