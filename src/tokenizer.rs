use std::io::BufRead;

#[derive(Debug)]
pub enum Error {
    NotAKeyword(String),
    InvalidPattern(String, Position),
    InvalidStream,
    UnclosedString(Position),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotAKeyword(lexeme) => write!(f, "Tokenizer error: not a keyword {}", lexeme),
            Error::InvalidPattern(lexeme, position) => write!(
                f,
                "Tokenizer error: invalid pattern {} on line {}",
                lexeme, position.row
            ),
            Error::InvalidStream => write!(f, "Tokenizer error: invalid stream. Cannot read"),
            Error::UnclosedString(position) => write!(
                f,
                "Tokenizer error: Unclosed string on line {}",
                position.row
            ),
        }
    }
}

// TODO if there is too many tokens the tokenizer stops working and shows next_token as EOF
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Token {
    #[default]
    None = 0,

    Identifier,
    Colon,
    Comma,
    Number,
    Star,
    Modulo,
    String,

    Let,
    Mut,
    Interpret,
    Fn,
    Return,
    Match,
    Struct,
    Arrow,

    Multiplication,
    Division,
    Addition,
    Subtraction,
    Int,
    LeftParantheses,
    RightParantheses,
    LeftBraces,
    RightBraces,
    Assignment,
    GreaterThan,
    LowerThan,
    Equals,
    Semicolon,
    If,
    Else,
    For,
    While,
    In,
    Dot,
    Range,
    Ignore,
    EOT,

    EOF,
    Error,
}

const MAX_STATE: usize = Token::Error as usize;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Position {
    pub row: u32,
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TokenInfo {
    pub token: Token,
    pub lexeme: String,
    pub start_position: Position,
}

struct DFA {
    num_states: usize,
    alphabet: [char; 256],
    last: char,
    position: Position,
}

pub fn tokenize<R: BufRead>(mut tokens_reader: R) -> Result<Vec<TokenInfo>, Error> {
    let mut dfa = DFA {
        num_states: MAX_STATE,
        alphabet: [char::default(); 256],
        last: char::default(),
        position: Position { row: 1, col: 1 },
    };

    let mut vec = Vec::new();

    for i in 0..=255 {
        dfa.alphabet[i] = char::from_u32(i as u32).unwrap();
    }

    match get_token(&mut tokens_reader, &mut dfa) {
        Ok(mut token_info) => {
            while token_info.token != Token::EOF {
                if token_info.token != Token::None {
                    vec.push(token_info);
                }

                token_info = match get_token(&mut tokens_reader, &mut dfa) {
                    Ok(token_info) => token_info,
                    Err(error) => return Err(error),
                }
            }
        }

        Err(error) => return Err(error),
    };

    vec.push(TokenInfo {
        token: Token::EOF,
        lexeme: String::from(""),
        start_position: dfa.position,
    });

    Ok(vec)
}

fn get_token<R: BufRead>(mut tokens_reader: R, dfa: &mut DFA) -> Result<TokenInfo, Error> {
    let transitions_table = create_transitions_table(dfa.alphabet.len(), dfa.num_states);
    let mut buffer = [0; 1];
    let mut token_info = TokenInfo {
        token: Token::None,
        lexeme: String::from(""),
        start_position: dfa.position,
    };

    let mut state = Token::None;
    let mut code: char;

    if dfa.last != char::default() {
        code = dfa.last;
        dfa.last = char::default();
        if code != '\n' && code != ' ' && code != '\t' {
            token_info.start_position = prev_position(dfa.position, code);
        }
    } else {
        if tokens_reader.read(&mut buffer).unwrap() > 0 {
            code = buffer[0] as char;
            dfa.position = next_position(dfa.position, code);
        } else {
            token_info.token = Token::EOF;
            return Ok(token_info);
        }
    }

    /*if code == '\"' {
        while code != '\"' {
            if tokens_reader.read(&mut buffer).unwrap() > 0 {
                code = buffer[0] as char;
                dfa.position = next_position(dfa.position, code);
            } else {
                token_info.token = Token::EOF;
                return Ok(token_info);
            }
        }

        token_info.lexeme.push(code);
        dfa.position = next_position(dfa.position, code);
        token_info.lexeme.push(code);
    }*/

    loop {
        let next_state = transitions_table[state as usize][code as usize];
        if next_state == Token::EOT || next_state == Token::EOF {
            break;
        }

        if state == Token::None && next_state == Token::None && code != char::default() {
            token_info.lexeme.push(code);
            return Err(Error::InvalidPattern(
                token_info.lexeme,
                token_info.start_position,
            ));
        }

        if next_state == Token::None {
            if state == Token::String {
                token_info.lexeme.push(code);
                token_info.token = state;
                if tokens_reader.read(&mut buffer).unwrap() > 0 {
                    code = buffer[0] as char;
                    dfa.last = code;
                    dfa.position = next_position(dfa.position, code);
                } else {
                    if !token_info.lexeme.is_empty() {
                        dfa.last = char::default();
                        break;
                    }
                    token_info.token = Token::EOF;
                    return Ok(token_info);
                }
            }
            break;
        }

        state = next_state;
        token_info.lexeme.push(code);

        if tokens_reader.read(&mut buffer).unwrap() > 0 {
            code = buffer[0] as char;
            dfa.last = code;
            dfa.position = next_position(dfa.position, code);
        } else {
            if !token_info.lexeme.is_empty() {
                dfa.last = char::default();
                break;
            }
            token_info.token = Token::EOF;
            return Ok(token_info);
        }
    }

    token_info.token = state;
    token_info.token = assign_if_reserved_identifier(&token_info);

    if token_info.token == Token::String && token_info.lexeme.chars().last().unwrap() != '\"' {
        return Err(Error::UnclosedString(token_info.start_position));
    }

    Ok(token_info)
}

fn assign_if_reserved_identifier(token_info: &TokenInfo) -> Token {
    match token_info.lexeme.as_ref() {
        "for" => Token::For,
        "while" => Token::While,
        "in" => Token::In,
        "let" => Token::Let,
        "mut" => Token::Mut,
        "fn" => Token::Fn,
        "return" => Token::Return,
        "if" => Token::If,
        "else" => Token::Else,
        "interpret" => Token::Interpret,
        "match" => Token::Match,
        "struct" => Token::Struct,
        _ => token_info.token,
    }
}

fn create_transitions_table(alphabet_len: usize, num_states: usize) -> Vec<Vec<Token>> {
    let mut transitions_table: Vec<Vec<Token>> = vec![vec![Token::None; alphabet_len]; num_states];

    let mut set_transition = |from: Token, c: char, to: Token| {
        transitions_table[from as usize][c as usize] = to;
    };

    let mut set_full_transition = |s: &str, to: Token| {
        let mut chars = s.chars();

        if let Some(first) = chars.next() {
            set_transition(Token::None, first, to);
        }

        for c in chars {
            set_transition(to, c, to);
        }
    };

    let _set_full_transitions = |transitions: &[(&str, Token)]| {
        for &(s, to) in transitions {
            set_full_transition(s, to);
        }
    };

    set_transition(Token::None, '=', Token::Assignment);
    set_transition(Token::Assignment, '>', Token::Arrow);
    set_transition(Token::None, '*', Token::Star);
    set_transition(Token::None, '%', Token::Modulo);
    set_transition(Token::None, '/', Token::Division);
    set_transition(Token::None, '+', Token::Addition);
    set_transition(Token::None, '-', Token::Subtraction);
    set_transition(Token::None, ',', Token::Comma);

    set_transition(Token::None, ':', Token::Colon);
    set_transition(Token::Assignment, '=', Token::Equals);

    set_transition(Token::None, '\"', Token::String);
    //set_transition(Token::String::string_start, '\"', Token::String::string_end);

    //TODO add to stop on " and not on \"
    for i in 0..=255 {
        let c = char::from_u32(i as u32).unwrap();
        if c != '\"' {
            set_transition(Token::String, c, Token::String)
        } else {
            set_transition(Token::String, c, Token::None)
        }
    }

    for i in '0'..='9' {
        set_transition(Token::None, i, Token::Number);
        set_transition(Token::Number, i, Token::Number);
        set_transition(Token::Identifier, i, Token::Identifier);
    }

    for i in 'a'..='z' {
        set_transition(Token::None, i, Token::Identifier);
        set_transition(Token::Identifier, i, Token::Identifier);
    }

    for i in 'A'..='Z' {
        set_transition(Token::None, i, Token::Identifier);
        set_transition(Token::Identifier, i, Token::Identifier);
    }

    set_transition(Token::Identifier, '_', Token::Identifier);
    set_transition(Token::None, '_', Token::Identifier);

    set_transition(Token::None, ' ', Token::EOT);
    set_transition(Token::None, '\t', Token::EOT);
    set_transition(Token::None, '\n', Token::EOT);
    set_transition(Token::EOT, ' ', Token::EOT);
    set_transition(Token::EOT, '\t', Token::EOT);
    set_transition(Token::EOT, '\n', Token::EOT);

    set_transition(Token::None, '(', Token::LeftParantheses);
    set_transition(Token::None, ')', Token::RightParantheses);
    set_transition(Token::None, '{', Token::LeftBraces);
    set_transition(Token::None, '}', Token::RightBraces);

    set_transition(Token::None, '.', Token::Dot);
    set_transition(Token::Dot, '.', Token::Range);

    set_transition(Token::None, Token::EOF as u8 as char, Token::EOF);
    transitions_table
}

fn next_position(position: Position, code: char) -> Position {
    let mut pos = position;
    if code == '\n' {
        pos.row += 1;
        pos.col = 1;
        return pos;
    }

    if code == '\t' {
        pos.col += 4;
        return pos;
    }

    pos.col += 1;

    pos
}

fn prev_position(position: Position, prev_code: char) -> Position {
    let mut pos = position;
    if prev_code == '\n' {
        pos.row -= 1;
        pos.col = 1;
        return pos;
    }

    if prev_code == '\t' {
        pos.col -= 4;
        return pos;
    }

    pos.col -= 1;

    pos
}
