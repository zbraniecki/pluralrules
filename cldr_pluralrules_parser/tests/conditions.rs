use cldr_pluralrules_parser::ast::*;
use cldr_pluralrules_parser::*;

#[test]
fn simple_expression() {
    let test = "i = 5";

    assert_eq!(
        Condition(vec![AndCondition(vec![Relation {
            expression: Expression {
                operand: Operand('i'),
                modulus: None,
            },
            operator: Operator::EQ,
            range_list: RangeList(vec![RangeListItem::Value(Value(5))]),
        }])]),
        parse_plural_rule(test)
    );
}

#[test]
fn multi_value() {
    let test = "i = 5, 7, 9";

    assert_eq!(
        Condition(vec![AndCondition(vec![Relation {
            expression: Expression {
                operand: Operand('i'),
                modulus: None,
            },
            operator: Operator::EQ,
            range_list: RangeList(vec![
                RangeListItem::Value(Value(5)),
                RangeListItem::Value(Value(7)),
                RangeListItem::Value(Value(9)),
            ]),
        }])]),
        parse_plural_rule(test)
    );
}

#[test]
fn multi_range() {
    let test = "i in 5..9, 11..15";

    assert_eq!(
        Condition(vec![AndCondition(vec![Relation {
            expression: Expression {
                operand: Operand('i'),
                modulus: None,
            },
            operator: Operator::In,
            range_list: RangeList(vec![
                RangeListItem::Range(Range {
                    lower_val: Value(5),
                    upper_val: Value(9),
                }),
                RangeListItem::Range(Range {
                    lower_val: Value(11),
                    upper_val: Value(15),
                }),
            ]),
        }])]),
        parse_plural_rule(test)
    );
}

#[test]
fn and_condition() {
    let test = "i in 5 and v not in 2";

    assert_eq!(
        Condition(vec![AndCondition(vec![
            Relation {
                expression: Expression {
                    operand: Operand('i'),
                    modulus: None,
                },
                operator: Operator::In,
                range_list: RangeList(vec![RangeListItem::Value(Value(5))]),
            },
            Relation {
                expression: Expression {
                    operand: Operand('v'),
                    modulus: None,
                },
                operator: Operator::NotIn,
                range_list: RangeList(vec![RangeListItem::Value(Value(2))]),
            },
        ])]),
        parse_plural_rule(test)
    );
}

#[test]
fn or_condition() {
    let test = "i is 5 or v within 2";

    assert_eq!(
        Condition(vec![
            AndCondition(vec![Relation {
                expression: Expression {
                    operand: Operand('i'),
                    modulus: None,
                },
                operator: Operator::Is,
                range_list: RangeList(vec![RangeListItem::Value(Value(5))]),
            }]),
            AndCondition(vec![Relation {
                expression: Expression {
                    operand: Operand('v'),
                    modulus: None,
                },
                operator: Operator::Within,
                range_list: RangeList(vec![RangeListItem::Value(Value(2))]),
            }]),
        ]),
        parse_plural_rule(test)
    );
}

#[test]
fn ars_many_rule() {
    let test = "n % 100 = 11..99 @integer 11~26, 111, 1011, … @decimal 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 111.0, 1011.0, …";

    assert_eq!(
        Condition(vec![AndCondition(vec![Relation {
            expression: Expression {
                operand: Operand('n'),
                modulus: Some(Modulo(Value(100))),
            },
            operator: Operator::EQ,
            range_list: RangeList(vec![RangeListItem::Range(Range {
                lower_val: Value(11),
                upper_val: Value(99),
            })]),
        }])]),
        parse_plural_rule(test)
    );
}

#[test]
fn be_one_rule() {
    let test = "n % 10 = 1 and n % 100 != 11 @integer 1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, … @decimal 1.0, 21.0, 31.0, 41.0, 51.0, 61.0, 71.0, 81.0, 101.0, 1001.0, …";
    assert_eq!(
        Condition(vec![AndCondition(vec![
            Relation {
                expression: Expression {
                    operand: Operand('n'),
                    modulus: Some(Modulo(Value(10))),
                },
                operator: Operator::EQ,
                range_list: RangeList(vec![RangeListItem::Value(Value(1))]),
            },
            Relation {
                expression: Expression {
                    operand: Operand('n'),
                    modulus: Some(Modulo(Value(100))),
                },
                operator: Operator::NotEQ,
                range_list: RangeList(vec![RangeListItem::Value(Value(11))]),
            },
        ])]),
        parse_plural_rule(test)
    );
}

#[test]
fn be_few_rule() {
    let test = "n % 10 = 2..4 and n % 100 != 12..14 @integer 2~4, 22~24, 32~34, 42~44, 52~54, 62, 102, 1002, … @decimal 2.0, 3.0, 4.0, 22.0, 23.0, 24.0, 32.0, 33.0, 102.0, 1002.0, …";
    assert_eq!(
        Condition(vec![AndCondition(vec![
            Relation {
                expression: Expression {
                    operand: Operand('n'),
                    modulus: Some(Modulo(Value(10))),
                },
                operator: Operator::EQ,
                range_list: RangeList(vec![RangeListItem::Range(Range {
                    lower_val: Value(2),
                    upper_val: Value(4),
                })]),
            },
            Relation {
                expression: Expression {
                    operand: Operand('n'),
                    modulus: Some(Modulo(Value(100))),
                },
                operator: Operator::NotEQ,
                range_list: RangeList(vec![RangeListItem::Range(Range {
                    lower_val: Value(12),
                    upper_val: Value(14),
                })]),
            },
        ])]),
        parse_plural_rule(test)
    );
}

#[test]
fn be_many_rule() {
    let test = "n % 10 = 0 or n % 10 = 5..9 or n % 100 = 11..14 @integer 0, 5~19, 100, 1000, 10000, 100000, 1000000, … @decimal 0.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …";
    assert_eq!(
        Condition(vec![
            AndCondition(vec![Relation {
                expression: Expression {
                    operand: Operand('n'),
                    modulus: Some(Modulo(Value(10))),
                },
                operator: Operator::EQ,
                range_list: RangeList(vec![RangeListItem::Value(Value(0))]),
            }]),
            AndCondition(vec![Relation {
                expression: Expression {
                    operand: Operand('n'),
                    modulus: Some(Modulo(Value(10))),
                },
                operator: Operator::EQ,
                range_list: RangeList(vec![RangeListItem::Range(Range {
                    lower_val: Value(5),
                    upper_val: Value(9),
                })]),
            }]),
            AndCondition(vec![Relation {
                expression: Expression {
                    operand: Operand('n'),
                    modulus: Some(Modulo(Value(100))),
                },
                operator: Operator::EQ,
                range_list: RangeList(vec![RangeListItem::Range(Range {
                    lower_val: Value(11),
                    upper_val: Value(14),
                })]),
            }]),
        ]),
        parse_plural_rule(test)
    );
}
