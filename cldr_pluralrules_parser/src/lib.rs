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
//!             operand: Operand('i'),
//!             modulus: None,
//!         },
//!         operator: Operator::Is,
//!         range_list: RangeList(vec![RangeListItem::Value(Value(5))]),
//!     }]),
//!     AndCondition(vec![Relation {
//!         expression: Expression {
//!             operand: Operand('v'),
//!             modulus: None,
//!         },
//!         operator: Operator::Within,
//!         range_list: RangeList(vec![RangeListItem::Value(Value(2))]),
//!     }]),
//! ]);
//!
//! assert_eq!(condition, parse_plural_rule("i is 5 or v within 2"))
//! ```
#[macro_use]
extern crate nom;

/// A public AST module for plural rule representations.
pub mod ast;
/// A private parsing module for plural rules.
mod parser;

use ast::*;
use nom::types::CompleteStr;
use parser::*;

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
///             operand: Operand('i'),
///             modulus: None,
///         },
///         operator: Operator::Is,
///         range_list: RangeList(vec![RangeListItem::Value(Value(5))]),
///     }]),
///     AndCondition(vec![Relation {
///         expression: Expression {
///             operand: Operand('v'),
///             modulus: None,
///         },
///         operator: Operator::Within,
///         range_list: RangeList(vec![RangeListItem::Value(Value(2))]),
///     }]),
/// ]);
///
/// assert_eq!(condition, parse_plural_rule("i is 5 or v within 2"))
/// ```
pub fn parse_plural_rule(source: &str) -> Condition {
    parse_rule(CompleteStr(source)).unwrap().1
}
