use super::field_expr::FieldExpr;

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonType {
    Equal,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
    In,
    NotIn,
    UnKnown
}

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub left: Vec<FieldExpr>,
    pub comparison: ComparisonType,
    pub right: Vec<FieldExpr>
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConditionExpr {
    Normal(Condition)
}