use std::io::{self, BufRead, BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;
use tracing::{error, info};

///The port where the local server is listening on the auth_code
#[allow(dead_code)]
pub const LOCAL_SERVER_PORT: u16 = 47828;

/// Opens the given URL in the default system browser and starts a local web server
/// to receive the authorization code.
#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub async fn open_url(url: &str) -> Option<String> {
    use async_process::Command;
    let spawned = Command::new("cmd").args(["/C", "explorer", url]).spawn();
    handle_browser_command(spawned)
}

/// Opens the given URL in the default system browser and starts a local web server
/// to receive the authorization code.
#[allow(dead_code)]
#[cfg(target_os = "macos")]
pub async fn open_url(url: &str) -> Option<String> {
    use async_process::Command;
    let spawned = Command::new("open").arg(url).spawn();
    handle_browser_command(spawned)
}

/// Opens the given URL in the default system browser and starts a local web server
/// to receive the authorization code.
#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub async fn open_url(url: &str) -> Option<String> {
    use async_process::Command;

    if let Some(command) = find_linux_browser_command().await {
        let spawned = Command::new(command).arg(url).spawn();
        return handle_browser_command(spawned);
    }

    info!("Open the following link manually in your browser: {url}");
    None
}

/// Method to check if the command to open the link in a browser is available on the computer
/// exists.
#[allow(dead_code)]
#[cfg(target_os = "linux")]
async fn is_command_available(cmd: &str) -> bool {
    use async_process::Command;
    Command::new("which")
        .arg(cmd)
        .output()
        .await
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false)
}

/// Method with all the commands which could open the browser to call the authorization url
/// If there is no command installed or available on the system, it returns a 'None' and the link
/// will be logged
#[allow(dead_code)]
#[cfg(target_os = "linux")]
async fn find_linux_browser_command() -> Option<String> {
    let candidates = [
        "xdg-open",
        "gnome-open",
        "kfmclient",
        "microsoft-edge",
        "wslview",
    ];
    for cmd in candidates.iter() {
        if is_command_available(cmd).await {
            return Some(cmd.to_string());
        }
    }
    None
}

/// starting the browser if the browser could be started, then the webserver should be started to
/// get the auth code
#[allow(dead_code)]
fn handle_browser_command(result: Result<async_process::Child, io::Error>) -> Option<String> {
    match result {
        Ok(_) => start_webserver(),
        Err(e) => {
            error!("Failed to start browser command: {e}");
            None
        }
    }
}

/// Starts the webserver on the `http://localhost`. Returns None, if the server could not have
/// started
#[allow(dead_code)]
/// Starts a simple HTTP server on localhost to receive the auth code.
fn start_webserver() -> Option<String> {
    TcpListener::bind(("127.0.0.1", LOCAL_SERVER_PORT))
        .ok()
        .and_then(handle_tcp_connection)
}

fn handle_tcp_connection(listener: TcpListener) -> Option<String> {
    listener
        .incoming()
        .take(1)
        .next()?
        .ok()
        .and_then(handle_client)
}
/// Main method to handle the incomming traffic.
/// After a 10s timeout the stream will be closed
/// if the stream could be opened, we read the whole request and try to extract the auth_code
/// Returns also the html code to show if it worked
#[allow(dead_code)]
fn handle_client(mut stream: TcpStream) -> Option<String> {
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .ok()?;

    let buf_reader = BufReader::new(&stream);
    let mut request_lines = vec![];
    for line in buf_reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        request_lines.push(line);
    }

    let request = request_lines.join("\n");

    let auth_code = extract_auth_code(&request);
    let response_body = r#"<!DOCTYPE html>
<html><head><title>Auth Complete</title></head>
<body><p>Authentication complete. You may close this tab.</p></body>
</html>"#;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    stream.write_all(response.as_bytes()).ok()?;
    stream.flush().ok()?;
    stream.shutdown(Shutdown::Both).ok()?;

    auth_code
}

/// Extracts the `code` query parameter from the request.
#[allow(dead_code)]
fn extract_auth_code(request: &str) -> Option<String> {
    let code_start = request.rfind("code=")? + 5;
    let rest = &request[code_start..];
    let end = rest.find('&').unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

#[cfg(test)]
mod test_internal_server {
    use super::*;
    use tracing::debug;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;
    fn init_logger() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber);
    }

    #[tokio::test]
    async fn test_valid_command() {
        init_logger();
        assert!(is_command_available("ls").await);
    }

    #[tokio::test]
    async fn test_invalid_command() {
        init_logger();
        assert!(!is_command_available("non_existing_command_foo").await);
    }

    #[test]
    fn test_extract_code_param() {
        let url = "GET /?code=abc123&state=xyz";
        assert_eq!(extract_auth_code(url).unwrap(), "abc123");
    }

    #[test]
    fn test_extract_code_at_end() {
        let url = "GET /?state=xyz&code=abc123";
        assert_eq!(extract_auth_code(url).unwrap(), "abc123");
    }

    #[test]
    fn test_extract_code_missing() {
        let url = "GET /?state=only";
        assert!(extract_auth_code(url).is_none());
    }
}
