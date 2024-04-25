use crate::parser::operator;
use crate::parser::range;
use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

pub fn body(parser_info: &mut ParserInfo, mut parent: Box<Node>) -> ParseResult {
    parent.children.push(operator(parser_info)?);
    if !parser_info.match_token(Token::RightBraces) {
        return Err(Error::MissingClosingParantheses(
            parser_info.current_token_info.clone(),
        ));
    }

    parent
        .children
        .push(Node::new_box(&parser_info.current_token_info));

    Ok(parent)
}
