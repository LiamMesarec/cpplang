/*use rust::evaluator;
use rust::parser;
use rust::tokenizer;
use std::io::Cursor;

fn evaluate_and_compare(input: &str, expected_output: &str) -> bool {
    match tokenizer::tokenize(Cursor::new(input)) {
        Ok(tokens) => match parser::parse(&tokens) {
            Ok(ast) => match evaluator::evaluate(ast) {
                Ok(output) => {
                    if output != expected_output {
                        println!("output: {}", output);
                        return false;
                    }

                    return true;
                }
                Err(error) => {
                    println!("{}", error);
                    return false;
                }
            },
            Err(error) => {
                println!("{}", error);
                return false;
            }
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
fn function() {
    assert!(evaluate_and_compare(
        r#"fn main(): u32 { 0 }"#,
        "uint32_t main() { 0 }"
    ));
}*/
