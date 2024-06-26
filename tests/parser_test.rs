use rust::parser;
use rust::tokenizer;
use std::io::Cursor;

fn is_parsable(input: &str) -> bool {
    match tokenizer::tokenize(Cursor::new(input)) {
        Ok(tokens) => match parser::parse(tokens) {
            Some(_) => return true,
            None => return false,
        },
        Err(error) => {
            println!("{}", error);
            false
        }
    }
}

#[test]
fn assignment() {
    assert!(is_parsable(
        r#"
let i: u32 = u
let a: UserDefined = 11+(22 + -10 / (10 * (50))*2)"#
    ));
    assert!(is_parsable(
        r#"
let i = 10"#
    ));
    assert!(is_parsable(
        r#"
	let mut i = 10"#
    ));
}

#[test]
#[ignore]
fn functions() {
    assert!(is_parsable(
        r#"
    fn main(): u32 {
        0
    }
"#
    ));

    assert!(is_parsable(
        r#"
    fn func(i: i32): u32 {
        0
    }
"#
    ));
}

#[test]
fn for_() {
    assert!(is_parsable(
        r#"
    for i in array {
        i
    }
"#
    ));

    assert!(is_parsable(
        r#"
        for i in 0..9 {
            i
        }
    "#
    ));
}

#[test]
fn if_() {
    assert!(is_parsable(
        r#"
    if i == 10 {
        20
    }
"#
    ));

    assert!(is_parsable(
        r#"
    if i == 12 {
        20
    } else {
        i == 10
    }
"#
    ));
}

#[test]
fn array() {
    assert!(is_parsable(
        r#"
    a[i] = 1
"#
    ));

    assert!(is_parsable(
        r#"
    a = [1*(10-2),2,3]
    let i = 1
    return a[i]
"#
    ));
}

#[test]
#[ignore]
fn match_() {
    assert!(is_parsable(
        r#"
    match i {
    	1 => 10
    }
"#
    ));
    assert!(is_parsable(
        r#"
    match i {
    	i => 1 == 3
    	i => 2 == 3
    }
"#
    ));
}

#[test]
#[ignore]
fn struct_() {
    assert!(is_parsable(
        r#"
    struct Struct {
        age: u32
        name: String
    }
"#
    ));
    assert!(is_parsable(
        r#"
    struct Struct {
    }
"#
    ));
}
