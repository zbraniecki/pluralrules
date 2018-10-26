//! A crate for generating plural rule operands from numberical input.
//!
//! This crate generates plural operands according to the specifications outlined at [Unicode's website](http://unicode.org/reports/tr35/tr35-numbers.html#Operands).
//!
//! Input is supported for int, float, and &str.
//!
//! # Examples
//!
//! Plural rules example for Polish
//!
//! ```
//! extern crate intl_pluralrules;
//! use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
//!
//! let locale_code = "pl";
//!  
//! assert!(IntlPluralRules::get_locales(PluralRuleType::CARDINAL).contains(&locale_code));
//!
//! let pr = IntlPluralRules::create(locale_code, PluralRuleType::CARDINAL).unwrap();
//! assert_eq!(pr.select(1), Ok(PluralCategory::ONE));
//! assert_eq!(pr.select("3"), Ok(PluralCategory::FEW));
//! assert_eq!(pr.select(12), Ok(PluralCategory::MANY));
//! assert_eq!(pr.select("5.0"), Ok(PluralCategory::OTHER));
//!
//! assert_eq!(pr.get_locale(), locale_code);
//! ```
#[macro_use]
extern crate matches;
extern crate phf;

/// A public AST module for plural rule representations.
pub mod operands;
mod rules;

use rules::*;

/// A public enum for handling the plural category.
/// Each plural category will vary, depending on the language that is being used and whether that language has that plural category.
#[derive(Debug, Eq, PartialEq)]
pub enum PluralCategory {
    ZERO,
    ONE,
    TWO,
    FEW,
    MANY,
    OTHER,
}

/// A public enum for handling plural type.
#[derive(Copy, Clone)]
pub enum PluralRuleType {
    /// Ordinal numbers express position or rank in a sequence. [More about oridinal numbers](https://en.wikipedia.org/wiki/Ordinal_number_(linguistics))
    ORDINAL,
    /// Cardinal numbers are natural numbers. [More about cardinal numbers](https://en.wikipedia.org/wiki/Cardinal_number)
    CARDINAL,
}

// pub use rules::PluralRuleType;
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
#[derive(Clone)]
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
    pub fn select<N: operands::IntoPluralOperands>(
        &self,
        number: N,
    ) -> Result<PluralCategory, &'static str> {
        let ops = operands::PluralOperands::from(number);
        let pr = self.function;
        match ops {
            Ok(ops) => Ok(pr(&ops)),
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
        assert_eq!(CLDR_VERSION, 34);
    }

    #[test]
    fn locale_test() {
        assert_eq!(
            IntlPluralRules::get_locales(PluralRuleType::CARDINAL).is_empty(),
            false
        );
    }
}
