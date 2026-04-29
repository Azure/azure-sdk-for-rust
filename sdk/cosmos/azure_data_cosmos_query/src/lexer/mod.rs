// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Lexer (tokenizer) for Cosmos DB SQL.
//!
//! Hand-crafted scanner that operates on UTF-8 `&str` input, producing tokens
//! with zero-copy text slices where possible.

use std::fmt;

/// A single token produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub span: Span,
}

/// Byte offset span in the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Token types produced by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    Identifier,
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    Parameter,

    // Keywords
    Select,
    From,
    Where,
    And,
    Or,
    Not,
    As,
    In,
    Between,
    Like,
    Escape,
    Order,
    By,
    Asc,
    Desc,
    Top,
    Distinct,
    Value,
    Group,
    Having,
    Join,
    Cross,
    Inner,
    Exists,
    Array,
    Null,
    True,
    False,
    Undefined,
    Offset,
    Limit,
    Udf,
    Is,
    Let,
    Left,
    Right,
    Set,
    Over,
    Rank,
    For,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Tilde,
    Ampersand,
    Pipe,
    Caret,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    LeftShift,
    RightShift,
    ZeroFillRightShift,
    StringConcat,
    Coalesce,
    Question,
    Colon,
    Bang,

    // Punctuation
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Dot,
    Comma,

    // Special
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Identifier => "identifier",
            Self::StringLiteral => "string",
            Self::IntegerLiteral => "integer",
            Self::FloatLiteral => "float",
            Self::Parameter => "parameter",
            Self::Select => "SELECT",
            Self::From => "FROM",
            Self::Where => "WHERE",
            Self::And => "AND",
            Self::Or => "OR",
            Self::Not => "NOT",
            Self::As => "AS",
            Self::In => "IN",
            Self::Between => "BETWEEN",
            Self::Like => "LIKE",
            Self::Escape => "ESCAPE",
            Self::Order => "ORDER",
            Self::By => "BY",
            Self::Asc => "ASC",
            Self::Desc => "DESC",
            Self::Top => "TOP",
            Self::Distinct => "DISTINCT",
            Self::Value => "VALUE",
            Self::Group => "GROUP",
            Self::Having => "HAVING",
            Self::Join => "JOIN",
            Self::Cross => "CROSS",
            Self::Inner => "INNER",
            Self::Exists => "EXISTS",
            Self::Array => "ARRAY",
            Self::Null => "null",
            Self::True => "true",
            Self::False => "false",
            Self::Undefined => "undefined",
            Self::Offset => "OFFSET",
            Self::Limit => "LIMIT",
            Self::Udf => "udf",
            Self::Is => "IS",
            Self::Let => "LET",
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::Set => "SET",
            Self::Over => "OVER",
            Self::Rank => "RANK",
            Self::For => "FOR",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Percent => "%",
            Self::Tilde => "~",
            Self::Ampersand => "&",
            Self::Pipe => "|",
            Self::Caret => "^",
            Self::Eq => "=",
            Self::NotEq => "!=",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::LtEq => "<=",
            Self::GtEq => ">=",
            Self::LeftShift => "<<",
            Self::RightShift => ">>",
            Self::ZeroFillRightShift => ">>>",
            Self::StringConcat => "||",
            Self::Coalesce => "??",
            Self::Question => "?",
            Self::Colon => ":",
            Self::Bang => "!",
            Self::LParen => "(",
            Self::RParen => ")",
            Self::LBracket => "[",
            Self::RBracket => "]",
            Self::LBrace => "{",
            Self::RBrace => "}",
            Self::Dot => ".",
            Self::Comma => ",",
            Self::Eof => "EOF",
        };
        write!(f, "{s}")
    }
}

/// The lexer that produces tokens from SQL source text.
pub struct Lexer<'a> {
    source: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given SQL source text.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            pos: 0,
        }
    }

    /// Produce the next token. Returns `Eof` when the input is exhausted.
    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace_and_comments();

        if self.pos >= self.bytes.len() {
            return Token {
                kind: TokenKind::Eof,
                text: "",
                span: Span {
                    start: self.pos,
                    end: self.pos,
                },
            };
        }

        let start = self.pos;
        let ch = self.bytes[self.pos];

        match ch {
            // String literal (single-quoted)
            b'\'' => self.scan_string_literal(start),

            // Double-quoted identifier
            b'"' => self.scan_quoted_identifier(start),

            // Parameter
            b'@' => self.scan_parameter(start),

            // Numbers
            b'0'..=b'9' => self.scan_number(start),

            // Identifiers and keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.scan_identifier(start),

            // Two/three-character operators and single-character tokens
            b'(' => self.single_char_token(start, TokenKind::LParen),
            b')' => self.single_char_token(start, TokenKind::RParen),
            b'[' => self.single_char_token(start, TokenKind::LBracket),
            b']' => self.single_char_token(start, TokenKind::RBracket),
            b'{' => self.single_char_token(start, TokenKind::LBrace),
            b'}' => self.single_char_token(start, TokenKind::RBrace),
            b'.' => self.single_char_token(start, TokenKind::Dot),
            b',' => self.single_char_token(start, TokenKind::Comma),
            b'+' => self.single_char_token(start, TokenKind::Plus),
            b'-' => self.single_char_token(start, TokenKind::Minus),
            b'*' => self.single_char_token(start, TokenKind::Star),
            b'/' => self.single_char_token(start, TokenKind::Slash),
            b'%' => self.single_char_token(start, TokenKind::Percent),
            b'~' => self.single_char_token(start, TokenKind::Tilde),
            b'^' => self.single_char_token(start, TokenKind::Caret),
            b'=' => self.single_char_token(start, TokenKind::Eq),
            b':' => self.single_char_token(start, TokenKind::Colon),

            b'!' => {
                self.pos += 1;
                if self.peek() == Some(b'=') {
                    self.pos += 1;
                    self.make_token(start, TokenKind::NotEq)
                } else {
                    self.make_token(start, TokenKind::Bang)
                }
            }

            b'<' => {
                self.pos += 1;
                match self.peek() {
                    Some(b'=') => {
                        self.pos += 1;
                        self.make_token(start, TokenKind::LtEq)
                    }
                    Some(b'<') => {
                        self.pos += 1;
                        self.make_token(start, TokenKind::LeftShift)
                    }
                    Some(b'>') => {
                        self.pos += 1;
                        self.make_token(start, TokenKind::NotEq)
                    }
                    _ => self.make_token(start, TokenKind::Lt),
                }
            }

            b'>' => {
                self.pos += 1;
                match self.peek() {
                    Some(b'=') => {
                        self.pos += 1;
                        self.make_token(start, TokenKind::GtEq)
                    }
                    Some(b'>') => {
                        self.pos += 1;
                        if self.peek() == Some(b'>') {
                            self.pos += 1;
                            self.make_token(start, TokenKind::ZeroFillRightShift)
                        } else {
                            self.make_token(start, TokenKind::RightShift)
                        }
                    }
                    _ => self.make_token(start, TokenKind::Gt),
                }
            }

            b'&' => {
                self.pos += 1;
                if self.peek() == Some(b'&') {
                    self.pos += 1;
                    self.make_token(start, TokenKind::And)
                } else {
                    self.make_token(start, TokenKind::Ampersand)
                }
            }

            b'|' => {
                self.pos += 1;
                match self.peek() {
                    Some(b'|') => {
                        self.pos += 1;
                        self.make_token(start, TokenKind::StringConcat)
                    }
                    _ => self.make_token(start, TokenKind::Pipe),
                }
            }

            b'?' => {
                self.pos += 1;
                if self.peek() == Some(b'?') {
                    self.pos += 1;
                    self.make_token(start, TokenKind::Coalesce)
                } else {
                    self.make_token(start, TokenKind::Question)
                }
            }

            _ => {
                // Unknown character — advance past it and return an identifier token
                // so the parser can produce a proper error.
                self.pos += 1;
                self.make_token(start, TokenKind::Identifier)
            }
        }
    }

    /// Tokenize the entire input into a vector of tokens (excluding EOF).
    pub fn tokenize(source: &'a str) -> Vec<Token<'a>> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok.kind == TokenKind::Eof {
                break;
            }
            tokens.push(tok);
        }
        tokens
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }

            // Skip line comments: -- ...
            if self.pos + 1 < self.bytes.len()
                && self.bytes[self.pos] == b'-'
                && self.bytes[self.pos + 1] == b'-'
            {
                self.pos += 2;
                while self.pos < self.bytes.len() && self.bytes[self.pos] != b'\n' {
                    self.pos += 1;
                }
                continue;
            }

            // Skip block comments: /* ... */
            if self.pos + 1 < self.bytes.len()
                && self.bytes[self.pos] == b'/'
                && self.bytes[self.pos + 1] == b'*'
            {
                self.pos += 2;
                while self.pos + 1 < self.bytes.len()
                    && !(self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/')
                {
                    self.pos += 1;
                }
                if self.pos + 1 < self.bytes.len() {
                    self.pos += 2; // skip */
                }
                continue;
            }

            break;
        }
    }

    fn scan_string_literal(&mut self, start: usize) -> Token<'a> {
        self.pos += 1; // skip opening quote
        while self.pos < self.bytes.len() {
            if self.bytes[self.pos] == b'\'' {
                // Check for escaped quote ('')
                if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'\'' {
                    self.pos += 2;
                } else {
                    self.pos += 1; // skip closing quote
                    return self.make_token(start, TokenKind::StringLiteral);
                }
            } else {
                self.pos += 1;
            }
        }
        // Unterminated string — return what we have
        self.make_token(start, TokenKind::StringLiteral)
    }

    fn scan_quoted_identifier(&mut self, start: usize) -> Token<'a> {
        self.pos += 1; // skip opening "
        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'"' {
            self.pos += 1;
        }
        if self.pos < self.bytes.len() {
            self.pos += 1; // skip closing "
        }
        self.make_token(start, TokenKind::Identifier)
    }

    fn scan_parameter(&mut self, start: usize) -> Token<'a> {
        self.pos += 1; // skip @
        while self.pos < self.bytes.len() && is_ident_char(self.bytes[self.pos]) {
            self.pos += 1;
        }
        self.make_token(start, TokenKind::Parameter)
    }

    fn scan_number(&mut self, start: usize) -> Token<'a> {
        let mut is_float = false;
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        // Decimal point
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'.' {
            // Make sure it's not a member access on a number (e.g., "1.toString()")
            // by checking the next char is a digit
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1].is_ascii_digit() {
                is_float = true;
                self.pos += 1; // skip .
                while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
            }
        }
        // Exponent
        if self.pos < self.bytes.len()
            && (self.bytes[self.pos] == b'e' || self.bytes[self.pos] == b'E')
        {
            is_float = true;
            self.pos += 1;
            if self.pos < self.bytes.len()
                && (self.bytes[self.pos] == b'+' || self.bytes[self.pos] == b'-')
            {
                self.pos += 1;
            }
            while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }
        if is_float {
            self.make_token(start, TokenKind::FloatLiteral)
        } else {
            self.make_token(start, TokenKind::IntegerLiteral)
        }
    }

    fn scan_identifier(&mut self, start: usize) -> Token<'a> {
        while self.pos < self.bytes.len() && is_ident_char(self.bytes[self.pos]) {
            self.pos += 1;
        }
        let text = &self.source[start..self.pos];
        let kind = keyword_lookup(text);
        Token {
            kind,
            text,
            span: Span {
                start,
                end: self.pos,
            },
        }
    }

    fn single_char_token(&mut self, start: usize, kind: TokenKind) -> Token<'a> {
        self.pos += 1;
        self.make_token(start, kind)
    }

    fn make_token(&self, start: usize, kind: TokenKind) -> Token<'a> {
        Token {
            kind,
            text: &self.source[start..self.pos],
            span: Span {
                start,
                end: self.pos,
            },
        }
    }
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Case-insensitive keyword lookup. Returns `Identifier` if not a keyword.
fn keyword_lookup(text: &str) -> TokenKind {
    // Match case-insensitively
    match text.to_ascii_uppercase().as_str() {
        "SELECT" => TokenKind::Select,
        "FROM" => TokenKind::From,
        "WHERE" => TokenKind::Where,
        "AND" => TokenKind::And,
        "OR" => TokenKind::Or,
        "NOT" => TokenKind::Not,
        "AS" => TokenKind::As,
        "IN" => TokenKind::In,
        "BETWEEN" => TokenKind::Between,
        "LIKE" => TokenKind::Like,
        "ESCAPE" => TokenKind::Escape,
        "ORDER" => TokenKind::Order,
        "BY" => TokenKind::By,
        "ASC" => TokenKind::Asc,
        "DESC" => TokenKind::Desc,
        "TOP" => TokenKind::Top,
        "DISTINCT" => TokenKind::Distinct,
        "VALUE" => TokenKind::Value,
        "GROUP" => TokenKind::Group,
        "HAVING" => TokenKind::Having,
        "JOIN" => TokenKind::Join,
        "CROSS" => TokenKind::Cross,
        "INNER" => TokenKind::Inner,
        "EXISTS" => TokenKind::Exists,
        "ARRAY" => TokenKind::Array,
        "NULL" => TokenKind::Null,
        "TRUE" => TokenKind::True,
        "FALSE" => TokenKind::False,
        "UNDEFINED" => TokenKind::Undefined,
        "OFFSET" => TokenKind::Offset,
        "LIMIT" => TokenKind::Limit,
        "UDF" => TokenKind::Udf,
        "IS" => TokenKind::Is,
        "LET" => TokenKind::Let,
        "LEFT" => TokenKind::Left,
        "RIGHT" => TokenKind::Right,
        "SET" => TokenKind::Set,
        "OVER" => TokenKind::Over,
        "RANK" => TokenKind::Rank,
        "FOR" => TokenKind::For,
        _ => TokenKind::Identifier,
    }
}

/// Extract the string content from a string literal token text (strip quotes, unescape).
pub fn extract_string_content(token_text: &str) -> String {
    // Remove surrounding quotes
    let inner = if token_text.len() >= 2 {
        &token_text[1..token_text.len() - 1]
    } else {
        token_text
    };
    // Unescape doubled quotes
    inner.replace("''", "'")
}

/// Extract the identifier name from a possibly-quoted identifier token text.
pub fn extract_identifier(token_text: &str) -> &str {
    if token_text.starts_with('"') && token_text.ends_with('"') && token_text.len() >= 2 {
        &token_text[1..token_text.len() - 1]
    } else {
        token_text
    }
}

/// Extract the parameter name from a parameter token text (strip the @).
pub fn extract_parameter_name(token_text: &str) -> &str {
    if let Some(stripped) = token_text.strip_prefix('@') {
        stripped
    } else {
        token_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_select() {
        let tokens = Lexer::tokenize("SELECT * FROM c");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Select);
        assert_eq!(tokens[1].kind, TokenKind::Star);
        assert_eq!(tokens[2].kind, TokenKind::From);
        assert_eq!(tokens[3].kind, TokenKind::Identifier);
        assert_eq!(tokens[3].text, "c");
    }

    #[test]
    fn string_literal() {
        let tokens = Lexer::tokenize("'hello world'");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(extract_string_content(tokens[0].text), "hello world");
    }

    #[test]
    fn escaped_string() {
        let tokens = Lexer::tokenize("'it''s'");
        assert_eq!(tokens.len(), 1);
        assert_eq!(extract_string_content(tokens[0].text), "it's");
    }

    #[test]
    fn numbers() {
        let tokens = Lexer::tokenize("42 3.14 1e10 2.5E-3");
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[1].kind, TokenKind::FloatLiteral);
        assert_eq!(tokens[2].kind, TokenKind::FloatLiteral);
        assert_eq!(tokens[3].kind, TokenKind::FloatLiteral);
    }

    #[test]
    fn parameters() {
        let tokens = Lexer::tokenize("@p1 @customer_id");
        assert_eq!(tokens[0].kind, TokenKind::Parameter);
        assert_eq!(extract_parameter_name(tokens[0].text), "p1");
        assert_eq!(tokens[1].kind, TokenKind::Parameter);
        assert_eq!(extract_parameter_name(tokens[1].text), "customer_id");
    }

    #[test]
    fn operators() {
        let tokens = Lexer::tokenize("!= <= >= << >> >>> || ??");
        assert_eq!(tokens[0].kind, TokenKind::NotEq);
        assert_eq!(tokens[1].kind, TokenKind::LtEq);
        assert_eq!(tokens[2].kind, TokenKind::GtEq);
        assert_eq!(tokens[3].kind, TokenKind::LeftShift);
        assert_eq!(tokens[4].kind, TokenKind::RightShift);
        assert_eq!(tokens[5].kind, TokenKind::ZeroFillRightShift);
        assert_eq!(tokens[6].kind, TokenKind::StringConcat);
        assert_eq!(tokens[7].kind, TokenKind::Coalesce);
    }

    #[test]
    fn keywords_case_insensitive() {
        let tokens = Lexer::tokenize("select FROM Where");
        assert_eq!(tokens[0].kind, TokenKind::Select);
        assert_eq!(tokens[1].kind, TokenKind::From);
        assert_eq!(tokens[2].kind, TokenKind::Where);
    }

    #[test]
    fn line_comment() {
        let tokens = Lexer::tokenize("SELECT -- this is a comment\n* FROM c");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Select);
        assert_eq!(tokens[1].kind, TokenKind::Star);
    }

    #[test]
    fn block_comment() {
        let tokens = Lexer::tokenize("SELECT /* comment */ * FROM c");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Select);
        assert_eq!(tokens[1].kind, TokenKind::Star);
    }

    #[test]
    fn full_query_tokenization() {
        let tokens = Lexer::tokenize(
            "SELECT c.name, c.age FROM c WHERE c.pk = 'hello' AND c.age > 21 ORDER BY c.age DESC",
        );
        assert!(tokens.len() > 10);
        assert_eq!(tokens[0].kind, TokenKind::Select);
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Desc);
    }
}
