#[derive(Debug, Clone, PartialEq)]
pub struct Condition(pub Vec<AndCondition>);

#[derive(Debug, Clone, PartialEq)]
pub struct AndCondition(pub Vec<Relation>);

#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
    pub expression: Expression,
    pub operator: Operator,
    pub range_list: RangeList,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    In,
    NotIn,
    Within,
    NotWithin,
    Is,
    IsNot,
    EQ,
    NotEQ,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub operand: Operand,
    pub modulus: Option<Modulo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modulo(pub Value);

#[derive(Debug, Clone, PartialEq)]
pub struct Operand(pub char);

#[derive(Debug, Clone, PartialEq)]
pub struct RangeList(pub Vec<RangeListItem>);

#[derive(Debug, Clone, PartialEq)]
pub enum RangeListItem {
    Range(Range),
    Value(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub lower_val: Value,
    pub upper_val: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value(pub usize);
