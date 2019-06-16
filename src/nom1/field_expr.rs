use super::SelectStatementChild;

#[derive(Debug, PartialEq, Clone)]
pub enum FieldExpr {
    Normal(Field),
    FixedValue(FixedValue),
    Statement(SelectStatementChild),
    Function(Function)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub table: Option<String>,
    pub name: String,
    pub alias: Option<String>
}

impl Field {
    pub fn new(n: &str) -> Field {
        Field {
            table: None,
            name: String::from(n),
            alias: None
        }
    }

    pub fn new_name_table(n: &str, t: &str) -> Field {
        let mut f = Self::new(n);
        f.table = Some(String::from(t));
        f
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FixedValue {
    pub value: String,
    pub value_type: ValueType,
    pub alias: Option<String>
}

impl FixedValue {
    pub fn new(v: &str, ty: ValueType) -> Self {
        FixedValue {
            value: String::from(v),
            value_type: ty,
            alias: None
        }
    }

    pub fn new_alias(v: &str, ty: ValueType, a: &str) -> Self {
        FixedValue {
            value: String::from(v),
            value_type: ty,
            alias: Some(String::from(a))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    NUMBER,
    STRING
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<FieldExpr>,
    pub alias: Option<String>
}

#[test]
fn field_name_table() {
    let f = Field::new_name_table("name", "table");
    let f_t = Field {
        table: Some(String::from("table")),
        name: String::from("name"),
        alias: None
    };

    assert_eq!(f, f_t);
}