use crate::lexer::chars::*;
use crate::lexer::tokens::*;
use std::iter::Iterator;

pub trait Tokenizer<'a> {
    fn tokenize(&'a mut self) -> &TokenStream;
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input_str: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    tokens: Vec<TokenType<'a>>,

    capture_start: usize,
    capture_end: usize,

    closing_delimiter_found: bool,
}
impl<'a> Lexer<'a> {
    pub fn new(input_str: &'a str) -> Self {
        Self {
            input_str,
            chars: input_str.char_indices().peekable(),
            tokens: Vec::with_capacity(input_str.len() / 5),

            capture_start: 0,
            capture_end: 0,

            closing_delimiter_found: false,
        }
    }

    fn lex_path(&mut self, curr_idx: usize) {
        self.capture_start = curr_idx;
        self.chars.next();
        for (i, ch) in self.chars.by_ref() {
            if let Ok(CharType::Whitespace | CharType::Newline) = CharType::try_from(ch) {
                break;
            }
            self.capture_end = i;
        }
        let path = &self.input_str[self.capture_start..=self.capture_end];
        self.tokens.push(TokenType::Path(path));
    }
}
impl<'a> Tokenizer<'a> for Lexer<'a> {
    fn tokenize(&'a mut self) -> &TokenStream {
        while let Some((i, ch)) = self.chars.next() {
            match CharType::try_from(ch) {
                Ok(ch) => match ch {
                    CharType::Char => {
                        (self.capture_start, self.capture_end) = (i, i);
                        while let Some(&(i, ch)) = self.chars.peek() {
                            if let Ok(CharType::Char | CharType::Digit | CharType::Minus) =
                                CharType::try_from(ch)
                            {
                                self.chars.next();
                                self.capture_end = i;
                            } else {
                                break;
                            };
                        }
                        self.tokens.push(TokenType::ident_or_keyword_from(
                            &self.input_str[self.capture_start..=self.capture_end],
                        ));
                    }
                    CharType::Plus => {
                        if let Some((_, next_ch)) = self.chars.peek() {
                            match CharType::try_from(*next_ch) {
                                Ok(CharType::Plus) => {
                                    self.chars.next();
                                    self.tokens.push(TokenType::Concat)
                                }
                                _ => self
                                    .tokens
                                    .push(TokenType::AdditiveOperator(AdditiveOperator::Add)),
                            }
                        } else {
                            panic!("Unexpected EOF, after + symbol");
                        }
                    }
                    CharType::Minus => {
                        if let Some(t) = self.tokens.last() {
                            match t {
                                TokenType::Ident(_) | TokenType::Flo(_) | TokenType::Int(_) => self
                                    .tokens
                                    .push(TokenType::AdditiveOperator(AdditiveOperator::Sub)),
                                _ => self.tokens.push(TokenType::ArithmNegation),
                            }
                        } else {
                            panic!("Unexpected EOF before - symbol");
                        }
                    }
                    CharType::Dquote => {
                        self.capture_start = i + 1;

                        for (i, ch) in self.chars.by_ref() {
                            if CharType::is_dquote(ch) {
                                self.tokens.push(TokenType::StrLiteral(
                                    &self.input_str[self.capture_start..i],
                                ));
                                self.closing_delimiter_found = true;
                                break;
                            }
                        }
                        if !self.closing_delimiter_found {
                            panic!("Unexpected EOF, expecting a second, closing double quote");
                        }
                        self.closing_delimiter_found = false;
                    }
                    CharType::Squote => {
                        if let Some((_, next_ch)) = self.chars.next() {
                            if !CharType::is_squote(next_ch) {
                                panic!("Expected to find a second single quote");
                            }
                        } else {
                            panic!("Unexpected EOF, expecting a second single quote");
                        }

                        self.capture_start = i + 2;
                        while let Some((i, ch)) = self.chars.next() {
                            if CharType::is_squote(ch) {
                                if let Some((_, next_ch)) = self.chars.next() {
                                    if CharType::is_squote(next_ch) {
                                        self.tokens.push(TokenType::StrLiteral(
                                            &self.input_str[self.capture_start..i],
                                        ));
                                        self.closing_delimiter_found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        if !self.closing_delimiter_found {
                            panic!("Unexpected EOF, expecting a second, closing single quote");
                        }
                        self.closing_delimiter_found = false;
                    }
                    CharType::Digit => {
                        (self.capture_start, self.capture_end) = (i, i);
                        while let Some(&(i, ch)) = self.chars.peek() {
                            if let Ok(CharType::Digit | CharType::Dot) = CharType::try_from(ch) {
                                self.chars.next();
                                self.capture_end = i;
                            } else {
                                break;
                            };
                        }
                        self.tokens.push(TokenType::num_from(
                            &self.input_str[self.capture_start..=self.capture_end],
                        ));
                    }
                    CharType::Dot => {
                        if let Some((_, next_ch)) = self.chars.peek() {
                            match CharType::try_from(*next_ch) {
                                Ok(CharType::ForwSlash) => {
                                    self.lex_path(i);
                                }
                                Err(err) => panic!("{err}"),
                                _ => {
                                    self.chars.next();
                                    self.tokens.push(TokenType::Access);
                                }
                            }
                        }
                    }
                    CharType::Tilde => {
                        if let Some((_, next_ch)) = self.chars.peek() {
                            match CharType::try_from(*next_ch) {
                                Ok(CharType::ForwSlash) => {
                                    self.lex_path(i);
                                }
                                _ => unimplemented!(),
                            }
                        }
                    }
                    CharType::ForwSlash => {
                        if let Some((_, next_ch)) = self.chars.peek() {
                            match CharType::try_from(*next_ch) {
                                Ok(CharType::Whitespace) => {
                                    self.chars.next();
                                    self.tokens.push(TokenType::MultiplicativeOperator(
                                        MultiplicativeOperator::Div,
                                    ))
                                }
                                _ => self.lex_path(i),
                            }
                        }
                    }
                    CharType::Langle => {
                        (self.capture_start, self.capture_end) = (i, i);
                        let iter_clone = self.chars.clone();

                        for (i, ch) in iter_clone {
                            match CharType::try_from(ch) {
                                Ok(
                                    CharType::Whitespace | CharType::Newline | CharType::Semicolon,
                                ) => {
                                    break;
                                }
                                Ok(CharType::Rangle) => {
                                    for _ in 0..i - self.capture_start {
                                        self.chars.next();
                                    }
                                    self.tokens.push(TokenType::NixPath(
                                        &self.input_str[self.capture_start..=i],
                                    ));
                                    self.closing_delimiter_found = true;
                                    break;
                                }
                                Err(err) => panic!("{err}"),
                                _ => continue,
                            }
                        }
                        if !self.closing_delimiter_found {
                            if let Some((_, next_ch)) = self.chars.peek() {
                                match CharType::try_from(*next_ch) {
                                    Ok(CharType::Equals) => {
                                        self.chars.next();
                                        self.tokens.push(TokenType::ArithmComparison(
                                            ArithmComparison::LessOrEquals,
                                        ));
                                    }
                                    Err(err) => panic!("{err}"),
                                    _ => self
                                        .tokens
                                        .push(TokenType::ArithmComparison(ArithmComparison::Less)),
                                }
                            }
                        }
                    }
                    CharType::Rangle => {
                        if let Some((_, next_ch)) = self.chars.peek() {
                            match CharType::try_from(*next_ch) {
                                Ok(CharType::Equals) => {
                                    self.chars.next();
                                    self.tokens.push(TokenType::ArithmComparison(
                                        ArithmComparison::MoreOrEquals,
                                    ));
                                }
                                Err(err) => panic!("{err}"),
                                _ => self
                                    .tokens
                                    .push(TokenType::ArithmComparison(ArithmComparison::More)),
                            }
                        }
                    }
                    CharType::Equals => {
                        if let Some((_, next_ch)) = self.chars.peek() {
                            match CharType::try_from(*next_ch) {
                                Ok(CharType::Equals) => {
                                    self.chars.next();
                                    self.tokens.push(TokenType::LogicalComparison(
                                        LogicalComparison::CompareEquals,
                                    ));
                                }
                                _ => self.tokens.push(TokenType::Assign),
                            }
                        }
                    }
                    CharType::Exclamation => {
                        if let Some((_, next_ch)) = self.chars.peek() {
                            match CharType::try_from(*next_ch) {
                                Ok(CharType::Equals) => {
                                    self.chars.next();
                                    self.tokens.push(TokenType::LogicalComparison(
                                        LogicalComparison::CompareNotEquals,
                                    ))
                                }
                                _ => self.tokens.push(TokenType::LogicalNegation),
                            }
                        }
                    }
                    CharType::Hash => {
                        for (_, ch) in self.chars.by_ref() {
                            match CharType::try_from(ch) {
                                Ok(CharType::Newline) => break,
                                Err(err) => panic!("{err}"),
                                _ => continue,
                            }
                        }
                    }
                    CharType::Whitespace | CharType::Newline => {
                        continue;
                    }
                    _ => {
                        self.tokens.push(TokenType::from(ch));
                    }
                },
                Err(err) => {
                    panic!("{err}")
                }
            }
        }

        self.tokens.as_slice()
    }
}

pub enum TokenStreamErrors {
    UnexpectedToken(UnexpectedTokenError),
}
#[derive(Debug)]
pub struct UnexpectedTokenError {
    msg: String,
}
impl UnexpectedTokenError {
    fn new(exp_tok: &TokenType, got_tok: &TokenType) -> Self {
        UnexpectedTokenError {
            msg: format!("Expected token: {:?}, but got: {:?}", exp_tok, got_tok),
        }
    }
}

pub type TokenStream<'a> = [TokenType<'a>];
