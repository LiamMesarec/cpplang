use crate::parser::range;
use crate::parser::body;
use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

/// for *range::declaration* in *range::expression* body
pub fn for_(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info);

    node.children.push(range::declaration(parser_info)?);

    if !parser_info.match_token(Token::In) {
        return Err(Error::InvalidFor(
            parser_info.current_token_info.clone(),
            String::from("Expected 'in'"),
        ));
    }

   node 
        .children
        .push(Node::new_box(&parser_info.current_token_info));

    node.children.push(range::expression(parser_info)?);

    node.children.push(body::body(parser_info)?);

    return Ok(node);
}
