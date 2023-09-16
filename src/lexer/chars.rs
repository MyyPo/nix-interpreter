use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum CharType {
    Char,
    Equals,
    Plus,
    Minus,
    Comma,
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenSquare,
    CloseSquare,
    Dot,
    At,
    Digit,
    Whitespace,
    Newline,
    Dquote,
    Squote,
    Exclamation,
    Tilde,
    ForwSlash,
    Langle,
    Rangle,
    Dollar,
    Question,
    Asterisk,
    Hash,
}

impl TryFrom<char> for CharType {
    type Error = InvalidTokenError;

    fn try_from(input_char: char) -> Result<Self, Self::Error> {
        match input_char {
            WHITESPACE => Ok(CharType::Whitespace),
            NEWLINE => Ok(CharType::Newline),
            SEMICOLON => Ok(CharType::Semicolon),
            EQUALS => Ok(CharType::Equals),
            PLUS => Ok(CharType::Plus),
            MINUS => Ok(CharType::Minus),
            DOT => Ok(CharType::Dot),
            DQUOTE => Ok(CharType::Dquote),
            SQUOTE => Ok(CharType::Squote),
            OPEN_PAREN => Ok(CharType::OpenParen),
            CLOSE_PAREN => Ok(CharType::CloseParen),
            OPEN_BRACE => Ok(CharType::OpenBrace),
            CLOSE_BRACE => Ok(CharType::CloseBrace),
            OPEN_SQUARE => Ok(CharType::OpenSquare),
            CLOSE_SQUARE => Ok(CharType::CloseSquare),
            LANGLE => Ok(CharType::Langle),
            RANGLE => Ok(CharType::Rangle),
            COMMA => Ok(CharType::Comma),
            AT => Ok(CharType::At),
            EXCLAMATION => Ok(CharType::Exclamation),
            TILDE => Ok(CharType::Tilde),
            FORW_SLASH => Ok(CharType::ForwSlash),
            DOLLAR => Ok(CharType::Dollar),
            QUESTION => Ok(CharType::Question),
            ASTERISK => Ok(CharType::Asterisk),
            HASH => Ok(CharType::Hash),
            _ => {
                if let Some(char_token) = get_char(input_char) {
                    return Ok(char_token);
                }
                if let Some(digit_token) = get_digit(input_char) {
                    return Ok(digit_token);
                }
                Err(InvalidTokenError::new(input_char))
            }
        }
    }
}

impl CharType {
    pub fn is_dquote(ch: char) -> bool {
        ch == DQUOTE
    }
    pub fn is_squote(ch: char) -> bool {
        ch == SQUOTE
    }
}

fn get_char(ch: char) -> Option<CharType> {
    if !ch.is_ascii_lowercase() && !ch.is_ascii_uppercase() && ch != '-' && ch != '.' && ch != '_' {
        return None;
    }
    Some(CharType::Char)
}

fn get_digit(ch: char) -> Option<CharType> {
    if !ch.is_ascii_digit() {
        return None;
    }
    Some(CharType::Digit)
}

const EQUALS: char = '=';
const PLUS: char = '+';
const MINUS: char = '-';
const OPEN_PAREN: char = '(';
const CLOSE_PAREN: char = ')';
const OPEN_BRACE: char = '{';
const CLOSE_BRACE: char = '}';
const OPEN_SQUARE: char = '[';
const CLOSE_SQUARE: char = ']';
const COMMA: char = ',';
const SEMICOLON: char = ';';
const DOT: char = '.';
const AT: char = '@';
const EXCLAMATION: char = '!';
const DQUOTE: char = '"';
const SQUOTE: char = '\'';
const WHITESPACE: char = ' ';
const NEWLINE: char = '\n';
const FORW_SLASH: char = '/';
const TILDE: char = '~';
const LANGLE: char = '<';
const RANGLE: char = '>';
const DOLLAR: char = '$';
const QUESTION: char = '?';
const ASTERISK: char = '*';
const HASH: char = '#';

#[derive(Debug, PartialEq)]
pub struct InvalidTokenError {
    cause: char,
}
impl InvalidTokenError {
    pub fn new(cause: char) -> InvalidTokenError {
        InvalidTokenError { cause }
    }
}
impl Display for InvalidTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid symbol provided in input: {}", self.cause)
    }
}
