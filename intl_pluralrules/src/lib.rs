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
//! use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
//! use unic_langid::LanguageIdentifier;
//!
//! let langid: LanguageIdentifier = "pl".parse().expect("Parsing failed.");
//!
//! assert!(IntlPluralRules::get_locales(PluralRuleType::CARDINAL).contains(&langid));
//!
//! let pr = IntlPluralRules::create(langid.clone(), PluralRuleType::CARDINAL).unwrap();
//! assert_eq!(pr.select(1), Ok(PluralCategory::ONE));
//! assert_eq!(pr.select("3"), Ok(PluralCategory::FEW));
//! assert_eq!(pr.select(12), Ok(PluralCategory::MANY));
//! assert_eq!(pr.select("5.0"), Ok(PluralCategory::OTHER));
//!
//! assert_eq!(pr.get_locale(), &langid);
//! ```

/// A public AST module for plural rule representations.
pub mod operands;
mod rules;

use unic_langid::LanguageIdentifier;

use crate::rules::*;

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
pub use crate::rules::CLDR_VERSION;

/// The main structure for selecting plural rules.
///
/// # Examples
///
/// ```
/// use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
/// use unic_langid::LanguageIdentifier;
///
/// let langid: LanguageIdentifier = "naq".parse().expect("Parsing failed.");
/// let pr_naq = IntlPluralRules::create(langid, PluralRuleType::CARDINAL).unwrap();
/// assert_eq!(pr_naq.select(1), Ok(PluralCategory::ONE));
/// assert_eq!(pr_naq.select("2"), Ok(PluralCategory::TWO));
/// assert_eq!(pr_naq.select(5.0), Ok(PluralCategory::OTHER));
/// ```
#[derive(Clone)]
pub struct IntlPluralRules {
    locale: LanguageIdentifier,
    function: PluralRule,
}

impl IntlPluralRules {
    /// Returns an instance of IntlPluralRules.
    ///
    /// # Examples
    /// ```
    /// use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
    /// use unic_langid::LanguageIdentifier;
    ///
    /// let langid: LanguageIdentifier = "naq".parse().expect("Parsing failed.");
    /// let pr_naq = IntlPluralRules::create(langid, PluralRuleType::CARDINAL);
    /// assert_eq!(pr_naq.is_ok(), !pr_naq.is_err());
    ///
    /// let langid: LanguageIdentifier = "xx".parse().expect("Parsing failed.");
    /// let pr_broken = IntlPluralRules::create(langid, PluralRuleType::CARDINAL);
    /// assert_eq!(pr_broken.is_err(), !pr_broken.is_ok());
    /// ```
    pub fn create<L: Into<LanguageIdentifier>>(
        langid: L,
        prt: PluralRuleType,
    ) -> Result<Self, &'static str> {
        let langid = langid.into();
        let returned_rule = rules::get_pr(&langid.to_string(), prt);
        match returned_rule {
            Ok(returned_rule) => Ok(Self {
                locale: langid,
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
    /// use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
    /// use unic_langid::LanguageIdentifier;
    ///
    /// let langid: LanguageIdentifier = "naq".parse().expect("Parsing failed.");
    /// let pr_naq = IntlPluralRules::create(langid, PluralRuleType::CARDINAL).unwrap();
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
    /// use intl_pluralrules::{IntlPluralRules, PluralRuleType};
    ///
    /// assert_eq!(
    ///     IntlPluralRules::get_locales(PluralRuleType::CARDINAL).is_empty(),
    ///     false
    /// );
    /// ```
    pub fn get_locales(prt: PluralRuleType) -> Vec<LanguageIdentifier> {
        rules::get_locales(prt)
            .iter()
            .filter(|s| *s != &"root")
            .map(|s| {
                s.parse()
                    .unwrap_or_else(|_| panic!("Parsing failed: {}.", s))
            })
            .collect()
    }

    /// Returns the locale name for this PluralRule instance.
    ///
    /// # Examples
    /// ```
    /// use intl_pluralrules::{IntlPluralRules, PluralRuleType};
    /// use unic_langid::LanguageIdentifier;
    ///
    /// let langid: LanguageIdentifier = "naq".parse().expect("Parsing failed.");
    /// let pr_naq = IntlPluralRules::create(langid.clone(), PluralRuleType::CARDINAL).unwrap();
    /// assert_eq!(pr_naq.get_locale(), &langid);
    /// ```
    pub fn get_locale(&self) -> &LanguageIdentifier {
        &self.locale
    }
}

#[cfg(test)]
mod tests {
    use super::{IntlPluralRules, PluralCategory, PluralRuleType, CLDR_VERSION};
    use unic_langid::LanguageIdentifier;

    #[test]
    fn cardinals_test() {
        let langid: LanguageIdentifier = "naq".parse().expect("Parsing failed.");
        let pr_naq = IntlPluralRules::create(langid, PluralRuleType::CARDINAL).unwrap();
        assert_eq!(pr_naq.select(1), Ok(PluralCategory::ONE));
        assert_eq!(pr_naq.select(2), Ok(PluralCategory::TWO));
        assert_eq!(pr_naq.select(5), Ok(PluralCategory::OTHER));

        let langid: LanguageIdentifier = "xx".parse().expect("Parsing failed.");
        let pr_broken = IntlPluralRules::create(langid, PluralRuleType::CARDINAL);
        assert_eq!(pr_broken.is_err(), !pr_broken.is_ok());
    }

    #[test]
    fn ordinals_rules() {
        let langid: LanguageIdentifier = "uk".parse().expect("Parsing failed.");
        let pr_naq = IntlPluralRules::create(langid, PluralRuleType::ORDINAL).unwrap();
        assert_eq!(pr_naq.select(33), Ok(PluralCategory::FEW));
        assert_eq!(pr_naq.select(113), Ok(PluralCategory::OTHER));
    }

    #[test]
    fn version_test() {
        assert_eq!(CLDR_VERSION, 36);
    }

    #[test]
    fn locale_test() {
        assert_eq!(
            IntlPluralRules::get_locales(PluralRuleType::CARDINAL).is_empty(),
            false
        );
    }
}
