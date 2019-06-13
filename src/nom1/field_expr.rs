use super::SelectStatementChild;

#[derive(Debug, PartialEq)]
pub enum FieldExpr {
    Normal(Field),
    FixedValue(FixedValue),
    Statement(SelectStatementChild),
    Function(Function)
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

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<FieldExpr>,
    pub alias: Option<String>
}