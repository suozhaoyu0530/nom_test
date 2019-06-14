mod keyword;

use nom::{multispace, is_alphanumeric, is_digit};
use nom::types::CompleteByteSlice;

use keyword::sql_keyword;

named!(pub sql_alphanumeric<CompleteByteSlice, CompleteByteSlice>,
    do_parse!(
        not!(peek!(sql_keyword)) >>
        ident: take_while1!(is_sql_alphanumeric) >>
        (ident)
    )
);

named!(pub sql_alphanumeric_nokey<CompleteByteSlice, CompleteByteSlice>,
    do_parse!(
        ident: take_while1!(is_sql_alphanumeric) >>
        (ident)
    )
);

named!(pub number_alphanumeric<CompleteByteSlice, CompleteByteSlice>,
    do_parse!(
        ident: take_while1!(is_value_digit) >>
        (ident)
    )
);

named!(pub as_alias<CompleteByteSlice, String>,
    do_parse!(
        multispace >>
        opt!(do_parse!(tag_no_case!("as") >> multispace >> ())) >>
        opt!(alt!(tag!("\"") | tag!("'"))) >>
        alias: sql_alphanumeric >>
        opt!(alt!(tag!("\"") | tag!("'"))) >>
        (String::from_utf8(alias.to_vec()).unwrap())
    )
);

fn is_sql_alphanumeric(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == '_' as u8
}

fn is_value_digit(chr: u8) -> bool {
    is_digit(chr) || chr == '.' as u8
    || chr == '+' as u8
    || chr == '-' as u8
    || chr == '*' as u8
    || chr == '/' as u8

}