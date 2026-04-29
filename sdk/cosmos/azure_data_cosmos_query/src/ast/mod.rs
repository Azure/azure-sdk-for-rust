// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SQL Abstract Syntax Tree types for the Cosmos DB SQL dialect.

use std::fmt;

/// Top-level parsed SQL program.
#[derive(Debug, Clone, PartialEq)]
pub struct SqlProgram {
    pub query: SqlQuery,
}

/// A complete SQL query:
/// `SELECT ... FROM ... WHERE ... GROUP BY ... ORDER BY ... OFFSET ... LIMIT`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlQuery {
    pub select: SqlSelectClause,
    pub from: Option<SqlFromClause>,
    pub where_clause: Option<SqlWhereClause>,
    pub group_by: Option<SqlGroupByClause>,
    pub order_by: Option<SqlOrderByClause>,
    pub offset_limit: Option<SqlOffsetLimitClause>,
}

/// The SELECT clause: `SELECT [DISTINCT] [TOP n] <spec>`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlSelectClause {
    pub distinct: bool,
    pub top: Option<SqlTopSpec>,
    pub spec: SqlSelectSpec,
}

/// What the SELECT clause selects.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlSelectSpec {
    /// `SELECT *`
    Star,
    /// `SELECT expr1 [AS alias1], expr2 [AS alias2], ...`
    List(Vec<SqlSelectItem>),
    /// `SELECT VALUE expr`
    Value(Box<SqlScalarExpression>),
}

/// A single item in a SELECT list: `expr [AS alias]`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlSelectItem {
    pub expression: SqlScalarExpression,
    pub alias: Option<String>,
}

/// `TOP n`
#[derive(Debug, Clone, PartialEq)]
pub enum SqlTopSpec {
    Literal(i64),
    Parameter(String),
}

/// `FROM <collection_expression>`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlFromClause {
    pub collection: SqlCollectionExpression,
}

/// `WHERE <expression>`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlWhereClause {
    pub expression: SqlScalarExpression,
}

/// `GROUP BY expr1, expr2, ...`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlGroupByClause {
    pub expressions: Vec<SqlScalarExpression>,
}

/// `ORDER BY item1, item2, ...`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlOrderByClause {
    pub items: Vec<SqlOrderByItem>,
}

/// A single ORDER BY item: `expr [ASC|DESC]`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlOrderByItem {
    pub expression: SqlScalarExpression,
    pub order: SqlSortOrder,
}

/// Sort order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlSortOrder {
    Unspecified,
    Ascending,
    Descending,
}

/// `OFFSET n LIMIT m`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlOffsetLimitClause {
    pub offset: SqlOffsetSpec,
    pub limit: SqlLimitSpec,
}

/// `OFFSET n` or `OFFSET @param`
#[derive(Debug, Clone, PartialEq)]
pub enum SqlOffsetSpec {
    Literal(i64),
    Parameter(String),
}

/// `LIMIT m` or `LIMIT @param`
#[derive(Debug, Clone, PartialEq)]
pub enum SqlLimitSpec {
    Literal(i64),
    Parameter(String),
}

/// Collection expressions used in FROM clauses.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlCollectionExpression {
    /// `<collection> [AS <alias>]` or `<collection> <alias>`
    Aliased {
        collection: SqlCollection,
        alias: Option<String>,
    },
    /// `<id> IN <collection>` — array iteration
    ArrayIterator {
        identifier: String,
        collection: SqlCollection,
    },
    /// `<left> JOIN <right>`
    Join {
        left: Box<SqlCollectionExpression>,
        right: Box<SqlCollectionExpression>,
    },
}

/// A collection source: either a path or a subquery.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlCollection {
    /// `<root>[.<path>]`
    Path {
        root: String,
        path: Vec<SqlPathSegment>,
    },
    /// `(<subquery>)`
    Subquery(Box<SqlQuery>),
}

/// A segment of a property path.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlPathSegment {
    /// `.identifier`
    Identifier(String),
    /// `[number]`
    Index(i64),
    /// `["string"]`
    StringIndex(String),
}

/// All scalar expression variants — the core of the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlScalarExpression {
    /// A literal value: `42`, `'hello'`, `true`, `null`, `undefined`
    Literal(SqlLiteral),
    /// A property reference: `c`, `id`, etc.
    PropertyRef(String),
    /// Member access: `source.member`
    MemberRef {
        source: Box<SqlScalarExpression>,
        member: String,
    },
    /// Indexer access: `source[index]`
    MemberIndexer {
        source: Box<SqlScalarExpression>,
        index: Box<SqlScalarExpression>,
    },
    /// Binary expression: `left op right`
    Binary {
        op: SqlBinaryOp,
        left: Box<SqlScalarExpression>,
        right: Box<SqlScalarExpression>,
    },
    /// Unary expression: `op operand`
    Unary {
        op: SqlUnaryOp,
        operand: Box<SqlScalarExpression>,
    },
    /// Function call: `name(args...)` or `udf.name(args...)`
    FunctionCall {
        name: String,
        args: Vec<SqlScalarExpression>,
        is_udf: bool,
    },
    /// `expr [NOT] BETWEEN low AND high`
    Between {
        expression: Box<SqlScalarExpression>,
        low: Box<SqlScalarExpression>,
        high: Box<SqlScalarExpression>,
        not: bool,
    },
    /// `expr [NOT] IN (item1, item2, ...)`
    In {
        expression: Box<SqlScalarExpression>,
        items: Vec<SqlScalarExpression>,
        not: bool,
    },
    /// `expr [NOT] LIKE pattern [ESCAPE escape_char]`
    Like {
        expression: Box<SqlScalarExpression>,
        pattern: Box<SqlScalarExpression>,
        escape: Option<String>,
        not: bool,
    },
    /// `condition ? if_true : if_false`
    Conditional {
        condition: Box<SqlScalarExpression>,
        if_true: Box<SqlScalarExpression>,
        if_false: Box<SqlScalarExpression>,
    },
    /// `left ?? right`
    Coalesce {
        left: Box<SqlScalarExpression>,
        right: Box<SqlScalarExpression>,
    },
    /// `EXISTS(<subquery>)`
    Exists(Box<SqlQuery>),
    /// Scalar subquery: `(<subquery>)` in scalar context
    Subquery(Box<SqlQuery>),
    /// `ARRAY(<subquery>)`
    Array(Box<SqlQuery>),
    /// `[expr1, expr2, ...]`
    ArrayCreate(Vec<SqlScalarExpression>),
    /// `{prop1: expr1, prop2: expr2, ...}`
    ObjectCreate(Vec<SqlObjectProperty>),
    /// `@parameter_name`
    ParameterRef(String),
    /// `expr IS NULL` / `expr IS NOT NULL` (parsed from IS expressions)
    IsNull {
        expression: Box<SqlScalarExpression>,
        not: bool,
    },
}

/// A property in an object literal: `name: expression`
#[derive(Debug, Clone, PartialEq)]
pub struct SqlObjectProperty {
    pub name: String,
    pub expression: SqlScalarExpression,
}

/// Literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlLiteral {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Null,
    Undefined,
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    ZeroFillRightShift,
    StringConcat,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlUnaryOp {
    Not,
    Minus,
    Plus,
    BitwiseNot,
}

impl fmt::Display for SqlBinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::Equal => "=",
            Self::NotEqual => "!=",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThanOrEqual => ">=",
            Self::And => "AND",
            Self::Or => "OR",
            Self::BitwiseAnd => "&",
            Self::BitwiseOr => "|",
            Self::BitwiseXor => "^",
            Self::LeftShift => "<<",
            Self::RightShift => ">>",
            Self::ZeroFillRightShift => ">>>",
            Self::StringConcat => "||",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for SqlUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Not => "NOT",
            Self::Minus => "-",
            Self::Plus => "+",
            Self::BitwiseNot => "~",
        };
        write!(f, "{s}")
    }
}
