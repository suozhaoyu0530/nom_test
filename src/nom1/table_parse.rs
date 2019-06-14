use nom::types::CompleteByteSlice;
use nom::multispace0;

use super::table_expr::{TableExpr, Table};
use super::{select_statement, SelectStatementChild};

use crate::commons::{as_alias, sql_alphanumeric};
use crate::nom1::field_expr::{Field, FieldExpr};
use crate::nom1::SelectStatement;

named!(pub table_list<CompleteByteSlice, Vec<TableExpr>>,
    many0!(
        alt!(
            do_parse!(
                statement: statement_reference >>
                opt!(
                    do_parse!(
                        multispace0 >>
                        tag!(",") >>
                        multispace0 >>
                        ()
                    )
                ) >>
                (TableExpr::Statement(statement))
            )
            | do_parse!(
                table: table_reference >>
                opt!(
                    do_parse!(
                        multispace0 >>
                        tag!(",") >>
                        multispace0 >>
                        ()
                    )
                ) >>
                (TableExpr::Normal(table))
            )
        )
    )
);

named!(table_reference<CompleteByteSlice, Table>,
    do_parse!(
        opt!(tag!("`")) >>
        name: sql_alphanumeric >>
        opt!(tag!("`")) >>
        alias: opt!(as_alias) >>
        (Table {
            name: String::from_utf8(name.to_vec()).unwrap(),
            alias: alias.map(|a| a.to_string())
        })
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

#[test]
fn normal_table() {
    let table = "class c, student";
    let table_slice = CompleteByteSlice(table.as_bytes());

    let t1 = Table {
        name: String::from("class"),
        alias: Some(String::from("c"))
    };
    let te1 = TableExpr::Normal(t1);

    let t2 = Table {
        name: String::from("student"),
        alias: None
    };
    let te2 = TableExpr::Normal(t2);

    let tev = vec![te1, te2];

    assert_eq!(tev, table_list(table_slice).unwrap().1);
}

#[test]
fn statement_table() {
    let table = "(select name from t) child";
    let table_slice = CompleteByteSlice(table.as_bytes());

    let fc1 = Field {
        table: None,
        name: String::from("name"),
        alias: None
    };
    let fec1 = FieldExpr::Normal(fc1);
    let fecv = vec![fec1];

    let ft1 = Table {
        name: String::from("t"),
        alias: None
    };
    let fet1 = TableExpr::Normal(ft1);
    let fectv = vec![fet1];

    let s = SelectStatement::new(fecv, fectv);
    let sc = SelectStatementChild {
        select_statement: s,
        alias: Some(String::from("child"))
    };

    let fet = TableExpr::Statement(sc);
    let fetv = vec![fet];

    assert_eq!(fetv, table_list(table_slice).unwrap().1);
}

#[test]
fn mix_table() {
    let table = "student as st, `class` c, (select t.name from teacher) tname";
    let table_slice = CompleteByteSlice(table.as_bytes());

    let t1 = Table {
        name: String::from("student"),
        alias: Some(String::from("st"))
    };
    let te1 = TableExpr::Normal(t1);

    let t2 = Table {
        name: String::from("class"),
        alias: Some(String::from("c"))
    };
    let te2 = TableExpr::Normal(t2);

    let fc1 = Field {
        table: Some(String::from("t")),
        name: String::from("name"),
        alias: None
    };
    let fec1 = FieldExpr::Normal(fc1);
    let fecv = vec![fec1];
    let tc1 = Table {
        name: String::from("teacher"),
        alias: None
    };
    let tec1 = TableExpr::Normal(tc1);
    let tecv = vec![tec1];
    let s = SelectStatement::new(fecv, tecv);
    let sc = SelectStatementChild {
        select_statement: s,
        alias: Some(String::from("tname"))
    };
    let te3 = TableExpr::Statement(sc);

    let tev = vec![te1, te2, te3];

    assert_eq!(tev, table_list(table_slice).unwrap().1);
}
