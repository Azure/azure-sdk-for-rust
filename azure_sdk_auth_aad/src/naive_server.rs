use crate::errors::ServerReceiveError;
use crate::AuthObj;
use oauth2::{AuthorizationCode, CsrfToken};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use url::Url;

pub fn naive_server(auth_obj: &AuthObj, port: u32) -> Result<AuthorizationCode, ServerReceiveError> {
    // A very naive implementation of the redirect server.
    // A ripoff of https://github.com/ramosbugs/oauth2-rs/blob/master/examples/msgraph.rs, stripped
    // down for simplicity.
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = match request_line.split_whitespace().nth(1) {
                    Some(redirect_url) => redirect_url,
                    None => return Err(ServerReceiveError::UnexpectedRedirectUrl { url: request_line }),
                };
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                println!("url == {}", url);

                let code = match url.query_pairs().find(|pair| {
                    let &(ref key, _) = pair;
                    key == "code"
                }) {
                    Some(qp) => AuthorizationCode::new(qp.1.into_owned()),
                    None => {
                        return Err(ServerReceiveError::QueryPairNotFound {
                            query_pair: "code".to_owned(),
                        })
                    }
                };

                let state = match url.query_pairs().find(|pair| {
                    let &(ref key, _) = pair;
                    key == "state"
                }) {
                    Some(qp) => CsrfToken::new(qp.1.into_owned()),
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
                let response = format!("HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}", message.len(), message);
                stream.write_all(response.as_bytes()).unwrap();

                // The server will terminate itself after collecting the first code.
                return Ok(code);
            }
        }
    }

    unreachable!()
}
