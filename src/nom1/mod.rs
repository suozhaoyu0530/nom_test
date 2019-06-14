mod field_expr;
mod field_parse;
mod table_expr;
mod table_parse;

use nom::types::CompleteByteSlice;
use nom::multispace;

use field_expr::FieldExpr;
use field_parse::field_list;
use table_parse::table_list;

use crate::nom1::table_expr::TableExpr;

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub fields: Vec<FieldExpr>,
    pub tables: Vec<TableExpr>
}

#[derive(Debug, PartialEq)]
pub struct SelectStatementChild {
    pub select_statement: SelectStatement,
    pub alias: Option<String>
}

impl SelectStatement {
    pub fn new(f: Vec<FieldExpr>, t: Vec<TableExpr>) -> SelectStatement {
        SelectStatement {
            fields: f,
            tables: t
        }
    }
}

named!(pub select_statement<CompleteByteSlice, SelectStatement>,
    do_parse!(
        tag_no_case!("select") >>
        multispace >>
        field: field_list >>
        multispace >>
        tag_no_case!("from") >>
        multispace >>
        table: table_list >>
        (SelectStatement {
            fields: field,
            tables: table
        })
    )
);