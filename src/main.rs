#[macro_use]
extern crate nom;

mod nom1;
mod commons;

use std::str;

fn main() {
//    let b = &[0x123u8, 0x10u8, 0x32u8, 0x32u8, 0x32u8, 0x32u8, 0x32u8, 0x32u8, 0x34u8, 0x97u8, 0x34u8, 0x58u8, 0x32u8, 0x52u8, 0x50u8, 0x44u8, 0x10u8, 0x32u8, 0x32u8, 0x32u8, 0x32u8, 0x32u8, 0x32u8, 0x34u8, 0x98u8, 0x34u8, 0x58u8, 0x32u8, 0x34u8, 0x120u8, 0x92u8, 0x125u8, 0x34u8, 0x10u8, 0x32u8, 0x32u8, 0x32u8, 0x32u8, 0x125u8, 0x92u8, 0x48u8];
//
//    let s = match str::from_utf8(b) {
//        Ok(v) => v,
//        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
//    };
//
//    println!("result: {}", s);
    let buf = &[0x26u8, 0x66u8, 0x76u8, 0x73u8];

    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("result: {}", s);

    let s = match String::from("aaa").to_uppercase().as_ref() {
        "AAA" => true,
        _ => false
    };
    println!("{}", s);
}
