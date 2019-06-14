use nom::types::CompleteByteSlice;
use nom::multispace0;

use crate::commons::{sql_alphanumeric, as_alias, number_alphanumeric};

use super::field_expr::{Field, FieldExpr, FixedValue, ValueType, Function};
use super::{SelectStatementChild, select_statement};
use crate::nom1::SelectStatement;

named!(pub field_list<CompleteByteSlice, Vec<FieldExpr>>,
    many0!(
        alt!(
            do_parse!(
                f: function_reference >>
                opt!(
                    do_parse!(
                        multispace0 >>
                        tag!(",") >>
                        multispace0 >>
                        ()
                    )
                ) >>
                (FieldExpr::Function(f))
            )
            | do_parse!(
                statement: statement_reference >>
                opt!(
                    do_parse!(
                        multispace0 >>
                        tag!(",") >>
                        multispace0 >>
                        ()
                    )
                ) >>
                (FieldExpr::Statement(statement))
            )
            | do_parse!(
                field: fixed_reference >>
                opt!(
                    do_parse!(
                        multispace0 >>
                        tag!(",") >>
                        multispace0 >>
                        ()
                    )
                ) >>
                (FieldExpr::FixedValue(field))
            )
            | do_parse!(
                field: field_reference >>
                opt!(
                    do_parse!(
                        multispace0 >>
                        tag!(",") >>
                        multispace0 >>
                        ()
                    )
                ) >>
                (FieldExpr::Normal(field))
            )
        )
    )
);

named!(field_reference<CompleteByteSlice, Field>,
    do_parse!(
        table: opt!(
            do_parse!(
                opt!(tag!("`")) >>
                tbl_name: sql_alphanumeric >>
                opt!(tag!("`")) >>
                tag!(".") >>
                (String::from_utf8(tbl_name.to_vec()).unwrap())
            )
        ) >>
        opt!(tag!("`")) >>
        name: sql_alphanumeric >>
        opt!(tag!("`")) >>
        alias: opt!(as_alias) >>
        (Field{
            table: table,
            name: String::from_utf8(name.to_vec()).unwrap(),
            alias: alias.map(|a| a.to_string())
        })
    )
);

named!(fixed_reference<CompleteByteSlice, FixedValue>,
    alt!(
        do_parse!(
            alt!(tag!("'") | tag!("\"")) >>
            value: sql_alphanumeric >>
            alt!(tag!("'") | tag!("\"")) >>
            alias: opt!(as_alias) >>
            (FixedValue {
                value: String::from_utf8(value.to_vec()).unwrap(),
                value_type: ValueType::STRING,
                alias: alias.map(|a| a.to_string())
            })
        )
        | do_parse!(
            value: number_alphanumeric >>
            alias: opt!(as_alias) >>
            (FixedValue {
                value: String::from_utf8(value.to_vec()).unwrap(),
                value_type: ValueType::NUMBER,
                alias: alias.map(|a| a.to_string())
            })
        )
    )
);

named!(statement_reference<CompleteByteSlice, SelectStatementChild>,
    do_parse!(
        tag!("(") >>
        statement: select_statement >>
        tag!(")") >>
        alias: opt!(as_alias) >>
        (SelectStatementChild {
            select_statement: statement,
            alias: alias.map(|a| a.to_string())
        })
    )
);

named!(function_reference<CompleteByteSlice, Function>,
    do_parse!(
        name: sql_alphanumeric >>
        tag!("(") >>
        fields: field_list >>
        tag!(")") >>
        alias: opt!(as_alias) >>
        (Function {
            name: String::from_utf8(name.to_vec()).unwrap(),
            params: fields,
            alias: alias.map(|a| a.to_string())
        })
    )
);

#[test]
fn field_test() {
    let field = "name n, t.age, 'from' as 'a', 1 as num, (select child from) child";
    let field_slice = CompleteByteSlice(field.as_bytes());

    let f1 = Field {
        table: None,
        name: String::from("name"),
        alias: Some(String::from("n"))
    };
    let fe1 = FieldExpr::Normal(f1);

    let f2 = Field {
        table: Some(String::from("t")),
        name: String::from("age"),
        alias: None
    };
    let fe2 = FieldExpr::Normal(f2);

    let f3 = FixedValue {
        value: String::from("from"),
        value_type: ValueType::STRING,
        alias: Some(String::from("a"))
    };
    let fe3 = FieldExpr::FixedValue(f3);

    let f4 = FixedValue {
        value: String::from("1"),
        value_type: ValueType::NUMBER,
        alias: Some(String::from("num"))
    };
    let fe4 = FieldExpr::FixedValue(f4);

    let fc1 = Field {
        table: None,
        name: String::from("child"),
        alias: None
    };
    let fec1 = FieldExpr::Normal(fc1);
    let fecv1 = vec![fec1];
    let s1 = SelectStatement {
        fields: fecv1
    };
    let sc1 = SelectStatementChild {
        select_statement: s1,
        alias: Some(String::from("child"))
    };
    let fe5 = FieldExpr::Statement(sc1);

    let fev = vec![fe1, fe2, fe3, fe4, fe5];

    assert_eq!(fev, field_list(field_slice).unwrap().1);
}