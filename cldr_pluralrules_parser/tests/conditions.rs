extern crate cldr_pluralrules_parser;

use cldr_pluralrules_parser::ast::*;
use cldr_pluralrules_parser::*;

#[test]
fn simple_expression() {
    let test = "i = 5";

    assert_eq!(
        Condition (vec![
            AndCondition (vec![
                Relation { 
                    expression: Expression {
                        operand: Operand('i'),
                        modulus: None
                    },
                    operator: Operator::EQ,
                    range_list: RangeList(vec![
                            RangeListItem::RLValue(
                                Value(5)
                        )
                    ])
                }
            ])
        ]),
        parse_plural_rule(test)
    );
}

#[test]
fn multi_value() {
    let test = "i = 5, 7, 9";

    assert_eq!(
        Condition (vec![
            AndCondition (vec![
                Relation { 
                    expression: Expression {
                        operand: Operand('i'),
                        modulus: None
                    },
                    operator: Operator::EQ,
                    range_list: RangeList(vec![
                        RangeListItem::RLValue(
                            Value(5)
                        ),
                        RangeListItem::RLValue(
                            Value(7)
                        ),
                        RangeListItem::RLValue(
                            Value(9)
                        )
                    ])
                }
            ])
        ]),
        parse_plural_rule(test)
    );
}

#[test]
fn multi_range() {
    let test = "i in 5..9, 11..15";

    assert_eq!(
        Condition (vec![
            AndCondition (vec![
                Relation { 
                    expression: Expression {
                        operand: Operand('i'),
                        modulus: None
                    },
                    operator: Operator::In,
                    range_list: RangeList(vec![
                        RangeListItem::RLRange(
                            Range {
                                lower_val: Value(5),
                                upper_val: Value(9)
                            }
                        ),
                        RangeListItem::RLRange(
                            Range {
                                lower_val: Value(11),
                                upper_val: Value(15)
                            }
                        )
                    ])
                }
            ])
        ]),
        parse_plural_rule(test)
    );
}

#[test]
fn and_condition() {
    let test = "i in 5 and v not in 2";

    assert_eq!(
        Condition (vec![
            AndCondition (vec![
                Relation { 
                    expression: Expression {
                        operand: Operand('i'),
                        modulus: None
                    },
                    operator: Operator::In,
                    range_list: RangeList(vec![
                            RangeListItem::RLValue(
                                Value(5)
                        )
                    ])
                },
                Relation { 
                    expression: Expression {
                        operand: Operand('v'),
                        modulus: None
                    },
                    operator: Operator::NotIn,
                    range_list: RangeList(vec![
                            RangeListItem::RLValue(
                                Value(2)
                        )
                    ])
                }
            ])
        ]),
        parse_plural_rule(test)
    );
}

#[test]
fn or_condition() {
    let test = "i is 5 or v within 2";

    assert_eq!(
        Condition (vec![
            AndCondition (vec![
                Relation { 
                    expression: Expression {
                        operand: Operand('i'),
                        modulus: None
                    },
                    operator: Operator::Is,
                    range_list: RangeList(vec![
                            RangeListItem::RLValue(
                                Value(5)
                        )
                    ])
                }
            ]),
            AndCondition (vec![
                Relation { 
                    expression: Expression {
                        operand: Operand('v'),
                        modulus: None
                    },
                    operator: Operator::Within,
                    range_list: RangeList(vec![
                            RangeListItem::RLValue(
                                Value(2)
                        )
                    ])
                }
            ])
        ]),
        parse_plural_rule(test)
    );
}