mod field_expr;
mod field_parse;
mod table_expr;
mod table_parse;
mod condition_expr;
mod condition_parse;

use nom::types::CompleteByteSlice;
use nom::{multispace, multispace0};

use field_expr::FieldExpr;
use field_parse::field_list;
use table_expr::TableExpr;
use table_parse::table_list;
use condition_expr::ConditionExpr;
use condition_parse::condition_list;
use crate::nom1::field_expr::{Field, FixedValue, ValueType};
use crate::nom1::table_expr::Table;
use crate::nom1::condition_expr::{Condition, ComparisonType};

#[derive(Debug, PartialEq, Clone)]
pub struct SelectStatement {
    pub fields: Vec<FieldExpr>,
    pub tables: Vec<TableExpr>,
    pub wheres: Option<Box<ConditionExpr>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct SelectStatementChild {
    pub select_statement: SelectStatement,
    pub alias: Option<String>
}

impl SelectStatement {
    pub fn new(f: Vec<FieldExpr>, t: Vec<TableExpr>) -> SelectStatement {
        SelectStatement {
            fields: f,
            tables: t,
            wheres: None
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
        wheres: opt!(
            do_parse!(
                multispace0 >>
                tag_no_case!("where") >>
                w: condition_list >>
                (w)
            )
        ) >>
        (SelectStatement {
            fields: field,
            tables: table,
            wheres: match wheres {
                Some(w) => Some(Box::new(w)),
                None => None
            }
        })
    )
);

#[test]
fn simple_sql() {
    let sql = "select * from teacher where 1=1";
    let sql_slice = CompleteByteSlice(sql.as_bytes());

    let f = Field::new("*");
    let fe = FieldExpr::Normal(f);

    let t = Table::new("teacher");
    let te = TableExpr::Normal(t);

    let fwl = FixedValue::new("1", ValueType::NUMBER);
    let fwel = FieldExpr::FixedValue(fwl);
    let fwr = FixedValue::new("1", ValueType::NUMBER);
    let fwer = FieldExpr::FixedValue(fwr);
    let c = Condition::new(fwel, ComparisonType::Equal, fwer);
    let ce = ConditionExpr::Normal(c);

    let s = SelectStatement {
        fields: vec![fe],
        tables: vec![te],
        wheres: Some(Box::new(ce))
    };

    assert_eq!(s, select_statement(sql_slice).unwrap().1);
}