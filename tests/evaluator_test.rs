use rust::evaluator;
use rust::parser;
use rust::parser::parser::parse;
use rust::tokenizer;
use std::io::Cursor;

fn evaluate_and_compare(input: &str, expected_output: &str) -> bool {
    match tokenizer::tokenize(Cursor::new(input)) {
        Ok(tokens) => match parse(tokens) {
            Some(ast) => {
                let output = evaluator::evaluate(&ast);
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
        "uint32_t main() { 0 }"
    ));
}

#[test]
fn if_() {
    assert!(evaluate_and_compare(
        r#"
    if i < 10 {
        20
    } else {
        i
    }
"#,
        "if ( i < 10 ) { 20 } else { i }"
    ));
}
