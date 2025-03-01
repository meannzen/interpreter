use std::borrow::Cow;

use miette::{Context, Error, LabeledSpan};

pub struct Token<'a> {
    origin: &'a str,
    kind: TokenKind,
}

pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Bang,
    Equal,
    BangEqual,
    EqualEqual,
    LessEqual,
    GraterEqual,
    Less,
    Grater,
    Slash,
    String,
    Number(f64),
    Ident,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = self.origin;
        match self.kind {
            TokenKind::LeftParen => write!(f, "LEFT_PAREN {i} null"),
            TokenKind::RightParen => write!(f, "RIGHT_PAREN {i} null"),
            TokenKind::LeftBrace => write!(f, "LEFT_BRACE {i} null"),
            TokenKind::RightBrace => write!(f, "RIGHT_BRACE {i} null"),
            TokenKind::Comma => write!(f, "COMMA {i} null"),
            TokenKind::Dot => write!(f, "DOT {i} null"),
            TokenKind::Minus => write!(f, "MINUS {i} null"),
            TokenKind::Plus => write!(f, "PLUS {i} null"),
            TokenKind::Semicolon => write!(f, "SEMICOLON {i} null"),
            TokenKind::Star => write!(f, "STAR {i} null"),
            TokenKind::Bang => write!(f, "BANG {i} null"),
            TokenKind::Equal => write!(f, "EQUAL {i} null"),
            TokenKind::BangEqual => write!(f, "BANG_EQUAL {i} null"),
            TokenKind::EqualEqual => write!(f, "EQUAL_EQUAL {i} null"),
            TokenKind::LessEqual => write!(f, "LESS_EQUAL {i} null"),
            TokenKind::GraterEqual => write!(f, "GREATER_EQUAL {i} null"),
            TokenKind::Slash => write!(f, "SLASH {i} null"),
            TokenKind::Less => write!(f, "LESS {i} null"),
            TokenKind::Grater => write!(f, "GREATER {i} null"),
            TokenKind::String => write!(f, "STRING \"{}\" ", TokenKind::unescape(i)),
            TokenKind::Number(n) => write!(f, "Number {i} {n}"),
            TokenKind::Ident => write!(f, "IDENTIFIER {i} null"),
            TokenKind::And => write!(f, "AND {i} null"),
            TokenKind::Class => write!(f, "ClASS {i} null"),
            TokenKind::Else => write!(f, "ELSE {i} null"),
            TokenKind::False => write!(f, "FALSE {i} null"),
            TokenKind::For => write!(f, "FOR {i} null"),
            TokenKind::Fun => write!(f, "FUN {i} null"),
            TokenKind::If => write!(f, "IF {i} null"),
            TokenKind::Nil => write!(f, "NIL {i} null"),
            TokenKind::Or => write!(f, "OR {i} null"),
            TokenKind::Return => write!(f, "RETURN {i} null"),
            TokenKind::Super => write!(f, "SUPER {i} null"),
            TokenKind::This => write!(f, "THIS {i} null"),
            TokenKind::True => write!(f, "TRUE {i} null"),
            TokenKind::Var => write!(f, "VAR {i} null"),
            TokenKind::While => write!(f, "WHILE {i} null"),
            TokenKind::Eof => write!(f, "EOF null"),
        }
    }
}

impl TokenKind {
    pub fn unescape<'a>(_s: &'a str) -> Cow<&'a str> {
        todo!()
    }
}

pub struct Lexer<'a> {
    rest: &'a str,
    whole: &'a str,
    byte: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            rest: input,
            whole: input,
            byte: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut chars = self.rest.chars();
            let c = chars.next()?;
            let c_str = &self.rest[..c.len_utf8()];
            let c_onwards = self.rest;
            self.rest = chars.as_str();
            self.byte += c.len_utf8();

            enum Started {
                String,
                Number,
                Ident,
                IfEqualElse(TokenKind, TokenKind),
            }

            let just = move |kind: TokenKind| {
                Some(Ok(Token {
                    kind,
                    origin: c_str,
                }))
            };

            let started =
                match c {
                    '(' => return just(TokenKind::LeftParen),
                    ')' => return just(TokenKind::RightParen),
                    '{' => return just(TokenKind::LeftBrace),
                    '}' => return just(TokenKind::RightBrace),
                    '.' => return just(TokenKind::Dot),
                    ',' => return just(TokenKind::Comma),
                    '-' => return just(TokenKind::Minus),
                    '+' => return just(TokenKind::Plus),
                    ';' => return just(TokenKind::Semicolon),
                    '*' => return just(TokenKind::Star),
                    '/' => return just(TokenKind::Slash),
                    '<' => Started::IfEqualElse(TokenKind::LessEqual, TokenKind::Less),
                    '>' => Started::IfEqualElse(TokenKind::GraterEqual, TokenKind::GraterEqual),
                    '!' => Started::IfEqualElse(TokenKind::BangEqual, TokenKind::Bang),
                    '=' => Started::IfEqualElse(TokenKind::EqualEqual, TokenKind::Equal),
                    '"' => Started::String,
                    '0'..='9' => Started::Number,
                    'a'..='z' | 'A'..='Z' | '_' => Started::Ident,
                    c if c.is_whitespace() => continue,
                    c => return Some(Err(miette::miette! {
                        labels = vec![
                            LabeledSpan::at(self.byte..self.byte + c.len_utf8(), "this character"),
                        ],
                        "Unexpected token '{c}' in input"
                    }
                    .with_source_code(self.whole.to_string()))),
                };

            break match started {
                Started::String => todo!(),
                Started::Number => {
                    let first_non_digit = c_onwards
                        .find(|c| !matches!(c, '.' | '0'..='9'))
                        .unwrap_or_else(|| c_onwards.len());
                    let mut literal = &c_onwards[..first_non_digit];
                    let mut dotted = literal.splitn(3, '.');
                    match (dotted.next(), dotted.next(), dotted.next()) {
                        (Some(one), Some(two), Some(_)) => {
                            literal = &literal[..one.len() + 1 + two.len()];
                        }
                        (Some(one), Some(two), None) if two.is_empty() => {
                            literal = &literal[..one.len()];
                        }
                        _ => {
                        }
                    }
                    let extra_bytes = literal.len() - c.len_utf8();
                    self.byte += extra_bytes;
                    self.rest = &self.rest[extra_bytes..];
                    let n = match literal.parse() {
                        Ok(n) => n,
                        Err(e) => {
                            return Some(Err(miette::miette! {
                                labels = vec![
                                    LabeledSpan::at(self.byte - literal.len()..self.byte, "this numeric literal"),
                                ],
                                "{e}",
                            }.with_source_code(self.whole.to_string())));
                        }
                    };
                    return Some(Ok(Token {
                        origin: literal,
                        kind: TokenKind::Number(n),
                    }));
                }
                Started::Ident => {
                    let first_non_ident = c_onwards
                        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
                        .unwrap_or_else(|| c_onwards.len());
                    let literal = &c_onwards[..first_non_ident];
                    let extra_bytes = literal.len() - c.len_utf8();
                    self.byte += extra_bytes;
                    self.rest = &self.rest[extra_bytes..];
                    let kind = match literal {
                        "and" => TokenKind::And,
                        "class" => TokenKind::Class,
                        "else" => TokenKind::Else,
                        "false" => TokenKind::False,
                        "for" => TokenKind::For,
                        "fun" => TokenKind::Fun,
                        "if" => TokenKind::If,
                        "nil" => TokenKind::Nil,
                        "or" => TokenKind::Or,
                        "return" => TokenKind::Return,
                        "super" => TokenKind::Super,
                        "this" => TokenKind::This,
                        "true" => TokenKind::True,
                        "var" => TokenKind::Var,
                        "while" => TokenKind::While,
                        _ => TokenKind::Ident,
                    };
                    return Some(Ok(Token {
                        origin: literal,
                        kind,
                   }));
                }
                Started::IfEqualElse(yes, no) => {
                    self.rest = self.rest.trim_start();
                    let trimmed = c_onwards.len() - self.rest.len() - 1;
                    self.byte += trimmed;
                    if self.rest.trim_start().starts_with("_") {
                        let span = &c_onwards[..c.len_utf8() + trimmed + 1];
                        self.rest = &self.rest[1..];
                        self.byte += 1;
                        Some(Ok(Token {
                            origin: span,
                            kind: yes,
                        }))
                    } else {
                        Some(Ok(Token {
                            origin: c_str,
                            kind: no,
                        }))
                    }
                }
            };
        }
    }
}
