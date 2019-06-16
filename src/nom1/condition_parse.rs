use nom::types::CompleteByteSlice;
use nom::multispace0;

use super::condition_expr::{ConditionExpr, ComparisonType,
                            Condition, NextType, ConditionParentheses};
use super::field_parse::field_list;
use crate::nom1::field_expr::{Field, FieldExpr, FixedValue, ValueType};

named!(condition_list<CompleteByteSlice, ConditionExpr>,
    alt!(
        do_parse!(
            condition: condition_reference >>
            (ConditionExpr::Normal(condition))
        )
        | do_parse!(
            condition: parentheses_condition >>
            (ConditionExpr::Parentheses(condition))
        )
    )
);

named!(condition_reference<CompleteByteSlice, Condition>,
    do_parse!(
        multispace0 >>
        left: field_list >>
        multispace0 >>
        comparison: condition_comparison >>
        multispace0 >>
        right: field_list >>
        next: opt!(next_condition) >>
        (Condition {
            left: left[0].clone(),
            comparison: comparison,
            right: right[0].clone(),
            next: next
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

named!(next_condition<CompleteByteSlice, NextType>,
    do_parse!(
        ty: preceded!(multispace0,
            alt!(
                tag_no_case!("and") | tag_no_case!("or")
            )
        ) >>
        multispace0 >>
        c: condition_list >>
        (match String::from_utf8(ty.to_vec()).unwrap().to_uppercase().as_ref() {
            "AND" => NextType::And(Box::new(c)),
            "OR" => NextType::Or(Box::new(c)),
            _ => NextType::UnKnown
        })
    )
);

named!(parentheses_condition<CompleteByteSlice, ConditionParentheses>,
    do_parse!(
        tag!("(") >>
        condition: condition_list >>
        tag!(")") >>
        n: opt!(next_condition) >>
        (ConditionParentheses {
            condition: Box::new(condition),
            next: match n {
                Some(ne) => Some(ne),
                None => None
            }
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
        left: fle1,
        comparison: ComparisonType::Equal,
        right: fre1,
        next: None
    };

    assert_eq!(cd1, condition_reference(c1_slice).unwrap().1);
}

#[test]
fn simple_condition() {
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

    let cd = Condition {
        left: fle1,
        comparison: ComparisonType::Equal,
        right: fre1,
        next: None
    };

    let ce = ConditionExpr::Normal(cd);

    assert_eq!(ce, condition_list(c1_slice).unwrap().1)
}

#[test]
fn and_condition() {
    let sql = "c.std_id = st.id and c.teacher_id=t.id";
    let sql_slice = CompleteByteSlice(sql.as_bytes());

    let fl1 = Field::new_name_table("std_id", "c");
    let fr1 = Field::new_name_table("id", "st");
    let fle1 = FieldExpr::Normal(fl1);
    let fre1 = FieldExpr::Normal(fr1);

    let fl2 = Field::new_name_table("teacher_id", "c");
    let fr2 = Field::new_name_table("id", "t");
    let fle2 = FieldExpr::Normal(fl2);
    let fre2 = FieldExpr::Normal(fr2);

    let c2 = Condition::new(fle2, ComparisonType::Equal, fre2);
    let ce = ConditionExpr::Normal(c2);

    let n = NextType::And(Box::new(ce));

    let c1 = Condition::new_next(fle1, ComparisonType::Equal, fre1, n);

    let ce_t = ConditionExpr::Normal(c1);

    assert_eq!(ce_t, condition_list(sql_slice).unwrap().1);
}

#[test]
fn or_condition() {
    let sql = "c.std_id = st.id or c.teacher_id=t.id";
    let sql_slice = CompleteByteSlice(sql.as_bytes());

    let fl1 = Field::new_name_table("std_id", "c");
    let fr1 = Field::new_name_table("id", "st");
    let fle1 = FieldExpr::Normal(fl1);
    let fre1 = FieldExpr::Normal(fr1);

    let fl2 = Field::new_name_table("teacher_id", "c");
    let fr2 = Field::new_name_table("id", "t");
    let fle2 = FieldExpr::Normal(fl2);
    let fre2 = FieldExpr::Normal(fr2);

    let c2 = Condition::new(fle2, ComparisonType::Equal, fre2);
    let ce = ConditionExpr::Normal(c2);

    let n = NextType::Or(Box::new(ce));

    let c1 = Condition::new_next(fle1, ComparisonType::Equal, fre1, n);

    let ce_t = ConditionExpr::Normal(c1);

    assert_eq!(ce_t, condition_list(sql_slice).unwrap().1);
}

#[test]
fn parentheses() {
    let sql = "((c.std_id=st.id))";
    let sql_slice = CompleteByteSlice(sql.as_bytes());

    let fl1 = Field::new_name_table("std_id", "c");
    let fle1 = FieldExpr::Normal(fl1);
    let fr1 = Field::new_name_table("id", "st");
    let fre1 = FieldExpr::Normal(fr1);

    let c = Condition::new(fle1, ComparisonType::Equal, fre1);
    let ce1 = ConditionExpr::Normal(c);
    let p1 = ConditionParentheses {
        condition: Box::new(ce1),
        next: None
    };
    let ce2 = ConditionExpr::Parentheses(p1);

    let p2 = ConditionParentheses {
        condition: Box::new(ce2),
        next: None
    };
    let ce3 = ConditionExpr::Parentheses(p2);

    assert_eq!(ce3, condition_list(sql_slice).unwrap().1);
}

#[test]
fn parentheses_next() {
    let sql = "st.name='John' and (c.std_id=st.id or c.teacher_id=t.id)";
    let sql_slice = CompleteByteSlice(sql.as_bytes());

    let fl1 = Field::new_name_table("name", "st");
    let fle1 = FieldExpr::Normal(fl1);
    let fr1 = FixedValue::new("John", ValueType::STRING);
    let fre1 = FieldExpr::FixedValue(fr1);

    let fl2 = Field::new_name_table("std_id", "c");
    let fle2 = FieldExpr::Normal(fl2);
    let fr2 = Field::new_name_table("id", "st");
    let fre2 = FieldExpr::Normal(fr2);

    let fl3 = Field::new_name_table("teacher_id", "c");
    let fle3 = FieldExpr::Normal(fl3);
    let fr3 = Field::new_name_table("id", "t");
    let fre3 = FieldExpr::Normal(fr3);

    let c2 = Condition::new(fle2, ComparisonType::Equal, fre2);
    let c3 = Condition::new(fle3, ComparisonType::Equal, fre3);

    let ce3 = ConditionExpr::Normal(c3);
    let ce2 = ConditionExpr::Normal(c2);

    let n2 = NextType::Or(Box::new(ce3));
    let p2 = ConditionParentheses::new_next(ce2, n2);
    let cp2 = ConditionExpr::Parentheses(p2);

    let n1 = NextType::And(Box::new(cp2));
    let c1 = Condition::new_next(fle1, ComparisonType::Equal, fre1, n1);
    let ce1 = ConditionExpr::Normal(c1);

    assert_eq!(ce1, condition_list(sql_slice).unwrap().1);
}