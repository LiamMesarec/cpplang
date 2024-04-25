use crate::parser::range;
use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

/// for *range::declaration* in *range::expression* body
pub fn for_(parser_info: &mut ParserInfo, mut parent: Box<Node>) -> ParseResult {
    parent = range::declaration(parser_info, parent)?;

    if !parser_info.match_token(Token::In) {
        return Err(Error::InvalidFor(
            parser_info.current_token_info.clone(),
            String::from("Expected 'in'"),
        ));
    }

    parent
        .children
        .push(Node::new_box(&parser_info.current_token_info));

    return Ok(range::expression(parser_info, parent)?);
}
