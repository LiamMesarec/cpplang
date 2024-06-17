use crate::tokenizer::Token;
use crate::tokenizer::TokenInfo;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub library: String,
}

pub fn init_types() -> Result<HashMap<String, TypeInfo>, Box<dyn Error>> {
    let mut hashmap = HashMap::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("types.csv")?;

    for result in rdr.records() {
        let record = result?;
        let key = record[0].to_string();
        let type_info = TypeInfo {
            name: record[1].to_string(),
            library: record[2].to_string(),
        };
        hashmap.insert(key, type_info);
    }

    Ok(hashmap)
}

pub fn translate_type(
    token_info: &TokenInfo,
    types: &HashMap<String, TypeInfo>,
) -> Option<TypeInfo> {
    if token_info.token != Token::Identifier {
        return None;
    }

    return match types.get(&token_info.lexeme) {
        Some(cpp_type) => Some(cpp_type.clone()),
        None => None,
    };
}

pub fn init_std_names() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut hashmap = HashMap::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("functions.csv")?;

    for result in rdr.records() {
        let record = result?;
        let key = record[0].to_string();
        let library = record[1].to_string();
        hashmap.insert(key, library);
    }

    Ok(hashmap)
}

pub fn get_library(name: &str, std_names: &HashMap<String, String>) -> Option<String> {
    return match std_names.get(name) {
        Some(library) => Some(library.clone()),
        None => None,
    };
}
