use nom::types::CompleteByteSlice;
use nom::multispace0;

use crate::commons::{sql_alphanumeric, as_alias, number_alphanumeric};

use super::field_expr::{Field, FieldExpr, FixedValue, ValueType, Function};
use super::{SelectStatementChild, select_statement};

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