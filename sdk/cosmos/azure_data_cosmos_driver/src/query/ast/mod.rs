// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SQL Abstract Syntax Tree types for the Cosmos DB SQL dialect.

use std::fmt;

/// Top-level parsed SQL program.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlProgram {
    pub query: SqlQuery,
}

/// A complete SQL query:
/// `SELECT ... FROM ... WHERE ... GROUP BY ... ORDER BY ... OFFSET ... LIMIT`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlQuery {
    pub(crate) select: SqlSelectClause,
    pub(crate) from: Option<SqlFromClause>,
    pub(crate) where_clause: Option<SqlWhereClause>,
    pub(crate) group_by: Option<SqlGroupByClause>,
    pub(crate) order_by: Option<SqlOrderByClause>,
    pub(crate) offset_limit: Option<SqlOffsetLimitClause>,
}

/// The SELECT clause: `SELECT [DISTINCT] [TOP n] <spec>`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlSelectClause {
    pub(crate) distinct: bool,
    pub(crate) top: Option<SqlTopSpec>,
    pub(crate) spec: SqlSelectSpec,
}

/// What the SELECT clause selects.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct SqlSelectItem {
    pub(crate) expression: SqlScalarExpression,
    pub(crate) alias: Option<String>,
}

/// `TOP n`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SqlTopSpec {
    Literal(i64),
    Parameter(String),
}

/// `FROM <collection_expression>`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlFromClause {
    pub(crate) collection: SqlCollectionExpression,
}

/// `WHERE <expression>`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlWhereClause {
    pub(crate) expression: SqlScalarExpression,
}

/// `GROUP BY expr1, expr2, ...`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlGroupByClause {
    pub(crate) expressions: Vec<SqlScalarExpression>,
}

/// `ORDER BY item1, item2, ...`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlOrderByClause {
    pub(crate) items: Vec<SqlOrderByItem>,
}

/// A single ORDER BY item: `expr [ASC|DESC]`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlOrderByItem {
    pub(crate) expression: SqlScalarExpression,
    pub(crate) order: SqlSortOrder,
}

/// Sort order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SqlSortOrder {
    Unspecified,
    Ascending,
    Descending,
}

/// `OFFSET n LIMIT m`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SqlOffsetLimitClause {
    pub(crate) offset: SqlOffsetSpec,
    pub(crate) limit: SqlLimitSpec,
}

/// `OFFSET n` or `OFFSET @param`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SqlOffsetSpec {
    Literal(i64),
    Parameter(String),
}

/// `LIMIT m` or `LIMIT @param`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SqlLimitSpec {
    Literal(i64),
    Parameter(String),
}

/// Collection expressions used in FROM clauses.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
pub struct SqlObjectProperty {
    pub(crate) name: String,
    pub(crate) expression: SqlScalarExpression,
}

/// Literal values.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
