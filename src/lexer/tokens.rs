use crate::lexer::chars::CharType;

#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    Ident(&'a str),
    StrLiteral(&'a str),
    Path(&'a str),
    NixPath(&'a str),
    Int(i32),
    Flo(f32),
    LogicalComparison(LogicalComparison),
    ArithmComparison(ArithmComparison),
    AdditiveOperator(AdditiveOperator),
    MultiplicativeOperator(MultiplicativeOperator),
    Bool(bool),
    LogicalNegation,
    ArithmNegation,
    Assign,
    Semicolon,
    Let,
    In,
    Inherit,
    Import,
    With,
    And,
    Or,
    Map,
    Null,
    CloseBrace,
    OpenBrace,
    CloseSquare,
    OpenSquare,
    OpenParen,
    CloseParen,
    Access,
    If,
    Then,
    Else,
    Rec,
    LogImpl,
    Update,
    Concat,
    Has,
}

#[derive(Debug, PartialEq)]
pub enum LogicalComparison {
    CompareEquals,
    CompareNotEquals,
}

#[derive(Debug, PartialEq)]
pub enum ArithmComparison {
    More,
    Less,
    MoreOrEquals,
    LessOrEquals,
}

#[derive(Debug, PartialEq)]
pub enum AdditiveOperator {
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub enum MultiplicativeOperator {
    Mult,
    Div,
}

impl<'a> From<CharType> for TokenType<'a> {
    fn from(input_char_type: CharType) -> Self {
        match input_char_type {
            CharType::Equals => Self::Assign,
            CharType::Semicolon => Self::Semicolon,
            CharType::Asterisk => Self::MultiplicativeOperator(MultiplicativeOperator::Mult),
            CharType::OpenBrace => Self::OpenBrace,
            CharType::CloseBrace => Self::CloseBrace,
            CharType::OpenSquare => Self::OpenSquare,
            CharType::CloseSquare => Self::CloseSquare,
            CharType::Question => Self::Has,
            CharType::OpenParen => Self::OpenParen,
            CharType::CloseParen => Self::CloseParen,
            _ => panic!("unhandled simple type provided: {:?}", input_char_type),
        }
    }
}

impl<'a> TokenType<'a> {
    pub fn ident_or_keyword_from(chars: &'a str) -> Self {
        match chars {
            LET => Self::Let,
            IN => Self::In,
            IMPORT => Self::Import,
            WITH => Self::With,
            INHERIT => Self::Inherit,
            MAP => Self::Map,
            NULL => Self::Null,
            TRUE | FALSE => Self::bool_from(chars),
            IF => Self::If,
            THEN => Self::Then,
            ELSE => Self::Else,
            LOGICAL_AND => Self::And,
            LOGICAL_OR => Self::Or,
            ARROW => Self::LogImpl,
            REC => Self::Rec,
            MORE_OR_EQUALS => Self::ArithmComparison(ArithmComparison::MoreOrEquals),
            LESS_OR_EQUALS => Self::ArithmComparison(ArithmComparison::LessOrEquals),
            UPDATE => Self::Update,
            _ => Self::Ident(chars),
        }
    }

    pub fn num_from(chars: &str) -> Self {
        if let Ok(parsed_int) = chars.parse::<i32>() {
            return TokenType::Int(parsed_int);
        }
        if let Ok(parsed_flo) = chars.parse::<f32>() {
            return TokenType::Flo(parsed_flo);
        }
        panic!("failed to parse a number chars: ${chars} to token type");
    }
    pub fn bool_from(chars: &str) -> Self {
        if let Ok(b) = chars.parse::<bool>() {
            return TokenType::Bool(b);
        }
        unimplemented!()
    }
}

const LET: &str = "let";
const IN: &str = "in";
const INHERIT: &str = "inherit";
const IMPORT: &str = "import";
const WITH: &str = "with";
const MAP: &str = "map";
const NULL: &str = "null";
const RELATIVE_PATH: &str = "./";
const ABSOLUTE_PATH: &str = "/";
const HOME_PATH: &str = "~";
const NIX_PATH: &str = "<";
const IF: &str = "if";
const THEN: &str = "then";
const ELSE: &str = "else";
const LOGICAL_AND: &str = "&&";
const LOGICAL_OR: &str = "||";
const ARROW: &str = "->";
const REC: &str = "rec";
const MORE_OR_EQUALS: &str = ">=";
const LESS_OR_EQUALS: &str = "<=";
const UPDATE: &str = "//";
const TRUE: &str = "true";
const FALSE: &str = "false";
