// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recursive descent parser for the Cosmos DB SQL dialect.
//!
//! Produces an [`SqlProgram`] AST from SQL text. Uses Pratt parsing
//! for operator precedence in scalar expressions.

use crate::ast::*;
use crate::lexer::{
    extract_identifier, extract_parameter_name, extract_string_content, Lexer, Span, Token,
    TokenKind,
};

/// Parse error with location information.
#[derive(Debug, Clone, thiserror::Error)]
#[error("{message} at offset {}", span.start)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

/// Parse a SQL string into an AST.
///
/// # Examples
/// ```
/// let program = azure_data_cosmos_query::parse("SELECT * FROM c WHERE c.id = '1'").unwrap();
/// assert!(program.query.where_clause.is_some());
/// ```
pub fn parse(sql: &str) -> Result<SqlProgram, ParseError> {
    let mut parser = Parser::new(sql);
    parser.parse_program()
}

const MAX_NESTING_DEPTH: usize = 128;

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
            TokenKind::FloatLiteral => {
                let n = self
                    .current
                    .text
                    .parse::<f64>()
                    .map_err(|_| self.error("invalid TOP value".into()))?;
                self.advance();
                Ok(Some(SqlTopSpec::Literal(n as i64)))
            }
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
        self.parse_ternary()
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
                    // Wasn't a NOT IN/BETWEEN/LIKE, so this is an error in this context
                    // because we already consumed NOT. Wrap it as a NOT unary.
                    return Ok(SqlScalarExpression::Unary {
                        op: SqlUnaryOp::Not,
                        operand: Box::new(expr),
                    });
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
                self.advance();
                if self.at(TokenKind::LParen) {
                    self.advance();
                    let query = self.parse_query()?;
                    self.expect(TokenKind::RParen)?;
                    SqlScalarExpression::Array(Box::new(query))
                } else {
                    // Could be just the identifier "ARRAY" used as a property
                    SqlScalarExpression::PropertyRef("ARRAY".to_string())
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
                self.advance();
                // Check if this is a subquery (starts with SELECT)
                if self.at(TokenKind::Select) {
                    let query = self.parse_query()?;
                    self.expect(TokenKind::RParen)?;
                    SqlScalarExpression::Subquery(Box::new(query))
                } else {
                    let expr = self.parse_scalar_expression()?;
                    self.expect(TokenKind::RParen)?;
                    expr
                }
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
                    // Use keyword text as identifier name
                    self.current.text.to_ascii_uppercase()
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
        matches!(p.query.select.spec, SqlSelectSpec::Value(_));
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
        matches!(p.query.select.spec, SqlSelectSpec::List(_));
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
        matches!(w.expression, SqlScalarExpression::Between { .. });
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
        matches!(from.collection, SqlCollectionExpression::Join { .. });
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
}
