// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(test)]
pub(crate) mod tests {
    use proc_macro2::{TokenStream, TokenTree};

    // cspell: ignore punct

    pub(crate) fn compare_token_tree(token: &TokenTree, expected_token: &TokenTree) -> bool {
        //        println!("Comparing token: {token:?} with expected token: {expected_token:?}");
        match token {
            TokenTree::Group(group) => match expected_token {
                TokenTree::Group(expected_group) => {
                    compare_token_stream(group.stream(), expected_group.stream())
                }
                _ => {
                    println!("Unexpected token: {expected_token:?}");
                    false
                }
            },
            TokenTree::Ident(ident) => match expected_token {
                TokenTree::Ident(expected_ident) => *expected_ident == *ident,
                _ => {
                    println!("Unexpected token: {expected_token:?}");
                    false
                }
            },
            TokenTree::Punct(punct) => match expected_token {
                TokenTree::Punct(expected_punct) => punct.as_char() == expected_punct.as_char(),
                _ => {
                    println!("Unexpected token: {expected_token:?}");
                    false
                }
            },
            TokenTree::Literal(literal) => match expected_token {
                TokenTree::Literal(expected_literal) => {
                    literal.to_string() == expected_literal.to_string()
                }
                _ => {
                    println!("Unexpected token: {expected_token:?}");
                    false
                }
            },
        }
    }

    pub(crate) fn compare_token_stream(actual: TokenStream, expected: TokenStream) -> bool {
        let actual_tokens = Vec::from_iter(actual);
        let expected_tokens = Vec::from_iter(expected);

        if actual_tokens.len() != expected_tokens.len() {
            println!(
                "Token lengths do not match: actual: {} != expected: {}",
                actual_tokens.len(),
                expected_tokens.len()
            );
            for (i, actual) in actual_tokens.iter().enumerate() {
                println!("Actual token at index {i}: {actual:?}");
            }

            for (i, expected) in expected_tokens.iter().enumerate() {
                println!("Expected token at index {i}: {expected:?}");
            }
            return false;
        }

        for (actual, expected) in actual_tokens.iter().zip(expected_tokens.iter()) {
            let equal = compare_token_tree(actual, expected);
            if !equal {
                println!("Tokens do not match: {actual:?} != {expected:?}");
                return false;
            }
        }
        true
    }
}
