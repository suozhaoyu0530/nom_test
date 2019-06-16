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
    pub left: FieldExpr,
    pub comparison: ComparisonType,
    pub right: FieldExpr,
    pub next: Option<NextType>
}

impl Condition {
    pub fn new(l: FieldExpr, c: ComparisonType, r: FieldExpr) -> Condition {
        Condition {
            left: l,
            comparison: c,
            right: r,
            next: None
        }
    }

    pub fn new_next(l: FieldExpr, c: ComparisonType, r: FieldExpr, n: NextType) -> Condition {
        let mut condition = Self::new(l, c, r);
        condition.next = Some(n);
        condition
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConditionExpr {
    Normal(Condition),
    Parentheses(ConditionParentheses)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionParentheses {
    pub condition: Box<ConditionExpr>,
    pub next: Option<NextType>
}

impl ConditionParentheses {
    pub fn new(c: ConditionExpr) -> Self {
        ConditionParentheses {
            condition: Box::new(c),
            next: None
        }
    }

    pub fn new_next(c: ConditionExpr, n: NextType) -> Self {
        ConditionParentheses {
            condition: Box::new(c),
            next: Some(n)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NextType {
    And(Box<ConditionExpr>),
    Or(Box<ConditionExpr>),
    UnKnown
}