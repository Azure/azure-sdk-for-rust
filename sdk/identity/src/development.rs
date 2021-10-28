//! Utilities for aiding in development
//!
//! These utilities should not be used in production
use crate::authorization_code_flow::AuthorizationCodeFlow;
use log::debug;
use oauth2::{AuthorizationCode, CsrfToken};
use url::Url;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

/// A very naive implementation of a redirect server.
///
/// A ripoff of https://github.com/ramosbugs/oauth2-rs/blob/master/examples/msgraph.rs, stripped
/// down for simplicity. This server blocks until redirected to.
///
/// This implementation should only be used for testing.
pub fn naive_redirect_server(
    auth_obj: &AuthorizationCodeFlow,
    port: u32,
) -> Result<AuthorizationCode, ServerReceiveError> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    // The server will terminate itself after collecting the first code.
    if let Some(mut stream) = listener.incoming().flatten().next() {
        let mut reader = BufReader::new(&stream);

        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let redirect_url = match request_line.split_whitespace().nth(1) {
            Some(redirect_url) => redirect_url,
            None => return Err(ServerReceiveError::UnexpectedRedirectUrl { url: request_line }),
        };
        let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

        debug!("url == {}", url);

        let code = match url.query_pairs().find(|(key, _)| key == "code") {
            Some((_, value)) => AuthorizationCode::new(value.into_owned()),
            None => {
                return Err(ServerReceiveError::QueryPairNotFound {
                    query_pair: "code".to_owned(),
                })
            }
        };

        let state = match url.query_pairs().find(|(key, _)| key == "state") {
            Some((_, value)) => CsrfToken::new(value.into_owned()),
            None => {
                return Err(ServerReceiveError::QueryPairNotFound {
                    query_pair: "state".to_owned(),
                })
            }
        };

        if state.secret() != auth_obj.csrf_state.secret() {
            return Err(ServerReceiveError::StateSecretMismatch {
                expected_state_secret: auth_obj.csrf_state.secret().to_owned(),
                received_state_secret: state.secret().to_owned(),
            });
        }

        let message = "Authentication complete. You can close this window now.";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes()).unwrap();

        return Ok(code);
    }

    unreachable!()
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ServerReceiveError {
    #[error("unexpected redirect url: {}", url)]
    UnexpectedRedirectUrl { url: String },
    #[error("query pair not found: {}", query_pair)]
    QueryPairNotFound { query_pair: String },
    #[error(
        "State secret mismatch: expected {}, recieved: {}",
        expected_state_secret,
        received_state_secret
    )]
    StateSecretMismatch {
        expected_state_secret: String,
        received_state_secret: String,
    },
}
