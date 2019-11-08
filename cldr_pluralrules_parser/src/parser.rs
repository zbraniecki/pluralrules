use super::ast::*;
use nom::{
    branch::alt,
    //error::context,
    bytes::complete::tag,
    character::complete::{digit1, one_of, space0, space1},
    combinator::{map, map_res, opt},
    multi::{separated_list, separated_nonempty_list},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn value(i: &str) -> IResult<&str, Value> {
    map_res(digit1, |s: &str| s.parse::<usize>().map(Value))(i)
}

fn range(i: &str) -> IResult<&str, Range> {
    map(
        separated_pair(value, tag(".."), value),
        |(lower_val, upper_val)| Range {
            lower_val,
            upper_val,
        },
    )(i)
}

fn range_list_item(i: &str) -> IResult<&str, RangeListItem> {
    alt((
        map(range, RangeListItem::Range),
        map(value, RangeListItem::Value),
    ))(i)
}

fn range_list(i: &str) -> IResult<&str, RangeList> {
    map(
        separated_list(tuple((space0, tag(","), space0)), range_list_item),
        RangeList,
    )(i)
}

fn operand(i: &str) -> IResult<&str, Operand> {
    map(one_of("nivwft"), |c| match c {
        'n' => Operand::N,
        'i' => Operand::I,
        'v' => Operand::V,
        'w' => Operand::W,
        'f' => Operand::F,
        't' => Operand::T,
        _ => unreachable!(),
    })(i)
}

fn mod_expression(i: &str) -> IResult<&str, Option<Modulo>> {
    opt(map(
        preceded(tuple((space0, alt((tag("mod"), tag("%"))), space1)), value),
        Modulo,
    ))(i)
}

fn expression(i: &str) -> IResult<&str, Expression> {
    map(tuple((operand, mod_expression)), |(operand, modulus)| {
        Expression { operand, modulus }
    })(i)
}

fn relation_operator(i: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("="), |_| Operator::EQ),
        map(tag("!="), |_| Operator::NotEQ),
        map(tuple((tag("is"), space1, opt(tag("not")))), |(_, _, n)| {
            if n.is_some() {
                Operator::IsNot
            } else {
                Operator::Is
            }
        }),
        map(tag("in"), |_| Operator::In),
        map(
            tuple((
                tag("not"),
                space1,
                alt((
                    map(tag("in"), |_| Operator::NotIn),
                    map(tag("within"), |_| Operator::NotWithin),
                )),
            )),
            |(_, _, v)| v,
        ),
        map(tag("within"), |_| Operator::Within),
    ))(i)
}

fn relation(i: &str) -> IResult<&str, Relation> {
    map(
        tuple((expression, space0, relation_operator, space0, range_list)),
        |(expression, _, operator, _, range_list)| Relation {
            expression,
            operator,
            range_list,
        },
    )(i)
}

fn and_condition(i: &str) -> IResult<&str, AndCondition> {
    map(
        separated_nonempty_list(tuple((space1, tag("and"), space1)), relation),
        AndCondition,
    )(i)
}

fn decimal_value(i: &str) -> IResult<&str, DecimalValue> {
    map(
        tuple((value, opt(preceded(tag("."), value)))),
        |(integer, decimal)| DecimalValue { integer, decimal },
    )(i)
}

fn sample_range(i: &str) -> IResult<&str, SampleRange> {
    map(
        tuple((
            decimal_value,
            opt(preceded(tuple((space0, tag("~"), space0)), decimal_value)),
        )),
        |(lower_val, upper_val)| SampleRange {
            lower_val,
            upper_val,
        },
    )(i)
}

fn sample_list(i: &str) -> IResult<&str, SampleList> {
    map(
        tuple((
            separated_nonempty_list(tuple((space0, tag(","), space0)), sample_range),
            opt(preceded(
                tuple((space0, tag(","), space0)),
                alt((tag("..."), tag("…"))),
            )),
        )),
        |(l, ellipsis)| SampleList {
            sample_ranges: l,
            ellipsis: ellipsis.is_some(),
        },
    )(i)
}

fn samples(i: &str) -> IResult<&str, Option<Samples>> {
    map(
        tuple((
            opt(preceded(
                tuple((space1, tag("@integer"), space1)),
                sample_list,
            )),
            opt(preceded(
                tuple((space1, tag("@decimal"), space1)),
                sample_list,
            )),
        )),
        |(integer, decimal)| {
            if integer.is_some() || decimal.is_some() {
                Some(Samples { integer, decimal })
            } else {
                None
            }
        },
    )(i)
}

pub fn parse_rule(i: &str) -> IResult<&str, Rule> {
    map(tuple((parse_condition, samples)), |(condition, samples)| {
        Rule { condition, samples }
    })(i)
}

pub fn parse_condition(i: &str) -> IResult<&str, Condition> {
    // We need to handle empty input and/or input that is empty until sample.
    if i.trim().is_empty() {
        return IResult::Ok(("", Condition(vec![])));
    }

    if i.trim().starts_with("@") {
        return IResult::Ok(("", Condition(vec![])));
    }
    map(
        separated_nonempty_list(tuple((space1, tag("or"), space1)), and_condition),
        Condition,
    )(i)
}
