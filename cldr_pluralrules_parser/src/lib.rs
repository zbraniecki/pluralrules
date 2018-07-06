#[macro_use]
extern crate nom;

pub mod ast;
pub mod parser;

use ast::*;
use parser::*;
use nom::types::CompleteStr;

/// Calls on parser.rs parse_rule function to parse a CLDR plural rule and get back an AST built with ast.rs struct Condition
pub fn parse_plural_rule(source: &str) -> Condition {
    parse_rule(CompleteStr(source)).unwrap().1
}
