use rust::evaluator;
use rust::parser;
use rust::tokenizer;
use std::io::Cursor;

fn evaluate_and_compare(input: &str, expected_output: &str) -> bool {
    match tokenizer::tokenize(Cursor::new(input)) {
        Ok(tokens) => match parser::parse(tokens) {
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
fn assignment() {
    assert!(evaluate_and_compare(
        r#"let i: u32 = u"#,
        "const uint32_t i = u;"
    ));
}

#[test]
fn function() {
    assert!(evaluate_and_compare(
        r#"fn main(): u32 { return 0 }"#,
        "uint32_t main() { return 0; }"
    ));
}

#[test]
fn if_() {
    assert!(evaluate_and_compare(
        r#"
    if i < 10 {
        i = 20
    } else {
        i = i + 20
    }
"#,
        "if ( i < 10 ) { i = 20; } else { i = i + 20; }"
    ));
}
