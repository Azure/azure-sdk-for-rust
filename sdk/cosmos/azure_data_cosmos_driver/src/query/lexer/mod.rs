// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore kinded

//! Lexer (tokenizer) for Cosmos DB SQL.
//!
//! Hand-crafted scanner that operates on UTF-8 `&str` input, producing tokens
//! with zero-copy text slices where possible.

use std::fmt;

// (#17) Length-bucketed keyword lookup lives in a sibling file.
mod keywords;
use keywords::keyword_lookup;

/// A single token produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) text: &'a str,
    pub(crate) span: Span,
}

/// Byte offset span in the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

/// Token types produced by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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

    /// (#6) Lexer error: a single-quoted string ran past EOF without a closing
    /// quote. The parser converts this into a `ParseError` instead of silently
    /// consuming the partial token as a normal `StringLiteral`.
    ErrUnterminatedString,

    /// lexer error — a double-quoted identifier ran past EOF without a
    /// closing quote. Same diagnostic principle as `ErrUnterminatedString`.
    ErrUnterminatedQuotedIdentifier,

    /// lexer error — a `/* ... */` block comment ran past EOF without a
    /// closing `*/`. Surfacing this as a token (rather than silently swallowing
    /// the rest of the input) means the parser fails with a precise diagnostic
    /// rather than producing a confusing "unexpected EOF" later.
    ErrUnterminatedBlockComment,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Identifier => "identifier",
            Self::StringLiteral => "string",
            Self::IntegerLiteral => "integer",
            Self::FloatLiteral => "float",
            Self::Parameter => "parameter",
            Self::ErrUnterminatedString => "unterminated string literal",
            Self::ErrUnterminatedQuotedIdentifier => "unterminated quoted identifier",
            Self::ErrUnterminatedBlockComment => "unterminated block comment",
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
    /// when `skip_whitespace_and_comments` runs into an unterminated
    /// `/* ... */` block, it stashes the start offset here so that the next
    /// `next_token` call emits a single `ErrUnterminatedBlockComment` token
    /// instead of silently swallowing the rest of the input.
    pending_block_comment_error: Option<usize>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given SQL source text.
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            pos: 0,
            pending_block_comment_error: None,
        }
    }

    /// Produce the next token. Returns `Eof` when the input is exhausted.
    pub(crate) fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace_and_comments();

        // if `skip_whitespace_and_comments` ran into an unterminated
        // block comment, surface it as a single error token before any
        // further work — the partial comment otherwise silently swallows the
        // remainder of the input.
        if let Some(err_start) = self.pending_block_comment_error.take() {
            return Token {
                kind: TokenKind::ErrUnterminatedBlockComment,
                text: &self.source[err_start..self.pos],
                span: Span {
                    start: err_start,
                    end: self.pos,
                },
            };
        }

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
                // respect UTF-8 character boundaries. The previous
                // single-byte advance turned a multi-byte char like `é`
                // (U+00E9, two bytes) into two single-byte `Identifier`
                // tokens, producing a wildly wrong AST. Walk forward to
                // the next char boundary so the error token spans exactly
                // one Unicode scalar value, which the parser can report
                // cleanly.
                let mut next_pos = self.pos + 1;
                while next_pos < self.bytes.len() && !self.source.is_char_boundary(next_pos) {
                    next_pos += 1;
                }
                self.pos = next_pos;
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
                let comment_start = self.pos;
                self.pos += 2;
                while self.pos + 1 < self.bytes.len()
                    && !(self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/')
                {
                    self.pos += 1;
                }
                if self.pos + 1 < self.bytes.len() {
                    self.pos += 2; // skip */
                } else {
                    // unterminated block comment — record the start
                    // offset and advance to EOF; `next_token` will emit a
                    // single `ErrUnterminatedBlockComment` token before
                    // returning `Eof`.
                    self.pos = self.bytes.len();
                    self.pending_block_comment_error = Some(comment_start);
                    return;
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
        // (#6) Unterminated string — surface as an error token so the parser
        // can fail with a precise diagnostic rather than silently consuming a
        // malformed `StringLiteral`.
        self.make_token(start, TokenKind::ErrUnterminatedString)
    }

    fn scan_quoted_identifier(&mut self, start: usize) -> Token<'a> {
        self.pos += 1; // skip opening "
        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'"' {
            self.pos += 1;
        }
        if self.pos < self.bytes.len() {
            self.pos += 1; // skip closing "
            self.make_token(start, TokenKind::Identifier)
        } else {
            // unterminated `"...` — surface as an error token so the
            // parser fails with a precise diagnostic instead of silently
            // consuming the partial identifier.
            self.make_token(start, TokenKind::ErrUnterminatedQuotedIdentifier)
        }
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

// (#17) Length-bucketed keyword lookup lives in the sibling keywords module.

/// Extract the string content from a string literal token text (strip quotes, unescape).
///
/// The lexer routes unterminated strings to [`TokenKind::ErrUnterminatedString`]
/// before this helper is reached, so the input is always a properly-quoted
/// `'...'` string literal.
pub(crate) fn extract_string_content(token_text: &str) -> String {
    let inner = if token_text.len() >= 2
        && token_text.starts_with(char::from(b'\''))
        && token_text.ends_with(char::from(b'\''))
    {
        &token_text[1..token_text.len() - 1]
    } else {
        token_text
    };
    // Unescape doubled quotes
    inner.replace("''", "'")
}

/// Extract the identifier name from a possibly-quoted identifier token text.
pub(crate) fn extract_identifier(token_text: &str) -> &str {
    if token_text.starts_with('"') && token_text.ends_with('"') && token_text.len() >= 2 {
        &token_text[1..token_text.len() - 1]
    } else {
        token_text
    }
}

/// Extract the parameter name from a parameter token text (strip the @).
pub(crate) fn extract_parameter_name(token_text: &str) -> &str {
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

    /// (#6) An unterminated string literal must surface as a distinct error
    /// token \u2014 not as a normal `StringLiteral` whose content swallows the rest
    /// of the input \u2014 so the parser can report a precise diagnostic instead of
    /// silently consuming a malformed token.
    #[test]
    fn unterminated_string_yields_error_token() {
        let tokens = Lexer::tokenize("'unclosed");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::ErrUnterminatedString);
    }

    /// Same situation but with trailing content that was previously absorbed
    /// into the malformed string literal.
    #[test]
    fn unterminated_string_with_trailing_input_yields_error_token() {
        let tokens = Lexer::tokenize("SELECT 'unclosed FROM c");
        // `SELECT` keyword followed by the error token \u2014 trailing characters
        // are part of the (un)quoted text but the kind is the error variant.
        assert_eq!(tokens.first().map(|t| t.kind), Some(TokenKind::Select));
        assert!(
            tokens
                .iter()
                .any(|t| t.kind == TokenKind::ErrUnterminatedString),
            "expected an ErrUnterminatedString token; got {:?}",
            tokens.iter().map(|t| t.kind).collect::<Vec<_>>()
        );
    }
    /// same diagnostic shape for unterminated `"...` quoted identifier.
    #[test]
    fn unterminated_quoted_identifier_yields_error_token() {
        let tokens = Lexer::tokenize("SELECT \"unclosed FROM c");
        assert_eq!(tokens.first().map(|t| t.kind), Some(TokenKind::Select));
        assert!(
            tokens
                .iter()
                .any(|t| t.kind == TokenKind::ErrUnterminatedQuotedIdentifier),
            "expected ErrUnterminatedQuotedIdentifier; got {:?}",
            tokens.iter().map(|t| t.kind).collect::<Vec<_>>()
        );
    }

    /// unterminated `/* ... */` block comment must surface as an error
    /// token rather than silently swallowing the rest of the input.
    #[test]
    fn unterminated_block_comment_yields_error_token() {
        let tokens = Lexer::tokenize("SELECT /* unclosed");
        assert_eq!(tokens.first().map(|t| t.kind), Some(TokenKind::Select));
        assert!(
            tokens
                .iter()
                .any(|t| t.kind == TokenKind::ErrUnterminatedBlockComment),
            "expected ErrUnterminatedBlockComment; got {:?}",
            tokens.iter().map(|t| t.kind).collect::<Vec<_>>()
        );
    }

    /// a non-ASCII character must produce a single error token whose
    /// span covers the full UTF-8 char (one Unicode scalar value), not a
    /// sequence of single-byte tokens straddling the char boundary.
    #[test]
    fn non_ascii_character_respects_char_boundary() {
        let tokens = Lexer::tokenize("\u{00e9}"); // 'é', 2 UTF-8 bytes
                                                  // The lexer routes unknown chars to a single-byte `Identifier`-kinded
                                                  // error token (the parser then produces a clean diagnostic). The
                                                  // important property F13 enforces is that the token spans the full
                                                  // 2-byte char — not 1 byte that splits the UTF-8 sequence.
        assert_eq!(tokens.len(), 1, "expected one token, got {:?}", tokens);
        assert_eq!(tokens[0].text.len(), 2);
        assert_eq!(tokens[0].text, "\u{00e9}");
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
