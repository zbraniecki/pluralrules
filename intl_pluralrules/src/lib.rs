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
//! assert_eq!(PluralOperands {
//!    n: 2_f64,
//!    i: 2,
//!    v: 0,
//!    w: 0,
//!    f: 0,
//!    t: 0,
//! }, PluralOperands::new(2))
//! ```
//!
//! From float
//!
//! ```
//! use intl_pluralrules::operands::*;
//! assert_eq!(PluralOperands {
//!    n: 1234.567_f64,
//!    i: 1234,
//!    v: 3,
//!    w: 3,
//!    f: 567,
//!    t: 567,
//! }, PluralOperands::new("-1234.567"))
//! ```
//!
//! From &str
//!
//! ```
//! use intl_pluralrules::operands::*;
//! assert_eq!(PluralOperands {
//!    n: 123.45_f64,
//!    i: 123,
//!    v: 2,
//!    w: 2,
//!    f: 45,
//!    t: 45,
//! }, PluralOperands::new(123.45))
//! ```
#[macro_use]
extern crate matches;

/// A public AST module for plural rule representations.
pub mod operands;
mod rules;

#[derive(Debug, Eq, PartialEq)]
pub enum PluralCategory {
    ZERO,
    ONE,
    TWO,
    FEW,
    MANY,
    OTHER,
}

pub fn select_plural_category<S: ToString>(lang: &str, number: S) -> PluralCategory {
    let f = rules::get_pr(lang);
    let ops = operands::PluralOperands::new(number);
    f(ops)
}

#[cfg(test)]
mod tests {
    use super::select_plural_category;
    use super::PluralCategory;
    #[test]
    fn it_works() {
        assert_eq!(select_plural_category("naq", 1), PluralCategory::ONE);
        assert_eq!(select_plural_category("naq", 2), PluralCategory::TWO);
        assert_eq!(select_plural_category("naq", 5), PluralCategory::OTHER);
    }
}
