#[macro_use]
extern crate nom;

pub mod ast;
pub mod parser;

use ast::*;
use nom::types::CompleteStr;
use parser::*;

pub fn parse_plural_rule(source: &str) -> Condition {
    let stuff = parse_rule(CompleteStr(source));

    let cond = stuff.unwrap();

    cond.1
}
