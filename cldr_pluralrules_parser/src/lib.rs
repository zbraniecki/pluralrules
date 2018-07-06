#[macro_use]
extern crate nom;

pub mod ast;
pub mod parser;

use ast::*;
use parser::*;
use nom::types::CompleteStr;

/// Given a string reference of a plural rule, will return the AST representation of that rule.
pub fn parse_plural_rule(source: &str) -> Condition {
    parse_rule(CompleteStr(source)).unwrap().1
}
