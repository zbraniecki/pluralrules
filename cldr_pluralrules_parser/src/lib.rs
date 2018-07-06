// pub mod parser;

#[macro_use]
extern crate nom;
// #[macro_use]
// extern crate serde_derive;
// #[macro_use]
// extern crate syn;

pub mod ast;

// use gen_pr::{PluralCategory};
use ast::*;

// use std::io;
use nom::{digit1,types::CompleteStr};
use std::str::FromStr;

// ==============
// <RULE PARSING>

// Captures integer values
named!(value<CompleteStr, Value>,
    ws!(map!(recognize!(many1!(digit1)), |recast| Value(usize::from_str(&recast.to_string()).unwrap() ) ))
);

// Captures the last half of the range
named!(range<CompleteStr,Range>,
    ws!(do_parse!(
        o : value >>
        map!(tag!(".."), |recast| recast.to_string() ) >>
        n : value >>
        (Range {
            lower_val: o,
            upper_val : n
        })
    ))
);

// Captures a numeric range (including singular values)
named!(range_list_item<CompleteStr,RangeListItem>, 
    ws!(do_parse!(
        r: alt!(
            range   => { |r| RangeListItem::RLRange(r) } |
            value   => { |v| RangeListItem::RLValue(v) }
        ) >>
        opt!(tag!(",")) >>
        (r)
    ))

);

named!(range_list<CompleteStr, RangeList >,
   ws!(map!(
        fold_many1!( range_list_item, Vec::new(), |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
        }), |recast| RangeList(recast) 
    ))
);

named!(within_operator<CompleteStr,Operator>,
    ws!(do_parse!(
        tag!("within") >>
        (Operator::Within)
    ))
);

named!(not_within_operator<CompleteStr,Operator>,
    ws!(do_parse!(
        tag!("not") >>
        tag!("within") >>
        (Operator::NotWithin)
    ))
);

// Captures in operators
named!(check_within_operator<CompleteStr,Operator>,
    ws!(alt!(within_operator | not_within_operator))
);

named!(in_operator<CompleteStr,Operator>,
    ws!(do_parse!(
        tag!("in") >>
        (Operator::In)
    ))
);

named!(not_in_operator<CompleteStr,Operator>,
    ws!(do_parse!(
        do_parse!(
                a: tag!("not") >>
                tag!("in") >>
                (a)
        ) >>
        (Operator::NotIn)
    ))
);

// Captures in operators
named!(check_in_operator<CompleteStr,Operator>,
    ws!(alt!(in_operator | not_in_operator))
);

// Capture is operators
named!(is_operator<CompleteStr,Operator>,
    ws!(do_parse!(
        tag!("is") >>
        (Operator::Is)
    ))
);

// Capture is operators
named!(not_is_operator<CompleteStr,Operator>,
    ws!(do_parse!(
        tag!("is") >>
        tag!("not") >>
        (Operator::IsNot)
    ))
);

// Capture is operators
named!(check_is_operator<CompleteStr,Operator>,
    ws!(alt!(not_is_operator | is_operator))
);

named!(eq_operator<CompleteStr,Operator>,
    ws!(do_parse!(
        tag!("=")>>
        (Operator::EQ)
    ))
);

named!(not_eq_operator<CompleteStr,Operator>,
    ws!(do_parse!(
        tag!("!=") >>
        (Operator::NotEQ)
    ))
);

// Captures in operators
named!(check_eq_operator<CompleteStr,Operator>,
    ws!(alt!(eq_operator | not_eq_operator))
);

// Captures an operand
named!(operand<CompleteStr,Operand>,
    ws!(map!(
        alt_complete!(
            tag!("n") | 
            tag!("i") | 
            tag!("v") | 
            tag!("w") | 
            tag!("f") | 
            tag!("t") ), 
        |recast| Operand (char::from_str(&recast).unwrap() ) 
    ))
);

named!(mod_expression<CompleteStr,Modulo>,
    ws!(do_parse!(
        alt_complete!( tag!("mod") | tag!("%") ) >>
        v : value >>
        (Modulo (v) )
    ))
);

// Captures an expression
named!(expression<CompleteStr,Expression>,
    ws!(do_parse!(
        rand: operand >>
        mod_expr: opt!(mod_expression) >>
        (Expression {
            operand: rand,
            modulus: mod_expr
        })
    ))
);

named!(within_relation<CompleteStr, Relation >,
    ws!(do_parse!(
        first_o : expression >>
        math_o : check_within_operator >>
        nums : range_list >>
        (Relation{
            expression: first_o,
            operator: math_o,
            range_list: nums
        })
    ))
);

named!(in_relation<CompleteStr, Relation >,
    ws!(do_parse!(
        first_o : expression >>
        math_o : check_in_operator >>
        nums : range_list >>
        (Relation{
            expression: first_o,
            operator: math_o,
            range_list: nums
        })
    ))
);

named!(is_relation<CompleteStr,Relation >,
    ws!(do_parse!(
        first_o : expression >>
        math_o : check_is_operator >>
        nums : value >>
        ( Relation {
            expression: first_o,
            operator: math_o,
            range_list: RangeList(vec![RangeListItem::RLValue(nums)] )
        })
    ))
);

named!(eq_relation<CompleteStr, Relation >,
    ws!(do_parse!(
        first_o : expression >>
        math_o : check_eq_operator >>
        nums : range_list >>
        (Relation{
            expression: first_o,
            operator: math_o,
            range_list: nums
        })
    ))
);

// Extracts plural rule lines for one language
named!(relation<CompleteStr, Relation >,
    ws!(alt_complete!(within_relation | in_relation | is_relation |
         eq_relation
    ))
);

named!(and_relation<CompleteStr,Relation >,
    ws!(do_parse!(
        opt!(tag!("and")) >>
        r: relation >>
        (r)
    ))
);

named!(and_condition<CompleteStr,AndCondition >,
    ws!(do_parse!(
        a : fold_many0!( and_relation, Vec::new(), |mut acc: Vec<_>, item| {
             acc.push(item);
             acc
         }) >>
        (AndCondition(a) )
    ))
);

named!(interm_condition<CompleteStr, AndCondition >,
    ws!(do_parse!(
        opt!(tag!("or")) >>
        s: and_condition >>
        (s)
    ))
);

named!(condition<CompleteStr, Condition >,
    ws!(do_parse!(
        a : fold_many0!( interm_condition, Vec::new(), |mut acc: Vec<_>, item| {
             acc.push(item);
             acc
         }) >>
        (Condition(a))
    ))
);

named!(parse_rule<CompleteStr,Condition >,
    ws!(call!(condition))
);

// </RULE PARSING>
// ===============

pub fn parse_plural_rule(source: &str) -> Condition {

    let stuff = parse_rule(CompleteStr(source));

    let cond = stuff.unwrap();

    cond.1
}
