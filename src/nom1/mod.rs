mod field_expr;
mod field_parse;

use nom::types::CompleteByteSlice;
use nom::multispace;

use field_expr::{Field, FieldExpr, FixedValue, ValueType, Function};
use field_parse::field_list;

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub fields: Vec<FieldExpr>
}

#[derive(Debug, PartialEq)]
pub struct SelectStatementChild {
    pub select_statement: SelectStatement,
    pub alias: Option<String>
}

named!(pub select_statement<CompleteByteSlice, SelectStatement>,
    do_parse!(
        tag_no_case!("select") >>
        multispace >>
        field: field_list >>
        multispace >>
        tag_no_case!("from") >>
        (SelectStatement {
            fields: field
        })
    )
);

#[test]
fn field_test() {
    let sql = "select name n, t.age, 'from' as 'a', 1 as num, (select child from) child \
    from";
    let f1 = Field {
        table: None,
        name: String::from("name"),
        alias: Some(String::from("n"))
    };

    let f2 = Field {
        table: Some(String::from("t")),
        name: String::from("age"),
        alias: None
    };

    let v1 = FixedValue {
        value: String::from("from"),
        value_type: ValueType::STRING,
        alias: Some(String::from("a"))
    };
    let v2 = FixedValue {
        value: String::from("1"),
        value_type: ValueType::NUMBER,
        alias: Some(String::from("num"))
    };

    let cf1 = Field {
        table: None,
        name: String::from("child"),
        alias: None
    };
    let cfe1 = FieldExpr::Normal(cf1);
    let cfev1 = vec![cfe1];
    let s1 = SelectStatement {
        fields: cfev1
    };
    let sc1 = SelectStatementChild {
        select_statement: s1,
        alias: Some(String::from("child"))
    };

    let fexpr1 = FieldExpr::Normal(f1);
    let fexpr2 = FieldExpr::Normal(f2);
    let fexpr3 = FieldExpr::FixedValue(v1);
    let fexpr4 = FieldExpr::FixedValue(v2);
    let fexpr5 = FieldExpr::Statement(sc1);

    let fexprv = vec![fexpr1, fexpr2, fexpr3, fexpr4, fexpr5];

    let s = SelectStatement {
        fields: fexprv
    };

    let sql_slice = CompleteByteSlice(sql.as_bytes());

    assert_eq!(s, select_statement(sql_slice).unwrap().1);
}

#[test]
fn function_test() {
    let sql = "select `t`.`age` age, sum(t.total) total from";
    let sql_slice = CompleteByteSlice(sql.as_bytes());

    let f1 = Field {
        table: Some(String::from("t")),
        name: String::from("age"),
        alias: Some(String::from("age"))
    };

    let fc1 = Field {
        table: Some(String::from("t")),
        name: String::from("total"),
        alias: None
    };

    let fec1 = FieldExpr::Normal(fc1);
    let fecv = vec![fec1];

    let ff1 = Function {
        name: String::from("sum"),
        params: fecv,
        alias: Some(String::from("total"))
    };

    let fe1 = FieldExpr::Normal(f1);
    let fe2 = FieldExpr::Function(ff1);
    let fev = vec![fe1, fe2];

    let s = SelectStatement {
        fields: fev
    };

    assert_eq!(s, select_statement(sql_slice).unwrap().1);
}