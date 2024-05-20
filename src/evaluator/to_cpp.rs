use std::collections::HashMap;
use std::error::Error;
use crate::tokenizer::TokenInfo;
use crate::tokenizer::Token;

#[derive(Debug)]
pub struct TypeInfo {
    name: String,
    library: String
}

pub fn init_types() -> Result<HashMap<String, TypeInfo>, Box<dyn Error>> {
    let mut hashmap = HashMap::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("types.csv")?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        let key = record[0].to_string();
        let type_info = TypeInfo {
            name: record[1].to_string(),
            library: record[2].to_string(),
        };
        hashmap.insert(key, type_info);
    }

    Ok(hashmap)
}

pub fn translate_type(token_info: &TokenInfo, types: &HashMap<String, TypeInfo>) -> Option<String> {

    if token_info.token != Token::Identifier {
        return None;
    }

    return match types.get(&token_info.lexeme) {
        Some(cpp_type) => Some(cpp_type.name.clone()),
        None => None,
    }
}
