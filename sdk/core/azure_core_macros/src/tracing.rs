// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(test)]
pub(crate) mod tests {
    use ::tracing::{error, trace};
    use proc_macro2::{TokenStream, TokenTree};
    static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

    pub(crate) fn setup_tracing() {
        INIT_LOGGING.call_once(|| {
            println!("Setting up test logger...");

            use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
            tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_ansi(std::env::var("NO_COLOR").map_or(true, |v| v.is_empty()))
                .with_writer(std::io::stderr)
                .init();
        });
    }

    // cspell: ignore punct

    pub(crate) fn compare_token_tree(token: &TokenTree, expected_token: &TokenTree) -> bool {
        match (token, expected_token) {
            (TokenTree::Group(group), TokenTree::Group(expected_group)) => {
                compare_token_stream(group.stream(), expected_group.stream())
            }

            (TokenTree::Ident(ident), TokenTree::Ident(expected_ident)) => {
                *expected_ident == *ident
            }
            (TokenTree::Punct(punct), TokenTree::Punct(expected_punct)) => {
                punct.as_char() == expected_punct.as_char()
            }
            (TokenTree::Literal(literal), TokenTree::Literal(expected_literal)) => {
                literal.to_string() == expected_literal.to_string()
            }
            _ => {
                error!("Unexpected token: {expected_token:?}");
                false
            }
        }
    }

    pub(crate) fn compare_token_stream(actual: TokenStream, expected: TokenStream) -> bool {
        let actual_tokens = Vec::from_iter(actual);
        let expected_tokens = Vec::from_iter(expected);

        if actual_tokens.len() != expected_tokens.len() {
            error!(
                "Token lengths do not match: actual: {} != expected: {}",
                actual_tokens.len(),
                expected_tokens.len()
            );
            for (i, actual) in actual_tokens.iter().enumerate() {
                trace!("Actual token at index {i}: {actual:?}");
            }

            for (i, expected) in expected_tokens.iter().enumerate() {
                trace!("Expected token at index {i}: {expected:?}");
            }
            return false;
        }

        for (actual, expected) in actual_tokens.iter().zip(expected_tokens.iter()) {
            let equal = compare_token_tree(actual, expected);
            if !equal {
                error!("Tokens do not match: {actual:?} != {expected:?}");
                return false;
            }
        }
        true
    }
}
