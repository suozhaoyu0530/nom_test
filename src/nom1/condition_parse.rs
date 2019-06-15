use nom::types::CompleteByteSlice;
use nom::multispace0;

use super::condition_expr::{ConditionExpr, ComparisonType, Condition};
use super::field_parse::field_list;
use crate::nom1::field_expr::{Field, FieldExpr};
use crate::commons::sql_alphanumeric;

//named!(condition_list<CompleteByteSlice, Vec<ConditionExpr>>,
//    do_parse!(
//
//    )

//);

named!(condition_reference<CompleteByteSlice, Condition>,
    do_parse!(
        multispace0 >>
        left: field_list >>
        multispace0 >>
        comparison: condition_comparison >>
        multispace0 >>
        right: field_list >>
        (Condition {
            left: left,
            comparison: comparison,
            right: right
        })
    )
);

named!(condition_comparison<CompleteByteSlice, ComparisonType>,
    do_parse!(
        c: preceded!(multispace0,
            alt!(
                tag!("=") | tag!("<") | tag!(">") |
                tag!("<=") | tag!(">=") |
                tag_no_case!("in") | tag_no_case!("not in")
            )
        ) >>
        (match String::from_utf8(c.to_vec()).unwrap().to_uppercase().as_ref() {
            "=" => ComparisonType::Equal,
            "<" => ComparisonType::LessThan,
            ">" => ComparisonType::GreaterThan,
            "<=" => ComparisonType::LessEqualThan,
            ">=" => ComparisonType::GreaterEqualThan,
            "IN" => ComparisonType::In,
            "NOT IN" => ComparisonType::NotIn,
            _ => ComparisonType::UnKnown
        })
    )
);

#[test]
fn comparison_test() {
    let c1 = "=";
    let c1_slice = CompleteByteSlice(c1.as_bytes());
    let ct1 = ComparisonType::Equal;
    assert_eq!(ct1, condition_comparison(c1_slice).unwrap().1);

//    let c2 = "not in";
//    let c2_slice = CompleteByteSlice(c2.as_bytes());
//    let ct2 = ComparisonType::NotIn;
//    assert_eq!(ct2, condition_comparison(c2_slice).unwrap().1);
}

#[test]
fn codition_test() {
    let c1 = "c.std_id = st.id";
    let c1_slice = CompleteByteSlice(c1.as_bytes());

    let fl1 = Field {
        table: Some(String::from("c")),
        name: String::from("std_id"),
        alias: None
    };
    let fle1 = FieldExpr::Normal(fl1);
    let fr1 = Field {
        table: Some(String::from("st")),
        name: String::from("id"),
        alias: None
    };
    let fre1 = FieldExpr::Normal(fr1);
    let cd1 = Condition {
        left: vec![fle1],
        comparison: ComparisonType::Equal,
        right: vec![fre1]
    };

    assert_eq!(cd1, condition_reference(c1_slice).unwrap().1);
}