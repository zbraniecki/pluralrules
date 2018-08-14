//! Plural operands in compliance with [CLDR Plural Rules](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules).
//!
//! See [full operands description](http://unicode.org/reports/tr35/tr35-numbers.html#Operands).
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
use std::isize;
use std::str::FromStr;

/// A full plural operands representation of a number. See [CLDR Plural Rules](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules) for complete operands description.
#[derive(Debug, PartialEq)]
pub struct PluralOperands {
    /// Absolute value of input
    pub n: f64,
    /// Integer value of input
    pub i: isize,
    /// Number of visible fraction digits with trailing zeros
    pub v: isize,
    /// Number of visible fraction digits without trailing zeros
    pub w: isize,
    /// Visible fraction digits with trailing zeros
    pub f: isize,
    /// Visible fraction digits without trailing zeros
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
    pub fn from<S: IntoPluralOperands>(num: S) -> Result<Self, &'static str> {
        num.into_plural()
    }
}

// once TryFrom stabilizes we can use that instead
/// A trait that can be implemented on any type to allow it for passing to [`select`](../struct.IntlPluralRules.html#method.select). This trait is made public for implementations of custom types. If you are using generic types, you should use [`from`](struct.PluralOperands.html#method.from).
/// 
/// # Example
///
/// ```
/// use intl_pluralrules::{IntlPluralRules, PluralRuleType, PluralCategory};
///
/// struct MyType {
///     value: isize
/// }
///
/// impl IntoPluralOperands for MyType {
///     fn into_plural(self) -> Result<PluralOperands, &'static str> {
///         Ok(PluralOperands {
///             n: self.value as f64,
///             i: self.value as isize,
///             v: 0,
///             w: 0,
///             f: 0,
///             t: 0,
///         })
///     }
/// }
///
/// let pr = IntlPluralRules::create("en", PluralRuleType::CARDINAL).unwrap();
/// let v = MyType { value: 5 };
///
/// assert_eq!(pr.select(v), Ok(PluralCategory::OTHER));
///
/// ```
pub trait IntoPluralOperands {
    fn into_plural(self) -> Result<PluralOperands, &'static str>;
}


impl<'a> IntoPluralOperands for &'a str {
    fn into_plural(self) -> Result<PluralOperands, &'static str> {
        let abs_str = if self.starts_with("-") {
            &self[1..]
        } else {
            &self
        };

        let absolute_value = f64::from_str(&abs_str).map_err(|_| "Incorrect number passed!")?;

        let integer_digits;
        let num_fraction_digits0;
        let num_fraction_digits;
        let fraction_digits0;
        let fraction_digits;

        if let Some(dec_pos) = abs_str.find('.') {
            let int_str = &abs_str[..dec_pos];
            let dec_str = &abs_str[(dec_pos + 1)..];

            integer_digits =
                isize::from_str(&int_str).map_err(|_| "Could not convert string to integer!")?;

            let backtrace = dec_str.trim_right_matches('0');

            num_fraction_digits0 = dec_str.chars().count() as isize;
            num_fraction_digits = backtrace.chars().count() as isize;
            fraction_digits0 =
                isize::from_str(dec_str).map_err(|_| "Could not convert string to integer!")?;
            fraction_digits = isize::from_str(backtrace).unwrap_or(0);
        } else {
            integer_digits = absolute_value as isize;
            num_fraction_digits0 = 0;
            num_fraction_digits = 0;
            fraction_digits0 = 0;
            fraction_digits = 0;
        }

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

macro_rules! impl_integer_type {
    ($ty:ident) => {
        impl IntoPluralOperands for $ty {
            fn into_plural(self) -> Result<PluralOperands, &'static str> {
                // XXXManishearth converting from u32 or u64 to isize may wrap
                Ok(PluralOperands {
                    n: self as f64,
                    i: self as isize,
                    v: 0,
                    w: 0,
                    f: 0,
                    t: 0,
                })
            }
        }
    };
    ($($ty:ident)+) => {
        $(impl_integer_type!($ty);)+
    };
}

macro_rules! impl_signed_integer_type {
    ($ty:ident) => {
        impl IntoPluralOperands for $ty {
            fn into_plural(self) -> Result<PluralOperands, &'static str> {
                // XXXManishearth converting from i64 to isize may wrap
                let x = (self as isize).checked_abs().ok_or("Number too big")?;
                Ok(PluralOperands {
                    n: x as f64,
                    i: x as isize,
                    v: 0,
                    w: 0,
                    f: 0,
                    t: 0,
                })
            }
        }
    };
    ($($ty:ident)+) => {
        $(impl_signed_integer_type!($ty);)+
    };
}

macro_rules! impl_convert_type {
    ($ty:ident) => {
        impl IntoPluralOperands for $ty {
            fn into_plural(self) -> Result<PluralOperands, &'static str> {
                <&str>::into_plural(&*self.to_string())
            }
        }
    };
    ($($ty:ident)+) => {
        $(impl_convert_type!($ty);)+
    };
}

impl_integer_type!(u8 u16 u32 u64 usize);
impl_signed_integer_type!(i8 i16 i32 i64 isize);
// XXXManishearth we can likely have dedicated float impls here
impl_convert_type!(f32 f64 String);
