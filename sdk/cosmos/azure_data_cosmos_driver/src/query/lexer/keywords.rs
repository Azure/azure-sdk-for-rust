// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Length-bucketed case-insensitive keyword lookup. Split out of lexer/mod.rs
//! (#17) so the table doesn't dwarf the scanner code.

use super::TokenKind;
pub(super) fn keyword_lookup(text: &str) -> TokenKind {
    // Short-circuit on length for the most common keywords
    match text.len() {
        2 => {
            if text.eq_ignore_ascii_case("AS") {
                return TokenKind::As;
            }
            if text.eq_ignore_ascii_case("BY") {
                return TokenKind::By;
            }
            if text.eq_ignore_ascii_case("IN") {
                return TokenKind::In;
            }
            if text.eq_ignore_ascii_case("IS") {
                return TokenKind::Is;
            }
            if text.eq_ignore_ascii_case("OR") {
                return TokenKind::Or;
            }
        }
        3 => {
            if text.eq_ignore_ascii_case("AND") {
                return TokenKind::And;
            }
            if text.eq_ignore_ascii_case("ASC") {
                return TokenKind::Asc;
            }
            if text.eq_ignore_ascii_case("FOR") {
                return TokenKind::For;
            }
            if text.eq_ignore_ascii_case("LET") {
                return TokenKind::Let;
            }
            if text.eq_ignore_ascii_case("NOT") {
                return TokenKind::Not;
            }
            if text.eq_ignore_ascii_case("SET") {
                return TokenKind::Set;
            }
            if text.eq_ignore_ascii_case("TOP") {
                return TokenKind::Top;
            }
            if text.eq_ignore_ascii_case("UDF") {
                return TokenKind::Udf;
            }
        }
        4 => {
            if text.eq_ignore_ascii_case("DESC") {
                return TokenKind::Desc;
            }
            if text.eq_ignore_ascii_case("FROM") {
                return TokenKind::From;
            }
            if text.eq_ignore_ascii_case("JOIN") {
                return TokenKind::Join;
            }
            if text.eq_ignore_ascii_case("LEFT") {
                return TokenKind::Left;
            }
            if text.eq_ignore_ascii_case("LIKE") {
                return TokenKind::Like;
            }
            if text.eq_ignore_ascii_case("NULL") {
                return TokenKind::Null;
            }
            if text.eq_ignore_ascii_case("OVER") {
                return TokenKind::Over;
            }
            if text.eq_ignore_ascii_case("RANK") {
                return TokenKind::Rank;
            }
            if text.eq_ignore_ascii_case("TRUE") {
                return TokenKind::True;
            }
        }
        5 => {
            if text.eq_ignore_ascii_case("ARRAY") {
                return TokenKind::Array;
            }
            if text.eq_ignore_ascii_case("CROSS") {
                return TokenKind::Cross;
            }
            if text.eq_ignore_ascii_case("FALSE") {
                return TokenKind::False;
            }
            if text.eq_ignore_ascii_case("GROUP") {
                return TokenKind::Group;
            }
            if text.eq_ignore_ascii_case("INNER") {
                return TokenKind::Inner;
            }
            if text.eq_ignore_ascii_case("LIMIT") {
                return TokenKind::Limit;
            }
            if text.eq_ignore_ascii_case("ORDER") {
                return TokenKind::Order;
            }
            if text.eq_ignore_ascii_case("RIGHT") {
                return TokenKind::Right;
            }
            if text.eq_ignore_ascii_case("VALUE") {
                return TokenKind::Value;
            }
            if text.eq_ignore_ascii_case("WHERE") {
                return TokenKind::Where;
            }
        }
        6 => {
            if text.eq_ignore_ascii_case("ESCAPE") {
                return TokenKind::Escape;
            }
            if text.eq_ignore_ascii_case("EXISTS") {
                return TokenKind::Exists;
            }
            if text.eq_ignore_ascii_case("HAVING") {
                return TokenKind::Having;
            }
            if text.eq_ignore_ascii_case("OFFSET") {
                return TokenKind::Offset;
            }
            if text.eq_ignore_ascii_case("SELECT") {
                return TokenKind::Select;
            }
        }
        7 if text.eq_ignore_ascii_case("BETWEEN") => {
            return TokenKind::Between;
        }
        8 if text.eq_ignore_ascii_case("DISTINCT") => {
            return TokenKind::Distinct;
        }
        9 if text.eq_ignore_ascii_case("UNDEFINED") => {
            return TokenKind::Undefined;
        }
        _ => {}
    }
    TokenKind::Identifier
}
