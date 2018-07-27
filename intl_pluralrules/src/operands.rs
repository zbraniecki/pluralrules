//! Plural operands in compliance with [CLDR Plural Rules](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules).
//!
//! # Example Plural Operands
//!
//! See [full operands description](http://unicode.org/reports/tr35/tr35-numbers.html#Operands).
//!
//! | n | i | v | w | f | t |
//! | - | - | - | - | - | - |
//! | 1.230 | 1 | 3 | 2 | 230 | 23 |

use std::isize;
use std::str::FromStr;

/// A full plural operands representation of a number. See [CLDR Plural Rules](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules) for complete operands description.
#[derive(Debug, PartialEq)]
pub struct PluralOperands {
    /// Absolute value of input
    pub n: f64,
    /// Integer value of input
    pub i: isize,
    /// Number of visible fraction digits with zeros
    pub v: isize,
    /// Number of visible fraction digits without zeros
    pub w: isize,
    /// Visible fraction digits with zeros
    pub f: isize,
    /// Visible fraction digits without zeros
    pub t: isize,
}

impl PluralOperands {
    /// Given numerical input (as numeric type or reference), returns the PluralOperands representation of the input.
    ///
    /// # Example
    ///
    /// ```
    /// use intl_pluralrules::operands::*;
    /// assert_eq!(Ok(PluralOperands {
    ///    n: 123.45_f64,
    ///    i: 123,
    ///    v: 2,
    ///    w: 2,
    ///    f: 45,
    ///    t: 45,
    /// }), PluralOperands::from(123.45))
    /// ```
    pub fn from<S: ToString>(num: S) -> Result<Self, &'static str> {
        let mut str_num: String = num.to_string();

        if str_num.starts_with("-") {
            str_num.remove(0);
        }

        let absolute_value = match f64::from_str(&str_num) {
            Ok(value) => value,
            Err(_) => return Err("Incorrect number passed!"),
        };

        let v: Vec<&str> = str_num.split('.').collect();

        let int_str = v.get(0).unwrap();
        let dec_str = v.get(1).unwrap_or(&"");

        let integer_digits = match isize::from_str(int_str) {
            Ok(integer_digits) => integer_digits,
            Err(_) => return Err("Failed to parse the integer to isize"),
        };

        let (num_fraction_digits0, num_fraction_digits, fraction_digits0, fraction_digits) =
            if dec_str.is_empty() {
                (0, 0, 0, 0)
            } else {
                let backtrace = dec_str.trim_right_matches('0');
                (
                    dec_str.chars().count() as isize,
                    backtrace.chars().count() as isize,
                    isize::from_str(dec_str).unwrap(),
                    isize::from_str(&backtrace).unwrap_or(0),
                )
            };

        Ok(PluralOperands {
            n: absolute_value,
            i: integer_digits,
            v: num_fraction_digits0,
            w: num_fraction_digits,
            f: fraction_digits0,
            t: fraction_digits,
        })
    }
}
