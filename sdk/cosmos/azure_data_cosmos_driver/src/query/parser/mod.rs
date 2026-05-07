// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recursive descent parser for the Cosmos DB SQL dialect.
//!
//! Produces an [`SqlProgram`] AST from SQL text. Uses Pratt parsing
//! for operator precedence in scalar expressions.

use crate::query::ast::{
    SqlBinaryOp, SqlCollection, SqlCollectionExpression, SqlFromClause, SqlGroupByClause,
    SqlLimitSpec, SqlLiteral, SqlObjectProperty, SqlOffsetLimitClause, SqlOffsetSpec,
    SqlOrderByClause, SqlOrderByItem, SqlPathSegment, SqlProgram, SqlQuery, SqlScalarExpression,
    SqlSelectClause, SqlSelectItem, SqlSelectSpec, SqlSortOrder, SqlTopSpec, SqlUnaryOp,
    SqlWhereClause,
};
use crate::query::lexer::{
    extract_identifier, extract_parameter_name, extract_string_content, Lexer, Span, Token,
    TokenKind,
};

/// Parse error with location information.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ParseError {
    pub(crate) message: String,
    pub(crate) span: Span,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at offset {}", self.message, self.span.start)
    }
}

impl std::error::Error for ParseError {}

/// Parse a SQL string into an AST.
///
/// # Examples
/// ```ignore
/// let program = azure_data_cosmos_driver::query::parse("SELECT * FROM c WHERE c.id = '1'").unwrap();
/// // The returned SqlProgram contains the parsed AST.
/// // Use plan::generate_query_plan() or eval::matches_query() to work with it.
/// ```
pub fn parse(sql: &str) -> Result<SqlProgram, ParseError> {
    let mut parser = Parser::new(sql);
    let program = parser.parse_program()?;
    // (#6) Surface any deferred lexer error (e.g. unterminated string literal
    // that appeared after the parser had already finished consuming).
    parser.check_pending_lex_error()?;
    Ok(program)
}

// Maximum subquery / parenthesis nesting depth. Each level walks through the
// ~14-stage precedence ladder in `parse_scalar_expression`, so each nested
// level consumes roughly 14 stack frames. 32 keeps the worst-case stack
// footprint comfortably under 1 MiB even in unoptimized debug builds, so the
// guard always fires before exhausting a default 2 MiB worker / test-harness
// thread stack. Real Cosmos SQL queries virtually never exceed single-digit
// nesting; this is purely a safety ceiling for adversarial / generated input.
const MAX_NESTING_DEPTH: usize = 32;

struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token<'a>,
    nesting: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token();
        Self {
            lexer,
            current,
            nesting: 0,
        }
    }

    /// (#6) Convert any in-flight lexer error token (e.g. unterminated string)
    /// into a structured [`ParseError`]. Called from `expect`, `parse`, and
    /// other choke points so the parser cannot silently accept a malformed
    /// token as if it were a well-formed `StringLiteral`.
    fn check_pending_lex_error(&self) -> Result<(), ParseError> {
        match self.current.kind {
            TokenKind::ErrUnterminatedString => {
                Err(self.error("unterminated string literal: missing closing single quote".into()))
            }
            // same diagnostic principle as `ErrUnterminatedString`.
            TokenKind::ErrUnterminatedQuotedIdentifier => {
                Err(self
                    .error("unterminated quoted identifier: missing closing double quote".into()))
            }
            TokenKind::ErrUnterminatedBlockComment => {
                Err(self.error("unterminated block comment: missing closing `*/`".into()))
            }
            _ => Ok(()),
        }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.current.kind == kind
    }

    fn at_eof(&self) -> bool {
        self.current.kind == TokenKind::Eof
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token<'a>, ParseError> {
        // (#6) If the lexer flagged the current token as a malformed literal,
        // raise a precise diagnostic before attempting the match.
        self.check_pending_lex_error()?;
        if self.current.kind == kind {
            let tok = self.current.clone();
            self.advance();
            Ok(tok)
        } else {
            Err(self.error(format!("expected {kind}, found {}", self.current.kind)))
        }
    }

    fn consume_if(&mut self, kind: TokenKind) -> bool {
        if self.current.kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    fn error(&self, message: String) -> ParseError {
        // when the parser is about to bail out, prefer the lexer's
        // diagnostic if the current token is a lex-error variant. Otherwise
        // a downstream "expected X" message would mask the real problem
        // (the malformed token never gets to the explicit `expect` call
        // that would have called `check_pending_lex_error`).
        let message = match self.current.kind {
            TokenKind::ErrUnterminatedString => {
                "unterminated string literal: missing closing single quote".to_string()
            }
            TokenKind::ErrUnterminatedQuotedIdentifier => {
                "unterminated quoted identifier: missing closing double quote".to_string()
            }
            TokenKind::ErrUnterminatedBlockComment => {
                "unterminated block comment: missing closing `*/`".to_string()
            }
            _ => message,
        };
        ParseError {
            message,
            span: self.current.span,
        }
    }

    fn push_nesting(&mut self) -> Result<(), ParseError> {
        self.nesting += 1;
        if self.nesting > MAX_NESTING_DEPTH {
            Err(self.error("query exceeds maximum nesting depth".into()))
        } else {
            Ok(())
        }
    }

    fn pop_nesting(&mut self) {
        self.nesting -= 1;
    }

    // ─── Top-level ───────────────────────────────────────────────────────

    fn parse_program(&mut self) -> Result<SqlProgram, ParseError> {
        let query = self.parse_query()?;
        if !self.at_eof() {
            return Err(self.error(format!("unexpected token: {}", self.current.kind)));
        }
        Ok(SqlProgram { query })
    }

    fn parse_query(&mut self) -> Result<SqlQuery, ParseError> {
        self.push_nesting()?;
        let select = self.parse_select_clause()?;
        let from = self.parse_opt_from_clause()?;
        let where_clause = self.parse_opt_where_clause()?;
        let group_by = self.parse_opt_group_by_clause()?;
        let order_by = self.parse_opt_order_by_clause()?;
        let offset_limit = self.parse_opt_offset_limit_clause()?;
        self.pop_nesting();
        Ok(SqlQuery {
            select,
            from,
            where_clause,
            group_by,
            order_by,
            offset_limit,
        })
    }

    // ─── SELECT ──────────────────────────────────────────────────────────

    fn parse_select_clause(&mut self) -> Result<SqlSelectClause, ParseError> {
        self.expect(TokenKind::Select)?;
        let distinct = self.consume_if(TokenKind::Distinct);
        let top = self.parse_opt_top_spec()?;
        let spec = self.parse_select_spec()?;
        Ok(SqlSelectClause {
            distinct,
            top,
            spec,
        })
    }

    fn parse_opt_top_spec(&mut self) -> Result<Option<SqlTopSpec>, ParseError> {
        if !self.consume_if(TokenKind::Top) {
            return Ok(None);
        }
        match self.current.kind {
            TokenKind::IntegerLiteral => {
                let n: i64 = self
                    .current
                    .text
                    .parse()
                    .map_err(|_| self.error("invalid TOP value".into()))?;
                self.advance();
                Ok(Some(SqlTopSpec::Literal(n)))
            }
            TokenKind::FloatLiteral => Err(self.error(
                "TOP value must be an integer literal or @parameter; floating-point not allowed"
                    .into(),
            )),
            TokenKind::Parameter => {
                let name = extract_parameter_name(self.current.text).to_string();
                self.advance();
                Ok(Some(SqlTopSpec::Parameter(name)))
            }
            _ => Err(self.error("expected number or parameter after TOP".into())),
        }
    }

    fn parse_select_spec(&mut self) -> Result<SqlSelectSpec, ParseError> {
        if self.consume_if(TokenKind::Star) {
            return Ok(SqlSelectSpec::Star);
        }
        if self.consume_if(TokenKind::Value) {
            let expr = self.parse_scalar_expression()?;
            return Ok(SqlSelectSpec::Value(Box::new(expr)));
        }
        // SELECT list
        let mut items = vec![self.parse_select_item()?];
        while self.consume_if(TokenKind::Comma) {
            items.push(self.parse_select_item()?);
        }
        Ok(SqlSelectSpec::List(items))
    }

    fn parse_select_item(&mut self) -> Result<SqlSelectItem, ParseError> {
        let expression = self.parse_scalar_expression()?;
        let alias = self.parse_opt_alias()?;
        Ok(SqlSelectItem { expression, alias })
    }

    fn parse_opt_alias(&mut self) -> Result<Option<String>, ParseError> {
        if self.consume_if(TokenKind::As) {
            return Ok(Some(self.parse_identifier_name()?));
        }
        // Identifier without AS keyword (but not a keyword that starts a clause)
        if self.current.kind == TokenKind::Identifier {
            let name = self.current.text.to_string();
            self.advance();
            return Ok(Some(name));
        }
        if self.current.kind == TokenKind::StringLiteral {
            let name = extract_string_content(self.current.text);
            self.advance();
            return Ok(Some(name));
        }
        Ok(None)
    }

    // ─── FROM ────────────────────────────────────────────────────────────

    fn parse_opt_from_clause(&mut self) -> Result<Option<SqlFromClause>, ParseError> {
        if !self.consume_if(TokenKind::From) {
            return Ok(None);
        }
        let collection = self.parse_collection_expression()?;
        Ok(Some(SqlFromClause { collection }))
    }

    fn parse_collection_expression(&mut self) -> Result<SqlCollectionExpression, ParseError> {
        let mut left = self.parse_primary_collection_expression()?;
        while self.consume_if(TokenKind::Join) {
            let right = self.parse_primary_collection_expression()?;
            left = SqlCollectionExpression::Join {
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_primary_collection_expression(
        &mut self,
    ) -> Result<SqlCollectionExpression, ParseError> {
        // Check for: <id> IN <collection>
        if self.current.kind == TokenKind::Identifier {
            let name = self.current.text.to_string();
            self.advance();
            if self.consume_if(TokenKind::In) {
                let collection = self.parse_collection_source()?;
                return Ok(SqlCollectionExpression::ArrayIterator {
                    identifier: name,
                    collection,
                });
            }
            // Not an array iterator — put the identifier back as start of a collection path
            // Actually, we already consumed the identifier, so build the collection path from it
            let path = self.parse_path_continuation()?;
            let collection = SqlCollection::Path { root: name, path };
            let alias = self.parse_opt_collection_alias()?;
            return Ok(SqlCollectionExpression::Aliased { collection, alias });
        }

        // Subquery: ( <query> )
        if self.at(TokenKind::LParen) {
            let collection = self.parse_collection_source()?;
            let alias = self.parse_opt_collection_alias()?;
            return Ok(SqlCollectionExpression::Aliased { collection, alias });
        }

        Err(self.error("expected collection expression".into()))
    }

    fn parse_collection_source(&mut self) -> Result<SqlCollection, ParseError> {
        if self.consume_if(TokenKind::LParen) {
            let query = self.parse_query()?;
            self.expect(TokenKind::RParen)?;
            return Ok(SqlCollection::Subquery(Box::new(query)));
        }
        let root = self.parse_identifier_name()?;
        let path = self.parse_path_continuation()?;
        Ok(SqlCollection::Path { root, path })
    }

    fn parse_path_continuation(&mut self) -> Result<Vec<SqlPathSegment>, ParseError> {
        let mut segments = Vec::new();
        loop {
            if self.consume_if(TokenKind::Dot) {
                let name = self.parse_identifier_name()?;
                segments.push(SqlPathSegment::Identifier(name));
            } else if self.consume_if(TokenKind::LBracket) {
                match self.current.kind {
                    TokenKind::IntegerLiteral => {
                        let idx: i64 = self
                            .current
                            .text
                            .parse()
                            .map_err(|_| self.error("invalid array index".into()))?;
                        self.advance();
                        self.expect(TokenKind::RBracket)?;
                        segments.push(SqlPathSegment::Index(idx));
                    }
                    TokenKind::StringLiteral => {
                        let s = extract_string_content(self.current.text);
                        self.advance();
                        self.expect(TokenKind::RBracket)?;
                        segments.push(SqlPathSegment::StringIndex(s));
                    }
                    _ => return Err(self.error("expected integer or string in brackets".into())),
                }
            } else {
                break;
            }
        }
        Ok(segments)
    }

    fn parse_opt_collection_alias(&mut self) -> Result<Option<String>, ParseError> {
        if self.consume_if(TokenKind::As) {
            return Ok(Some(self.parse_identifier_name()?));
        }
        // Bare identifier alias (not a clause keyword)
        if self.current.kind == TokenKind::Identifier && !self.is_clause_keyword() {
            let name = self.current.text.to_string();
            self.advance();
            return Ok(Some(name));
        }
        Ok(None)
    }

    fn is_clause_keyword(&self) -> bool {
        matches!(
            self.current.kind,
            TokenKind::Where
                | TokenKind::Group
                | TokenKind::Order
                | TokenKind::Offset
                | TokenKind::Limit
                | TokenKind::Join
                | TokenKind::Select
        )
    }

    // ─── WHERE ───────────────────────────────────────────────────────────

    fn parse_opt_where_clause(&mut self) -> Result<Option<SqlWhereClause>, ParseError> {
        if !self.consume_if(TokenKind::Where) {
            return Ok(None);
        }
        let expression = self.parse_scalar_expression()?;
        Ok(Some(SqlWhereClause { expression }))
    }

    // ─── GROUP BY ────────────────────────────────────────────────────────

    fn parse_opt_group_by_clause(&mut self) -> Result<Option<SqlGroupByClause>, ParseError> {
        if !self.consume_if(TokenKind::Group) {
            return Ok(None);
        }
        self.expect(TokenKind::By)?;
        let mut expressions = vec![self.parse_scalar_expression()?];
        while self.consume_if(TokenKind::Comma) {
            expressions.push(self.parse_scalar_expression()?);
        }
        Ok(Some(SqlGroupByClause { expressions }))
    }

    // ─── ORDER BY ────────────────────────────────────────────────────────

    fn parse_opt_order_by_clause(&mut self) -> Result<Option<SqlOrderByClause>, ParseError> {
        if !self.consume_if(TokenKind::Order) {
            return Ok(None);
        }
        self.expect(TokenKind::By)?;
        let mut items = vec![self.parse_order_by_item()?];
        while self.consume_if(TokenKind::Comma) {
            items.push(self.parse_order_by_item()?);
        }
        Ok(Some(SqlOrderByClause { items }))
    }

    fn parse_order_by_item(&mut self) -> Result<SqlOrderByItem, ParseError> {
        let expression = self.parse_scalar_expression()?;
        let order = if self.consume_if(TokenKind::Asc) {
            SqlSortOrder::Ascending
        } else if self.consume_if(TokenKind::Desc) {
            SqlSortOrder::Descending
        } else {
            SqlSortOrder::Unspecified
        };
        Ok(SqlOrderByItem { expression, order })
    }

    // ─── OFFSET LIMIT ────────────────────────────────────────────────────

    fn parse_opt_offset_limit_clause(
        &mut self,
    ) -> Result<Option<SqlOffsetLimitClause>, ParseError> {
        if !self.at(TokenKind::Offset) {
            return Ok(None);
        }
        self.advance();
        let offset = self.parse_offset_or_limit_value()?;
        self.expect(TokenKind::Limit)?;
        let limit = self.parse_offset_or_limit_value()?;
        Ok(Some(SqlOffsetLimitClause {
            offset: match offset {
                OffsetLimitVal::Lit(n) => SqlOffsetSpec::Literal(n),
                OffsetLimitVal::Param(p) => SqlOffsetSpec::Parameter(p),
            },
            limit: match limit {
                OffsetLimitVal::Lit(n) => SqlLimitSpec::Literal(n),
                OffsetLimitVal::Param(p) => SqlLimitSpec::Parameter(p),
            },
        }))
    }

    fn parse_offset_or_limit_value(&mut self) -> Result<OffsetLimitVal, ParseError> {
        match self.current.kind {
            TokenKind::IntegerLiteral => {
                let n: i64 = self
                    .current
                    .text
                    .parse()
                    .map_err(|_| self.error("invalid integer".into()))?;
                self.advance();
                Ok(OffsetLimitVal::Lit(n))
            }
            TokenKind::Parameter => {
                let name = extract_parameter_name(self.current.text).to_string();
                self.advance();
                Ok(OffsetLimitVal::Param(name))
            }
            _ => Err(self.error("expected integer or parameter for OFFSET/LIMIT".into())),
        }
    }

    // ─── Scalar Expressions (Pratt parser) ───────────────────────────────

    fn parse_scalar_expression(&mut self) -> Result<SqlScalarExpression, ParseError> {
        self.push_nesting()?;
        let result = self.parse_ternary();
        self.pop_nesting();
        result
    }

    /// Ternary: `expr ? expr : expr` and coalesce `expr ?? expr`
    fn parse_ternary(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let expr = self.parse_or()?;
        if self.consume_if(TokenKind::Question) {
            if self.at(TokenKind::Question) {
                // Actually this was ?? — but we already consumed the first ?, oops.
                // The lexer handles ?? as Coalesce, so this path won't happen.
                // But just in case:
                self.advance();
                let right = self.parse_or()?;
                return Ok(SqlScalarExpression::Coalesce {
                    left: Box::new(expr),
                    right: Box::new(right),
                });
            }
            let if_true = self.parse_scalar_expression()?;
            self.expect(TokenKind::Colon)?;
            let if_false = self.parse_scalar_expression()?;
            return Ok(SqlScalarExpression::Conditional {
                condition: Box::new(expr),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
            });
        }
        if self.consume_if(TokenKind::Coalesce) {
            let right = self.parse_scalar_expression()?;
            return Ok(SqlScalarExpression::Coalesce {
                left: Box::new(expr),
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    /// OR
    fn parse_or(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_and()?;
        while self.consume_if(TokenKind::Or) {
            let right = self.parse_and()?;
            left = SqlScalarExpression::Binary {
                op: SqlBinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// AND
    fn parse_and(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_not()?;
        while self.consume_if(TokenKind::And) {
            let right = self.parse_not()?;
            left = SqlScalarExpression::Binary {
                op: SqlBinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// NOT (unary prefix)
    fn parse_not(&mut self) -> Result<SqlScalarExpression, ParseError> {
        if self.consume_if(TokenKind::Not) {
            let operand = self.parse_not()?;
            return Ok(SqlScalarExpression::Unary {
                op: SqlUnaryOp::Not,
                operand: Box::new(operand),
            });
        }
        self.parse_in_between_like()
    }

    /// IN, BETWEEN, LIKE (postfix on comparison expressions)
    fn parse_in_between_like(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let expr = self.parse_comparison()?;

        // NOT IN / NOT BETWEEN / NOT LIKE
        if self.at(TokenKind::Not) {
            self.advance();
            match self.current.kind {
                TokenKind::In => {
                    self.advance();
                    return self.parse_in_list(expr, true);
                }
                TokenKind::Between => {
                    self.advance();
                    return self.parse_between(expr, true);
                }
                TokenKind::Like => {
                    self.advance();
                    return self.parse_like(expr, true);
                }
                _ => {
                    // We consumed NOT but the next token is not IN/BETWEEN/LIKE, so this is
                    // a parse error. Previously this arm silently re-wrapped the already-
                    // parsed expression as NOT (expr), inverting the user's predicate.
                    return Err(self.error(
                        "NOT must be followed by IN, BETWEEN, or LIKE in this position".into(),
                    ));
                }
            }
        }

        match self.current.kind {
            TokenKind::In => {
                self.advance();
                return self.parse_in_list(expr, false);
            }
            TokenKind::Between => {
                self.advance();
                return self.parse_between(expr, false);
            }
            TokenKind::Like => {
                self.advance();
                return self.parse_like(expr, false);
            }
            TokenKind::Is => {
                self.advance();
                let not = self.consume_if(TokenKind::Not);
                self.expect(TokenKind::Null)?;
                return Ok(SqlScalarExpression::IsNull {
                    expression: Box::new(expr),
                    not,
                });
            }
            _ => {}
        }

        Ok(expr)
    }

    fn parse_in_list(
        &mut self,
        expr: SqlScalarExpression,
        not: bool,
    ) -> Result<SqlScalarExpression, ParseError> {
        self.expect(TokenKind::LParen)?;
        let mut items = vec![self.parse_scalar_expression()?];
        while self.consume_if(TokenKind::Comma) {
            items.push(self.parse_scalar_expression()?);
        }
        self.expect(TokenKind::RParen)?;
        Ok(SqlScalarExpression::In {
            expression: Box::new(expr),
            items,
            not,
        })
    }

    fn parse_between(
        &mut self,
        expr: SqlScalarExpression,
        not: bool,
    ) -> Result<SqlScalarExpression, ParseError> {
        let low = self.parse_comparison()?;
        self.expect(TokenKind::And)?;
        let high = self.parse_comparison()?;
        Ok(SqlScalarExpression::Between {
            expression: Box::new(expr),
            low: Box::new(low),
            high: Box::new(high),
            not,
        })
    }

    fn parse_like(
        &mut self,
        expr: SqlScalarExpression,
        not: bool,
    ) -> Result<SqlScalarExpression, ParseError> {
        let pattern = self.parse_comparison()?;
        let escape = if self.consume_if(TokenKind::Escape) {
            let tok = self.expect(TokenKind::StringLiteral)?;
            Some(extract_string_content(tok.text))
        } else {
            None
        };
        Ok(SqlScalarExpression::Like {
            expression: Box::new(expr),
            pattern: Box::new(pattern),
            escape,
            not,
        })
    }

    /// Comparison: =, !=, <, >, <=, >=
    fn parse_comparison(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_bitwise_or()?;
        loop {
            let op = match self.current.kind {
                TokenKind::Eq => SqlBinaryOp::Equal,
                TokenKind::NotEq => SqlBinaryOp::NotEqual,
                TokenKind::Lt => SqlBinaryOp::LessThan,
                TokenKind::Gt => SqlBinaryOp::GreaterThan,
                TokenKind::LtEq => SqlBinaryOp::LessThanOrEqual,
                TokenKind::GtEq => SqlBinaryOp::GreaterThanOrEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_bitwise_or()?;
            left = SqlScalarExpression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Bitwise OR: |
    fn parse_bitwise_or(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_bitwise_xor()?;
        while self.current.kind == TokenKind::Pipe {
            self.advance();
            let right = self.parse_bitwise_xor()?;
            left = SqlScalarExpression::Binary {
                op: SqlBinaryOp::BitwiseOr,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Bitwise XOR: ^
    fn parse_bitwise_xor(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_bitwise_and()?;
        while self.consume_if(TokenKind::Caret) {
            let right = self.parse_bitwise_and()?;
            left = SqlScalarExpression::Binary {
                op: SqlBinaryOp::BitwiseXor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Bitwise AND: &
    fn parse_bitwise_and(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_shift()?;
        while self.current.kind == TokenKind::Ampersand {
            self.advance();
            let right = self.parse_shift()?;
            left = SqlScalarExpression::Binary {
                op: SqlBinaryOp::BitwiseAnd,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Shift: <<, >>, >>>
    fn parse_shift(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_string_concat()?;
        loop {
            let op = match self.current.kind {
                TokenKind::LeftShift => SqlBinaryOp::LeftShift,
                TokenKind::RightShift => SqlBinaryOp::RightShift,
                TokenKind::ZeroFillRightShift => SqlBinaryOp::ZeroFillRightShift,
                _ => break,
            };
            self.advance();
            let right = self.parse_string_concat()?;
            left = SqlScalarExpression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// String concat: ||
    fn parse_string_concat(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_additive()?;
        while self.consume_if(TokenKind::StringConcat) {
            let right = self.parse_additive()?;
            left = SqlScalarExpression::Binary {
                op: SqlBinaryOp::StringConcat,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Addition / Subtraction: +, -
    fn parse_additive(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.current.kind {
                TokenKind::Plus => SqlBinaryOp::Add,
                TokenKind::Minus => SqlBinaryOp::Subtract,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = SqlScalarExpression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Multiplication / Division / Modulo: *, /, %
    fn parse_multiplicative(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.current.kind {
                TokenKind::Star => SqlBinaryOp::Multiply,
                TokenKind::Slash => SqlBinaryOp::Divide,
                TokenKind::Percent => SqlBinaryOp::Modulo,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = SqlScalarExpression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Unary: -, +, ~, NOT
    fn parse_unary(&mut self) -> Result<SqlScalarExpression, ParseError> {
        match self.current.kind {
            TokenKind::Minus => {
                self.advance();
                // Optimization: fold unary minus into integer/float literals
                match self.current.kind {
                    TokenKind::IntegerLiteral => {
                        let n: i64 = self
                            .current
                            .text
                            .parse()
                            .map_err(|_| self.error("invalid integer".into()))?;
                        self.advance();
                        let expr = SqlScalarExpression::Literal(SqlLiteral::Integer(-n));
                        self.parse_postfix(expr)
                    }
                    TokenKind::FloatLiteral => {
                        let n: f64 = self
                            .current
                            .text
                            .parse()
                            .map_err(|_| self.error("invalid float".into()))?;
                        self.advance();
                        let expr = SqlScalarExpression::Literal(SqlLiteral::Number(-n));
                        self.parse_postfix(expr)
                    }
                    _ => {
                        let operand = self.parse_unary()?;
                        Ok(SqlScalarExpression::Unary {
                            op: SqlUnaryOp::Minus,
                            operand: Box::new(operand),
                        })
                    }
                }
            }
            TokenKind::Plus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(SqlScalarExpression::Unary {
                    op: SqlUnaryOp::Plus,
                    operand: Box::new(operand),
                })
            }
            TokenKind::Tilde => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(SqlScalarExpression::Unary {
                    op: SqlUnaryOp::BitwiseNot,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary_expression(),
        }
    }

    /// Primary expressions: literals, identifiers, function calls, parenthesized, array/object constructors
    fn parse_primary_expression(&mut self) -> Result<SqlScalarExpression, ParseError> {
        let expr = match self.current.kind {
            // String literal
            TokenKind::StringLiteral => {
                let s = extract_string_content(self.current.text);
                self.advance();
                SqlScalarExpression::Literal(SqlLiteral::String(s))
            }

            // Integer literal
            TokenKind::IntegerLiteral => {
                let n: i64 = self
                    .current
                    .text
                    .parse()
                    .map_err(|_| self.error("invalid integer".into()))?;
                self.advance();
                SqlScalarExpression::Literal(SqlLiteral::Integer(n))
            }

            // Float literal
            TokenKind::FloatLiteral => {
                let n: f64 = self
                    .current
                    .text
                    .parse()
                    .map_err(|_| self.error("invalid float".into()))?;
                self.advance();
                SqlScalarExpression::Literal(SqlLiteral::Number(n))
            }

            // Boolean / null / undefined
            TokenKind::True => {
                self.advance();
                SqlScalarExpression::Literal(SqlLiteral::Boolean(true))
            }
            TokenKind::False => {
                self.advance();
                SqlScalarExpression::Literal(SqlLiteral::Boolean(false))
            }
            TokenKind::Null => {
                self.advance();
                SqlScalarExpression::Literal(SqlLiteral::Null)
            }
            TokenKind::Undefined => {
                self.advance();
                SqlScalarExpression::Literal(SqlLiteral::Undefined)
            }

            // Parameter
            TokenKind::Parameter => {
                let name = extract_parameter_name(self.current.text).to_string();
                self.advance();
                SqlScalarExpression::ParameterRef(name)
            }

            // EXISTS ( subquery )
            TokenKind::Exists => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let query = self.parse_query()?;
                self.expect(TokenKind::RParen)?;
                SqlScalarExpression::Exists(Box::new(query))
            }

            // ARRAY ( subquery )
            TokenKind::Array => {
                let array_text = self.current.text.to_string();
                self.advance();
                if self.at(TokenKind::LParen) {
                    self.advance();
                    let query = self.parse_query()?;
                    self.expect(TokenKind::RParen)?;
                    SqlScalarExpression::Array(Box::new(query))
                } else {
                    // preserve source casing for keyword-as-property.
                    // `c.array` must look up `"array"`, not `"ARRAY"`.
                    SqlScalarExpression::PropertyRef(array_text)
                }
            }

            // Array literal: [ expr, expr, ... ]
            TokenKind::LBracket => {
                self.advance();
                let mut items = Vec::new();
                if !self.at(TokenKind::RBracket) {
                    items.push(self.parse_scalar_expression()?);
                    while self.consume_if(TokenKind::Comma) {
                        items.push(self.parse_scalar_expression()?);
                    }
                }
                self.expect(TokenKind::RBracket)?;
                SqlScalarExpression::ArrayCreate(items)
            }

            // Object literal: { name: expr, ... }
            TokenKind::LBrace => {
                self.advance();
                let mut props = Vec::new();
                if !self.at(TokenKind::RBrace) {
                    props.push(self.parse_object_property()?);
                    while self.consume_if(TokenKind::Comma) {
                        props.push(self.parse_object_property()?);
                    }
                }
                self.expect(TokenKind::RBrace)?;
                SqlScalarExpression::ObjectCreate(props)
            }

            // Parenthesized expression or subquery
            TokenKind::LParen => {
                self.push_nesting()?;
                self.advance();
                // Check if this is a subquery (starts with SELECT)
                let result = if self.at(TokenKind::Select) {
                    let query = self.parse_query()?;
                    self.expect(TokenKind::RParen)?;
                    SqlScalarExpression::Subquery(Box::new(query))
                } else {
                    let expr = self.parse_scalar_expression()?;
                    self.expect(TokenKind::RParen)?;
                    expr
                };
                self.pop_nesting();
                result
            }

            // UDF function call: udf.name(args)
            TokenKind::Udf => {
                self.advance();
                self.expect(TokenKind::Dot)?;
                let name = self.parse_identifier_name()?;
                self.expect(TokenKind::LParen)?;
                let args = self.parse_argument_list()?;
                self.expect(TokenKind::RParen)?;
                SqlScalarExpression::FunctionCall {
                    name,
                    args,
                    is_udf: true,
                }
            }

            // Identifier — could be property ref or function call
            // Also handle keywords that can appear as identifiers in certain contexts
            // (LEFT, RIGHT, LET, RANK, etc.)
            TokenKind::Identifier
            | TokenKind::Left
            | TokenKind::Right
            | TokenKind::Let
            | TokenKind::Rank
            | TokenKind::Value => {
                let name = if self.current.kind == TokenKind::Identifier {
                    extract_identifier(self.current.text).to_string()
                } else {
                    // preserve the source casing of keyword-as-identifier.
                    // Cosmos JSON property lookup is case-sensitive, so
                    // `c.left` must search for the property `"left"`, not
                    // `"LEFT"`. The previous `to_ascii_uppercase` collapsed
                    // both casings to `"LEFT"` and silently produced wrong
                    // member-access results.
                    self.current.text.to_string()
                };
                self.advance();

                // Function call: name(args)
                if self.at(TokenKind::LParen) {
                    self.advance();
                    // Check for aggregate-like subquery: name( SELECT ... )
                    if self.at(TokenKind::Select) {
                        let query = self.parse_query()?;
                        self.expect(TokenKind::RParen)?;
                        // Treat as subquery function (ALL, FIRST, LAST)
                        let upper = name.to_ascii_uppercase();
                        return match upper.as_str() {
                            "EXISTS" => Ok(SqlScalarExpression::Exists(Box::new(query))),
                            "ARRAY" => Ok(SqlScalarExpression::Array(Box::new(query))),
                            _ => Ok(SqlScalarExpression::Subquery(Box::new(query))),
                        };
                    }
                    let args = self.parse_argument_list()?;
                    self.expect(TokenKind::RParen)?;
                    SqlScalarExpression::FunctionCall {
                        name,
                        args,
                        is_udf: false,
                    }
                } else {
                    SqlScalarExpression::PropertyRef(name)
                }
            }

            _ => return Err(self.error(format!("unexpected token: {}", self.current.kind))),
        };

        // Parse postfix: member access (.member), indexer ([expr])
        self.parse_postfix(expr)
    }

    fn parse_postfix(
        &mut self,
        mut expr: SqlScalarExpression,
    ) -> Result<SqlScalarExpression, ParseError> {
        loop {
            if self.consume_if(TokenKind::Dot) {
                let member = self.parse_identifier_name()?;
                expr = SqlScalarExpression::MemberRef {
                    source: Box::new(expr),
                    member,
                };
            } else if self.consume_if(TokenKind::LBracket) {
                let index = self.parse_scalar_expression()?;
                self.expect(TokenKind::RBracket)?;
                expr = SqlScalarExpression::MemberIndexer {
                    source: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_argument_list(&mut self) -> Result<Vec<SqlScalarExpression>, ParseError> {
        if self.at(TokenKind::RParen) {
            return Ok(Vec::new());
        }
        let mut args = vec![self.parse_scalar_expression()?];
        while self.consume_if(TokenKind::Comma) {
            args.push(self.parse_scalar_expression()?);
        }
        Ok(args)
    }

    fn parse_object_property(&mut self) -> Result<SqlObjectProperty, ParseError> {
        let name = match self.current.kind {
            TokenKind::StringLiteral => {
                let s = extract_string_content(self.current.text);
                self.advance();
                s
            }
            _ => self.parse_identifier_name()?,
        };
        self.expect(TokenKind::Colon)?;
        let expression = self.parse_scalar_expression()?;
        Ok(SqlObjectProperty { name, expression })
    }

    fn parse_identifier_name(&mut self) -> Result<String, ParseError> {
        match self.current.kind {
            TokenKind::Identifier => {
                let name = extract_identifier(self.current.text).to_string();
                self.advance();
                Ok(name)
            }
            // Allow keywords as identifiers in property positions
            TokenKind::Left
            | TokenKind::Right
            | TokenKind::Value
            | TokenKind::Let
            | TokenKind::Rank
            | TokenKind::Set
            | TokenKind::Over
            | TokenKind::For
            | TokenKind::Top
            | TokenKind::Asc
            | TokenKind::Desc
            | TokenKind::Distinct
            | TokenKind::Null
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Undefined
            | TokenKind::Array
            | TokenKind::Order
            | TokenKind::Group
            | TokenKind::Offset
            | TokenKind::Limit
            | TokenKind::Select
            | TokenKind::From
            | TokenKind::Where
            | TokenKind::By
            | TokenKind::As
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Not
            | TokenKind::In
            | TokenKind::Between
            | TokenKind::Like
            | TokenKind::Escape
            | TokenKind::Join
            | TokenKind::Cross
            | TokenKind::Inner
            | TokenKind::Exists
            | TokenKind::Is
            | TokenKind::Having
            | TokenKind::Udf => {
                let name = self.current.text.to_string();
                self.advance();
                Ok(name)
            }
            _ => Err(self.error(format!("expected identifier, found {}", self.current.kind))),
        }
    }
}

enum OffsetLimitVal {
    Lit(i64),
    Param(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_select_star() {
        let p = parse("SELECT * FROM c").unwrap();
        assert_eq!(p.query.select.spec, SqlSelectSpec::Star);
        assert!(p.query.from.is_some());
    }

    #[test]
    fn parse_select_value() {
        let p = parse("SELECT VALUE c.name FROM c").unwrap();
        assert!(
            matches!(p.query.select.spec, SqlSelectSpec::Value(_)),
            "expected SqlSelectSpec::Value, got {:?}",
            p.query.select.spec
        );
    }

    #[test]
    fn parse_where_equality() {
        let p = parse("SELECT * FROM c WHERE c.pk = 'hello'").unwrap();
        assert!(p.query.where_clause.is_some());
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::Binary {
                op: SqlBinaryOp::Equal,
                left,
                right,
            } => {
                // left should be c.pk
                match left.as_ref() {
                    SqlScalarExpression::MemberRef { source, member } => {
                        assert_eq!(member, "pk");
                        match source.as_ref() {
                            SqlScalarExpression::PropertyRef(name) => assert_eq!(name, "c"),
                            _ => panic!("expected PropertyRef"),
                        }
                    }
                    _ => panic!("expected MemberRef"),
                }
                // right should be 'hello'
                match right.as_ref() {
                    SqlScalarExpression::Literal(SqlLiteral::String(s)) => {
                        assert_eq!(s, "hello")
                    }
                    _ => panic!("expected string literal"),
                }
            }
            _ => panic!("expected binary equal"),
        }
    }

    #[test]
    fn parse_complex_query() {
        let p = parse(
            "SELECT c.name, c.age AS a FROM c WHERE c.pk = 'x' AND c.age > 21 ORDER BY c.age DESC OFFSET 0 LIMIT 10",
        )
        .unwrap();
        assert!(!p.query.select.distinct);
        assert!(
            matches!(p.query.select.spec, SqlSelectSpec::List(_)),
            "expected SqlSelectSpec::List"
        );
        assert!(p.query.order_by.is_some());
        assert!(p.query.offset_limit.is_some());
    }

    #[test]
    fn parse_top() {
        let p = parse("SELECT TOP 10 * FROM c").unwrap();
        assert_eq!(p.query.select.top, Some(SqlTopSpec::Literal(10)));
    }

    #[test]
    fn parse_distinct() {
        let p = parse("SELECT DISTINCT c.name FROM c").unwrap();
        assert!(p.query.select.distinct);
    }

    #[test]
    fn parse_in_expression() {
        let p = parse("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c')").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::In { items, not, .. } => {
                assert!(!not);
                assert_eq!(items.len(), 3);
            }
            _ => panic!("expected IN expression"),
        }
    }

    #[test]
    fn parse_between() {
        let p = parse("SELECT * FROM c WHERE c.age BETWEEN 18 AND 65").unwrap();
        let w = p.query.where_clause.unwrap();
        assert!(
            matches!(w.expression, SqlScalarExpression::Between { .. }),
            "expected SqlScalarExpression::Between"
        );
    }

    #[test]
    fn parse_function_call() {
        let p = parse("SELECT * FROM c WHERE CONTAINS(c.name, 'test')").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::FunctionCall {
                name, args, is_udf, ..
            } => {
                assert_eq!(name, "CONTAINS");
                assert_eq!(args.len(), 2);
                assert!(!is_udf);
            }
            _ => panic!("expected function call"),
        }
    }

    #[test]
    fn parse_parameter() {
        let p = parse("SELECT * FROM c WHERE c.id = @id").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::Binary { right, .. } => match right.as_ref() {
                SqlScalarExpression::ParameterRef(name) => assert_eq!(name, "id"),
                _ => panic!("expected parameter ref"),
            },
            _ => panic!("expected binary expression"),
        }
    }

    #[test]
    fn parse_array_literal() {
        let p = parse("SELECT [1, 2, 3] FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::ArrayCreate(items) => assert_eq!(items.len(), 3),
                _ => panic!("expected array create"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_object_literal() {
        let p = parse("SELECT {'name': c.name, 'age': c.age} FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::ObjectCreate(props) => assert_eq!(props.len(), 2),
                _ => panic!("expected object create"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_join() {
        let p = parse("SELECT * FROM c JOIN t IN c.tags").unwrap();
        let from = p.query.from.unwrap();
        assert!(
            matches!(from.collection, SqlCollectionExpression::Join { .. }),
            "expected SqlCollectionExpression::Join"
        );
    }

    #[test]
    fn parse_group_by() {
        let p = parse("SELECT c.city, COUNT(1) FROM c GROUP BY c.city").unwrap();
        assert!(p.query.group_by.is_some());
    }

    #[test]
    fn parse_is_null() {
        let p = parse("SELECT * FROM c WHERE c.x IS NULL").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::IsNull { not, .. } => assert!(!not),
            _ => panic!("expected IS NULL"),
        }
    }

    #[test]
    fn parse_is_not_null() {
        let p = parse("SELECT * FROM c WHERE c.x IS NOT NULL").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::IsNull { not, .. } => assert!(*not),
            _ => panic!("expected IS NOT NULL"),
        }
    }

    #[test]
    fn parse_nested_member_access() {
        let p = parse("SELECT c.a.b.c FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => {
                // Should be MemberRef(MemberRef(MemberRef(PropertyRef("c"), "a"), "b"), "c")
                let expr = &items[0].expression;
                match expr {
                    SqlScalarExpression::MemberRef { member, .. } => assert_eq!(member, "c"),
                    _ => panic!("expected member ref"),
                }
            }
            _ => panic!("expected list"),
        }
    }

    #[test]
    fn parse_udf_call() {
        let p = parse("SELECT * FROM c WHERE udf.myFunc(c.x)").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::FunctionCall { name, is_udf, .. } => {
                assert_eq!(name, "myFunc");
                assert!(is_udf);
            }
            _ => panic!("expected UDF call"),
        }
    }

    #[test]
    fn parse_negative_number() {
        let p = parse("SELECT * FROM c WHERE c.x = -42").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::Binary { right, .. } => match right.as_ref() {
                SqlScalarExpression::Literal(SqlLiteral::Integer(-42)) => {}
                _ => panic!("expected -42 literal"),
            },
            _ => panic!("expected binary"),
        }
    }

    // ── Expression parsing ──────────────────────────────────────────────

    #[test]
    fn parse_string_concat() {
        let p = parse("SELECT c.first || ' ' || c.last FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::Binary {
                    op: SqlBinaryOp::StringConcat,
                    ..
                } => {}
                _ => panic!("expected StringConcat"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_coalesce() {
        let p = parse("SELECT c.name ?? 'unknown' FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::Coalesce { .. } => {}
                _ => panic!("expected Coalesce"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_ternary() {
        let p = parse("SELECT c.age > 18 ? 'adult' : 'child' FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::Conditional { .. } => {}
                _ => panic!("expected Conditional"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_array_create_empty() {
        let p = parse("SELECT [] FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::ArrayCreate(elements) => assert!(elements.is_empty()),
                _ => panic!("expected empty ArrayCreate"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_object_create_complex() {
        let p = parse("SELECT {'name': c.name, 'info': {'age': c.age}} FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::ObjectCreate(props) => {
                    assert_eq!(props.len(), 2);
                    assert_eq!(props[0].name, "name");
                    assert_eq!(props[1].name, "info");
                    // nested object
                    match &props[1].expression {
                        SqlScalarExpression::ObjectCreate(inner) => {
                            assert_eq!(inner.len(), 1);
                            assert_eq!(inner[0].name, "age");
                        }
                        _ => panic!("expected nested ObjectCreate"),
                    }
                }
                _ => panic!("expected ObjectCreate"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_not_in() {
        let p = parse("SELECT * FROM c WHERE c.x NOT IN (1, 2)").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::In { not, items, .. } => {
                assert!(*not);
                assert_eq!(items.len(), 2);
            }
            _ => panic!("expected NOT IN"),
        }
    }

    #[test]
    fn parse_not_between() {
        let p = parse("SELECT * FROM c WHERE c.x NOT BETWEEN 1 AND 10").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::Between { not, .. } => assert!(*not),
            _ => panic!("expected NOT BETWEEN"),
        }
    }

    #[test]
    fn parse_not_like() {
        let p = parse("SELECT * FROM c WHERE c.name NOT LIKE '%test%'").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::Like { not, .. } => assert!(*not),
            _ => panic!("expected NOT LIKE"),
        }
    }

    #[test]
    fn parse_like_with_escape() {
        let p = parse(r"SELECT * FROM c WHERE c.name LIKE '%\_%' ESCAPE '\'").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::Like { escape, not, .. } => {
                assert!(!*not);
                assert_eq!(escape.as_deref(), Some("\\"));
            }
            _ => panic!("expected LIKE with ESCAPE"),
        }
    }

    #[test]
    fn parse_exists_subquery() {
        let p = parse("SELECT * FROM c WHERE EXISTS(SELECT VALUE 1 FROM c)").unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::Exists(q) => {
                assert!(matches!(q.select.spec, SqlSelectSpec::Value(_)));
            }
            _ => panic!("expected EXISTS subquery"),
        }
    }

    #[test]
    fn parse_array_subquery() {
        let p = parse("SELECT ARRAY(SELECT t FROM t IN c.tags) FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::Array(q) => {
                    assert!(q.from.is_some());
                }
                _ => panic!("expected ARRAY subquery"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_scalar_subquery_in_where() {
        let p = parse("SELECT * FROM c WHERE c.x = (SELECT VALUE MAX(t.id) FROM t IN c.items)")
            .unwrap();
        let w = p.query.where_clause.unwrap();
        match &w.expression {
            SqlScalarExpression::Binary {
                op: SqlBinaryOp::Equal,
                right,
                ..
            } => {
                assert!(matches!(right.as_ref(), SqlScalarExpression::Subquery(_)));
            }
            _ => panic!("expected binary equal with subquery"),
        }
    }

    #[test]
    fn parse_multiple_joins() {
        let p = parse("SELECT * FROM c JOIN t IN c.tags JOIN s IN c.skills").unwrap();
        let from = p.query.from.unwrap();
        match &from.collection {
            SqlCollectionExpression::Join { left, right } => {
                // right is the second JOIN (s IN c.skills)
                assert!(matches!(
                    right.as_ref(),
                    SqlCollectionExpression::ArrayIterator { .. }
                ));
                // left is the first JOIN (c JOIN t IN c.tags)
                assert!(matches!(
                    left.as_ref(),
                    SqlCollectionExpression::Join { .. }
                ));
            }
            _ => panic!("expected Join"),
        }
    }

    #[test]
    fn parse_offset_limit_params() {
        let p = parse("SELECT * FROM c OFFSET @off LIMIT @lim").unwrap();
        let ol = p.query.offset_limit.unwrap();
        assert_eq!(ol.offset, SqlOffsetSpec::Parameter("off".into()));
        assert_eq!(ol.limit, SqlLimitSpec::Parameter("lim".into()));
    }

    #[test]
    fn parse_top_parameter() {
        let p = parse("SELECT TOP @n * FROM c").unwrap();
        assert_eq!(p.query.select.top, Some(SqlTopSpec::Parameter("n".into())));
    }

    #[test]
    fn parse_bitwise_and_operator() {
        let p = parse("SELECT c.x & 255 FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::Binary {
                    op: SqlBinaryOp::BitwiseAnd,
                    ..
                } => {}
                _ => panic!("expected BitwiseAnd"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_shift_operators() {
        let p = parse("SELECT c.x << 2, c.x >> 1, c.x >>> 3 FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => {
                assert_eq!(items.len(), 3);
                match &items[0].expression {
                    SqlScalarExpression::Binary {
                        op: SqlBinaryOp::LeftShift,
                        ..
                    } => {}
                    _ => panic!("expected LeftShift"),
                }
                match &items[1].expression {
                    SqlScalarExpression::Binary {
                        op: SqlBinaryOp::RightShift,
                        ..
                    } => {}
                    _ => panic!("expected RightShift"),
                }
                match &items[2].expression {
                    SqlScalarExpression::Binary {
                        op: SqlBinaryOp::ZeroFillRightShift,
                        ..
                    } => {}
                    _ => panic!("expected ZeroFillRightShift"),
                }
            }
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_unary_plus() {
        let p = parse("SELECT +c.x FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::Unary {
                    op: SqlUnaryOp::Plus,
                    ..
                } => {}
                _ => panic!("expected unary Plus"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_unary_bitwise_not() {
        let p = parse("SELECT ~c.x FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::Unary {
                    op: SqlUnaryOp::BitwiseNot,
                    ..
                } => {}
                _ => panic!("expected unary BitwiseNot"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_nested_function() {
        let p = parse("SELECT UPPER(CONCAT(c.first, ' ', c.last)) FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::FunctionCall { name, args, .. } => {
                    assert_eq!(name, "UPPER");
                    assert_eq!(args.len(), 1);
                    match &args[0] {
                        SqlScalarExpression::FunctionCall {
                            name: inner_name,
                            args: inner_args,
                            ..
                        } => {
                            assert_eq!(inner_name, "CONCAT");
                            assert_eq!(inner_args.len(), 3);
                        }
                        _ => panic!("expected inner CONCAT"),
                    }
                }
                _ => panic!("expected FunctionCall"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_case_insensitive_keywords() {
        let p = parse("select * from c where c.x = 1 order by c.x").unwrap();
        assert_eq!(p.query.select.spec, SqlSelectSpec::Star);
        assert!(p.query.from.is_some());
        assert!(p.query.where_clause.is_some());
        assert!(p.query.order_by.is_some());
    }

    #[test]
    fn parse_multiple_select_items() {
        let p = parse("SELECT c.a, c.b AS beta, c.c FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0].alias, None);
                assert_eq!(items[1].alias.as_deref(), Some("beta"));
                assert_eq!(items[2].alias, None);
            }
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_select_with_computation() {
        let p = parse("SELECT c.price * c.qty AS total FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].alias.as_deref(), Some("total"));
                match &items[0].expression {
                    SqlScalarExpression::Binary {
                        op: SqlBinaryOp::Multiply,
                        ..
                    } => {}
                    _ => panic!("expected Multiply"),
                }
            }
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_deeply_nested_members() {
        let p = parse("SELECT c.a.b.c.d.e FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => {
                // Traverse: MemberRef("e") -> MemberRef("d") -> MemberRef("c") -> MemberRef("b") -> MemberRef("a") -> PropertyRef("c")
                let mut expr = &items[0].expression;
                let expected = ["e", "d", "c", "b", "a"];
                for name in &expected {
                    match expr {
                        SqlScalarExpression::MemberRef { source, member } => {
                            assert_eq!(member, name);
                            expr = source.as_ref();
                        }
                        _ => panic!("expected MemberRef for {name}"),
                    }
                }
                match expr {
                    SqlScalarExpression::PropertyRef(root) => assert_eq!(root, "c"),
                    _ => panic!("expected root PropertyRef"),
                }
            }
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_empty_array_literal_in_value() {
        let p = parse("SELECT VALUE [] FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::Value(expr) => match expr.as_ref() {
                SqlScalarExpression::ArrayCreate(items) => assert!(items.is_empty()),
                _ => panic!("expected ArrayCreate"),
            },
            _ => panic!("expected SELECT VALUE"),
        }
    }

    #[test]
    fn parse_empty_object_literal_in_value() {
        let p = parse("SELECT VALUE {} FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::Value(expr) => match expr.as_ref() {
                SqlScalarExpression::ObjectCreate(props) => assert!(props.is_empty()),
                _ => panic!("expected ObjectCreate"),
            },
            _ => panic!("expected SELECT VALUE"),
        }
    }

    #[test]
    fn parse_member_indexer() {
        let p = parse("SELECT c.items[0] FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::MemberIndexer { source, index } => {
                    match source.as_ref() {
                        SqlScalarExpression::MemberRef { member, .. } => {
                            assert_eq!(member, "items");
                        }
                        _ => panic!("expected MemberRef source"),
                    }
                    match index.as_ref() {
                        SqlScalarExpression::Literal(SqlLiteral::Integer(0)) => {}
                        _ => panic!("expected integer 0 index"),
                    }
                }
                _ => panic!("expected MemberIndexer"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_string_member_indexer() {
        let p = parse("SELECT c['name'] FROM c").unwrap();
        match &p.query.select.spec {
            SqlSelectSpec::List(items) => match &items[0].expression {
                SqlScalarExpression::MemberIndexer { source, index } => {
                    match source.as_ref() {
                        SqlScalarExpression::PropertyRef(name) => assert_eq!(name, "c"),
                        _ => panic!("expected PropertyRef source"),
                    }
                    match index.as_ref() {
                        SqlScalarExpression::Literal(SqlLiteral::String(s)) => {
                            assert_eq!(s, "name");
                        }
                        _ => panic!("expected string index"),
                    }
                }
                _ => panic!("expected MemberIndexer"),
            },
            _ => panic!("expected select list"),
        }
    }

    #[test]
    fn parse_group_by_multiple() {
        let p = parse("SELECT c.city, c.state, COUNT(1) FROM c GROUP BY c.city, c.state").unwrap();
        let gb = p.query.group_by.unwrap();
        assert_eq!(gb.expressions.len(), 2);
    }

    // ── Regression tests ────────────────────────────────────────────────

    #[test]
    fn parse_postfix_not_without_in_between_like_errors() {
        // Regression: previously, `WHERE c.x = 5 NOT ORDER BY c.y` was silently
        // rewritten to `WHERE NOT (c.x = 5) ORDER BY c.y`, inverting the user's
        // predicate.
        let result = parse("SELECT * FROM c WHERE c.x = 5 NOT ORDER BY c.y");
        assert!(
            result.is_err(),
            "stray postfix NOT should be a parse error, not a silent NOT-rewrite"
        );
        let msg = result.unwrap_err().message;
        let upper = msg.to_ascii_uppercase();
        assert!(
            upper.contains("NOT")
                && (upper.contains("IN") || upper.contains("BETWEEN") || upper.contains("LIKE")),
            "error message should mention NOT must be followed by IN/BETWEEN/LIKE, got: {msg}"
        );
    }

    #[test]
    fn parse_top_float_literal_is_error() {
        assert!(parse("SELECT TOP 3.7 * FROM c").is_err());
    }

    #[test]
    fn parse_offset_limit_float_literals_are_errors() {
        assert!(parse("SELECT * FROM c OFFSET 1.5 LIMIT 5").is_err());
        assert!(parse("SELECT * FROM c OFFSET 0 LIMIT 5.5").is_err());
    }

    #[test]
    fn deeply_nested_parens_does_not_stack_overflow() {
        // The depth guard (MAX_NESTING_DEPTH) must reject deeply nested parens
        // with a parse error long before the parser thread stack is exhausted.
        // Runs on the test harness's default stack on purpose: production
        // callers do not generally configure 16 MiB stacks, so the guard must
        // be tight enough to be safe under realistic stack budgets.
        let mut sql = String::from("SELECT VALUE ");
        for _ in 0..2000 {
            sql.push('(');
        }
        sql.push('1');
        for _ in 0..2000 {
            sql.push(')');
        }
        sql.push_str(" FROM c");
        let result = parse(&sql);
        assert!(
            result.is_err(),
            "deeply nested parens must be rejected by the depth guard"
        );
        let msg = result.unwrap_err().message;
        assert!(
            msg.to_ascii_lowercase().contains("nesting"),
            "expected nesting-depth error, got: {msg}"
        );
    }

    /// (#6) An unterminated string literal must produce a parse error rather
    /// than silently consuming the remainder of the input as a partial
    /// `StringLiteral` (which the Gateway would have rejected with a 400 but
    /// the local parser used to swallow). The diagnostic must mention the
    /// missing closing quote so authors can locate the typo.
    #[test]
    fn parse_unterminated_string_literal_is_error() {
        let result = parse("SELECT * FROM c WHERE c.x = 'unclosed");
        assert!(
            result.is_err(),
            "unterminated string must produce a parse error"
        );
        let msg = result.unwrap_err().message.to_ascii_lowercase();
        assert!(
            msg.contains("unterminated") && msg.contains("string"),
            "diagnostic must mention an unterminated string literal, got: {msg}"
        );
    }

    /// (#6) Same regression at the very end of the input \u2014 verifies the
    /// deferred check after `parse_program` returns Ok also catches it.
    #[test]
    fn parse_unterminated_string_at_end_of_input_is_error() {
        // The string literal is the last thing on the input; without the
        // deferred lex-error check the parser would happily return Ok.
        assert!(parse("SELECT VALUE 'unclosed").is_err());
    }

    /// (#9) Deep `AND` chains must not stack-overflow either the parser
    /// (recursive descent through Pratt parsing) nor any downstream pass that
    /// walks the resulting AST. Mirrors the existing nested-parens regression
    /// test but covers the binary-operator recursion path used by the local
    /// plan generator's `extract_pk_from_expression` / `intersect_pk_filters`.
    #[test]
    fn deeply_nested_and_chain_does_not_stack_overflow() {
        // 4000 left-deep `AND` clauses is representative of generated queries
        // we have seen in the wild. Both the parser (iterative for AND/OR
        // chains via the precedence-climbing loop) and the plan-generator
        // walks (`visit_expr_for_info`, `extract_*_pk`, `flatten_and`)
        // are explicitly iterative for these cases, so this must run to
        // completion on a default worker thread stack (~2 MiB on Windows).
        let mut sql = String::from("SELECT * FROM c WHERE c.x = 1");
        for i in 0..4000 {
            sql.push_str(&format!(" AND c.x = {i}"));
        }
        // Either the depth guard rejects it, or the parser succeeds and the
        // plan generator processes it without crashing. Both are acceptable
        // -- what we must NOT do is overflow the thread stack.
        match parse(&sql) {
            Ok(program) => {
                let plan = crate::query::plan::generate_query_plan(&program.query, &["/pk"]);
                assert!(plan.is_ok(), "plan generation must not fail for deep AND");
            }
            Err(e) => {
                let msg = e.message.to_ascii_lowercase();
                assert!(
                    msg.contains("nesting") || msg.contains("depth"),
                    "if parsing rejects, the error must come from the depth guard, got: {msg}"
                );
            }
        }
    }

    // (#7) The whitelist in `parse_identifier_name` is the contract for
    // which lexer keywords may also appear as property names (e.g.
    // `c.value`, `c.from`, `c.order`). Adding a new keyword to the lexer
    // without updating that whitelist would silently reject valid Cosmos
    // queries that happen to use the new keyword as a property name. These
    // tests pin the contract so the regression surfaces immediately.
    //
    // If a new keyword is added to the lexer and *deliberately* not allowed
    // as a property name, remove the corresponding case from this list.
    fn assert_keyword_parses_as_property(keyword: &str) {
        let sql = format!("SELECT c.{keyword} FROM c");
        crate::query::parse(&sql).unwrap_or_else(|e| {
            panic!(
                "lexer keyword '{keyword}' must be accepted as a property name; \
                 update parse_identifier_name when adding a new keyword.\n  error: {e}"
            )
        });
    }

    #[test]
    fn keyword_as_property_name_select_value_from_etc() {
        for kw in [
            "value",
            "left",
            "right",
            "let",
            "rank",
            "set",
            "over",
            "for",
            "top",
            "asc",
            "desc",
            "distinct",
            "null",
            "true",
            "false",
            "undefined",
            "array",
            "order",
            "group",
            "offset",
            "limit",
            "select",
            "from",
            "where",
            "by",
            "as",
            "and",
            "or",
            "not",
            "in",
            "between",
            "like",
            "escape",
            "join",
            "cross",
            "inner",
            "exists",
            "is",
            "having",
            "udf",
        ] {
            assert_keyword_parses_as_property(kw);
        }
    }

    #[test]
    fn keyword_as_nested_property_name() {
        // The whitelist must also work in nested member positions (`a.b.c`).
        crate::query::parse("SELECT c.address.from FROM c")
            .expect("nested keyword property must parse");
        crate::query::parse("SELECT c.order.value FROM c")
            .expect("chained keyword properties must parse");
    }
}
