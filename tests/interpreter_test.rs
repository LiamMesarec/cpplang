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
    assert!(evaluate_and_compare(
        r#"fn main(): u32 { 0 }"#,
        "0"
    ));
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
#[ignore]
fn for_() {
    assert!(evaluate_and_compare(
        r#"
        for i in 0..5 {
        }
        
"#,
        "15"
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
        
        v = 10

        std::println("noj", i, "koj", v)
        
        return v

        
        
"#,
        "10"
    ));
}
