use super::SelectStatementChild;

#[derive(Debug, PartialEq)]
pub enum FieldExpr {
    Normal(Field),
    FixedValue(FixedValue),
    Statement(SelectStatementChild)
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub table: Option<String>,
    pub name: String,
    pub alias: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct FixedValue {
    pub value: String,
    pub value_type: ValueType,
    pub alias: Option<String>
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    NUMBER,
    STRING
}