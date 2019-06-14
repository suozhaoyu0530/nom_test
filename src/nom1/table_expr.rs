use super::SelectStatementChild;

#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub alias: Option<String>
}

#[derive(Debug, PartialEq)]
pub enum TableExpr {
    Normal(Table),
    Statement(SelectStatementChild)
}