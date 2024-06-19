use rust::evaluator;
use rust::parser;
use rust::tokenizer;
use std::io::Cursor;

fn evaluate_and_compare(input: &str, expected_output: &str) -> bool {
    match tokenizer::tokenize(Cursor::new(input)) {
        Ok(tokens) => match parser::parse(tokens) {
            Some(ast) => {
                let output = evaluator::interpret(&ast);
                println!("{:?}", ast.statements);
                if output != expected_output {
                    println!("output: {}", output);
                    return false;
                }

                return true;
            }
            None => return false,
        },
        Err(error) => {
            println!("{}", error);
            return false;
        }
    }
}

#[test]
#[ignore]
fn assignment() {
    assert!(evaluate_and_compare(
        r#"let i: u32 = u"#,
        "const uint_32t i = u;"
    ));
}

#[test]
#[ignore]
fn function() {
    assert!(evaluate_and_compare(r#"fn main(): u32 { 0 }"#, "0"));
}

#[test]
#[ignore]
fn fun_() {
    assert!(evaluate_and_compare(
        r#"
        fn test(i : i64): i64 {
        let f = i + 10
        return f
        }
        
        fn main(): i64 {
        let i = 1
        let v = 2
        if ( 5 < 10 ) {
        let a = i + 10
        }
        
        int n = test(i)
        std::println("noj", i, "koj", v)
        
        return n
}
        
        
"#,
        "0"
    ));
}
#[test]
fn arr_() {
    assert!(evaluate_and_compare(
        r#"
let mut arr: Array<i32> = [5, 6, 7, 3, 10]
        let i = 1
        return arr[i]
"#,
        "6"
    ));
}
#[test]
fn for_() {
    assert!(evaluate_and_compare(
        r#"
        let i = 0
        let m = 0
        for i in 1..5 {
        	m = i + m
        	std::println("i in loop ", i)
        }
        return m
"#,
        "10"
    ));
}


#[test]
fn sort_() {
    assert!(evaluate_and_compare(
        r#"
let mut arr: Array<i32> = [5, 6, 7, 3, 10]
let n = 5
let i = 0
let j = 0

for i in 0..n-1 {
    let tmp = n - 1
    for j in 0..tmp-i {
        let left = arr[j]
        let right = arr[j + 1]
        if left > right {
            arr[j] = right
            arr[j + 1] = left
        }
    }
}
let n = 5
for i in 0..n {
    std::println(i, " " ,arr[i])
}
return arr
"#,
        "[3, 5, 6, 7, 10]"
    ));
}


#[test]
fn if_() {
    assert!(evaluate_and_compare(
        r#"
        let i = 1
        let v = 2
        if ( 5 < 10 ) {
        let a = i + 10
        }
        
        v = 10 + i

        std::println("noj", i, "koj", v)
        
        return v

        
        
"#,
        "11"
    ));
}
