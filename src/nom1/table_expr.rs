use super::SelectStatementChild;
use super::condition_expr::ConditionExpr;

use crate::nom1::SelectStatement;

#[derive(Debug, PartialEq, Clone)]
pub struct Table {
    pub name: String,
    pub alias: Option<String>,
    pub join: Option<Box<TableJoin>>
}

impl Table {
    pub fn new(n: &str) -> Table {
        Table {
            name: String::from(n),
            alias: None,
            join: None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableExpr {
    Normal(Table),
    Statement(SelectStatementChild)
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementChildJoin {
    pub select_statement: SelectStatement,
    pub alias: Option<String>,
    pub join: Option<TableJoin>
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableJoin {
    pub join_type: JoinType,
    pub table_expr: TableExpr,
    pub on: ConditionExpr
}

#[derive(Debug, PartialEq, Clone)]
pub enum JoinType {
    LEFT,
    RIGHT,
    INNER
}