//! Given a string reference of a plural rule, will return the AST representation of that rule.
//!
//! # Examples
//!
//! ```
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
//! ])
//!
//! assert_eq!(condition, parse_plural_rule("i is 5 or v within 2"))
//! ```
#[macro_use]
extern crate nom;

/// A public AST module for plural rule representations.
pub mod ast;
/// A private parsing module for plural rules.
pub mod parser;

use ast::*;
use parser::*;
use nom::types::CompleteStr;

/// Given a string reference of a plural rule, will return the AST representation of that rule.
///
/// # Examples
///
/// ```
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
/// ])
///
/// assert_eq!(condition, parse_plural_rule("i is 5 or v within 2"))
/// ```
pub fn parse_plural_rule(source: &str) -> Condition {
    parse_rule(CompleteStr(source)).unwrap().1
}
