use crate::tokenizer::{Token, TokenInfo};
use ptree::{Style, TreeItem};
use std::borrow::Cow;
use std::io::Write;

mod body;
mod for_;
mod function;
mod if_;
mod match_;
mod range;
mod struct_;

#[derive(Debug)]
pub enum Error {
    Generic(TokenInfo, String),
    InvalidFor(TokenInfo, String),
    InvalidAssignment(TokenInfo, String),
    MissingClosingBrackets(TokenInfo),
    MissingClosingParantheses(TokenInfo),
    ExpectedStartingBrackets(TokenInfo),
    ExpectedStartingParantheses(TokenInfo),
    MissingType(TokenInfo, String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(token_info, string) => write!(
                f,
                "Syntax error: unexpected token '{}' after {} on line {}",
                token_info.lexeme, string, token_info.start_position.row
            ),
            Error::InvalidFor(token_info, string) => write!(
                f,
                "Syntax error: invalid for loop structure, unexpected token '{}' on line {}. {}",
                token_info.lexeme, token_info.start_position.row, string
            ),
            Error::InvalidAssignment(token_info, string) => write!(
                f,
                "Syntax error: invalid assignment; found '{}' after {} on line {}",
                token_info.lexeme, string, token_info.start_position.row
            ),
            Error::MissingClosingBrackets(token_info) => write!(
                f,
                "Syntax error: missing closing brackets on line {}",
                token_info.start_position.row
            ),
            Error::MissingClosingParantheses(token_info) => write!(
                f,
                "Syntax error: missing closing parantheses on line {}",
                token_info.start_position.row
            ),
            Error::ExpectedStartingBrackets(token_info) => write!(
                f,
                "Syntax error: expected {{, found '{}' on line {}",
                token_info.lexeme, token_info.start_position.row
            ),
            Error::ExpectedStartingParantheses(token_info) => write!(
                f,
                "Syntax error: expected (, found '{}' on line {}",
                token_info.lexeme, token_info.start_position.row
            ),
            Error::MissingType(token_info, string) => write!(
                f,
                "Syntax error: expected ': Typename' after {}, found '{}' on line {}",
                token_info.lexeme, string, token_info.start_position.row
            ),
        }
    }
}

struct ParserInfo<'slice> {
    tokens: &'slice [TokenInfo],
    current_token_info: TokenInfo,
    i: usize,
}

impl ParserInfo<'_> {
    fn match_token(&mut self, expected_token: Token) -> bool {
        self.current_token_info = self.tokens[self.i].clone();
        if self.tokens[self.i].token == expected_token {
            self.i += 1;
            return true;
        }

        false
    }

    fn match_any_of(&mut self, expected_tokens: &[Token]) -> bool {
        self.current_token_info = self.tokens[self.i].clone();
        if let Some(token) = self.tokens.get(self.i) {
            if expected_tokens
                .iter()
                .any(|&expected_token| token.token == expected_token)
            {
                self.i += 1;
                return true;
            }
        }
        false
    }

    fn last_n_token_lexemes(&self, n: usize) -> String {
        let mut counter = 1;
        let mut string: String = String::from("");

        if self.i == 0 {
            return String::from("");
        }

        while n > 0 {
            string = format!("{} {}", &string, self.tokens[self.i - counter].lexeme);
            counter += 1;

            if self.i - counter == 0 {
                break;
            }
        }

        string.chars().rev().collect::<String>()
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub token_info: TokenInfo,
    pub children: Vec<Box<Node>>,
}

impl TreeItem for Node {
    type Child = Self;
    fn write_self<W: Write>(&self, f: &mut W, _style: &Style) -> std::io::Result<()> {
        write!(
            f,
            "{}, ({:?})",
            self.token_info.lexeme, self.token_info.token
        )
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::Owned(self.children.iter().map(|node| *node.clone()).collect())
    }
}

impl Node {
    pub fn new_box(token_info: &TokenInfo) -> Box<Node> {
        Box::new(Node {
            token_info: token_info.clone(),
            children: vec![],
        })
    }

    pub fn new_empty_box() -> Box<Node> {
        Box::new(Node {
            token_info: TokenInfo::default(),
            children: vec![],
        })
    }
}

pub type ParseResult = Result<Box<Node>, Error>;

pub fn parse(tokens: &[TokenInfo]) -> ParseResult {
    let mut parser_info = ParserInfo {
        tokens,
        current_token_info: TokenInfo::default(),
        i: 0,
    };

    let mut root = Node::new_empty_box();

    while !parser_info.match_token(Token::EOF) {
        root.children.push(operator(&mut parser_info)?);
    }

    ptree::print_tree(&*root.clone()).unwrap();
    Ok(root)
}

pub fn operator(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = primary(parser_info)?;
    while parser_info.match_token(Token::Equals) {
        node.children
            .push(Node::new_box(&parser_info.current_token_info));
        node.children.push(primary(parser_info)?);
    }

    Ok(node)
}

fn assignment(parser_info: &mut ParserInfo, mut parent: Box<Node>) -> ParseResult {
    if parser_info.match_token(Token::Identifier) {
        parent
            .children
            .push(Node::new_box(&parser_info.current_token_info));

        if parser_info.match_token(Token::Colon) {
            parent
                .children
                .push(Node::new_box(&parser_info.current_token_info));

            if parser_info.match_token(Token::Identifier) {
                parent
                    .children
                    .push(Node::new_box(&parser_info.current_token_info));
            }

            if parser_info.match_token(Token::Assignment) {
                parent
                    .children
                    .push(Node::new_box(&parser_info.current_token_info));
                parent.children.push(operator(parser_info)?);

                return Ok(parent);
            }
        }
    }

    Err(Error::InvalidAssignment(
        parser_info.current_token_info.clone(),
        parser_info.last_n_token_lexemes(3),
    ))
}

fn parameter_list(parser_info: &mut ParserInfo, mut parent: Box<Node>) -> ParseResult {
    while parser_info.match_token(Token::Identifier) {
        parent
            .children
            .push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Colon) {
            return Err(Error::MissingType(
                parser_info.current_token_info.clone(),
                parser_info.last_n_token_lexemes(3),
            ));
        }

        parent
            .children
            .push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Identifier) {
            return Err(Error::MissingType(
                parser_info.current_token_info.clone(),
                parser_info.last_n_token_lexemes(3),
            ));
        }

        parent
            .children
            .push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Comma) {
            break;
        }

        parent
            .children
            .push(Node::new_box(&parser_info.current_token_info));
    }

    Ok(parent)
}

fn primary(parser_info: &mut ParserInfo) -> ParseResult {
    if parser_info.match_token(Token::Let) {
        return assignment(parser_info, Node::new_box(&parser_info.current_token_info));
    } else if parser_info.match_token(Token::Fn) {
        return function::function(parser_info);
    } else if parser_info.match_token(Token::For) {
        return for_::for_(parser_info);
    } else if parser_info.match_token(Token::If) {
        return if_::if_(parser_info);
    } else if parser_info.match_token(Token::Match) {
        return match_::match_(parser_info);
    } else if parser_info.match_token(Token::Struct) {
        return struct_::struct_(parser_info);
    } else if parser_info.match_token(Token::LeftParantheses) {
        let mut node = Node::new_box(&parser_info.current_token_info);
        node.children.push(operator(parser_info)?);
        if !parser_info.match_token(Token::RightParantheses) {
            return Err(Error::MissingClosingParantheses(
                parser_info.current_token_info.clone(),
            ));
        }

        node.children
            .push(Node::new_box(&parser_info.current_token_info));

        Ok(node)
    } else if parser_info.match_token(Token::LeftBraces) {
        return body::body(parser_info);
    } else if parser_info.match_token(Token::Identifier) || parser_info.match_token(Token::Number) {
        Ok(Node::new_box(&parser_info.current_token_info))
    } else {
        Err(Error::Generic(
            parser_info.current_token_info.clone(),
            parser_info.last_n_token_lexemes(3),
        ))
    }
}
