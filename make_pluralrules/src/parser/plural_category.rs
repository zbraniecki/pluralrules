//! Plural rule categories in compliance with [Unicode](https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules)

/// An enum for all plural rule categories.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PluralCategory {
    ZERO,
    ONE,
    TWO,
    FEW,
    MANY,
    OTHER,
}
