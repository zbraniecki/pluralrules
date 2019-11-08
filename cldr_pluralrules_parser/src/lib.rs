//! A crate for parsing CLDR plural rules.
//!
//! This crate parses plural rules and returns an AST representation of the rule. Plural rules must be written according to the specifications outlined at [Unicode's website](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules).
//!
//! Plural rules, compatible with this crate, can be found at [this GitHub repository](https://github.com/unicode-cldr/cldr-core/blob/master/supplemental/plurals.json).
//!
//! # Examples
//!
//! ```
//! use cldr_pluralrules_parser::parse_plural_rule;
//! use cldr_pluralrules_parser::ast::*;
//!
//! let condition = Condition(vec![
//!     AndCondition(vec![Relation {
//!         expression: Expression {
//!             operand: Operand::I,
//!             modulus: None,
//!         },
//!         operator: Operator::Is,
//!         range_list: RangeList(vec![RangeListItem::Value(Value(5))]),
//!     }]),
//!     AndCondition(vec![Relation {
//!         expression: Expression {
//!             operand: Operand::V,
//!             modulus: None,
//!         },
//!         operator: Operator::Within,
//!         range_list: RangeList(vec![RangeListItem::Value(Value(2))]),
//!     }]),
//! ]);
//!
//! assert_eq!(
//!     condition,
//!     parse_plural_rule("i is 5 or v within 2")
//!         .expect("Parsing succeeded")
//!         .condition
//! )
//! ```

/// A public AST module for plural rule representations.
pub mod ast;
/// A private parsing module for plural rules.
mod parser;

/// Given a string reference of a plural rule, will return the AST representation of that rule.
///
/// # Examples
///
/// ```
/// use cldr_pluralrules_parser::parse_plural_rule;
/// use cldr_pluralrules_parser::ast::*;
///
/// let condition = Condition(vec![
///     AndCondition(vec![Relation {
///         expression: Expression {
///             operand: Operand::I,
///             modulus: None,
///         },
///         operator: Operator::Is,
///         range_list: RangeList(vec![RangeListItem::Value(Value(5))]),
///     }]),
///     AndCondition(vec![Relation {
///         expression: Expression {
///             operand: Operand::V,
///             modulus: None,
///         },
///         operator: Operator::Within,
///         range_list: RangeList(vec![RangeListItem::Value(Value(2))]),
///     }]),
/// ]);
///
/// assert_eq!(
///     condition,
///     parse_plural_rule("i is 5 or v within 2")
///         .expect("Parsing succeeded")
///         .condition
/// )
/// ```
pub fn parse_plural_rule<S: AsRef<str>>(source: S) -> Result<ast::Rule, String> {
    match parser::parse_rule(source.as_ref()) {
        Ok(("", rule)) => Ok(rule),
        //Ok((_, rule)) => Ok(rule),
        Ok((left, _)) => Err(format!("Left string: {}", left)),
        _ => Err("Parser failed".to_string()),
    }
}

pub fn parse_plural_condition<S: AsRef<str>>(source: S) -> Result<ast::Condition, String> {
    match parser::parse_condition(source.as_ref()) {
        Ok((_, rule)) => Ok(rule),
        _ => Err("Parser failed".to_string()),
    }
}
