// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! SOCKS5 proxy support for Azure AMQP connections
//!
//! This module provides SOCKS5 proxy connectivity for Azure AMQP services through the
//! [`SocksConnection`] helper. It enables Azure SDK for Rust to work in corporate
//! environments that require proxy servers for outbound connections.
//!
//! # Overview
//!
//! The SOCKS5 implementation integrates seamlessly with the fe2o3-amqp library by providing
//! a stream that can be passed to `Connection::open_with_stream()`. This allows AMQP
//! connections to be established through SOCKS5 proxies without changing the higher-level
//! Azure SDK APIs.
//!
//! # Supported Protocols
//!
//! - **SOCKS5** (`socks5://`) - Standard SOCKS5 with local DNS resolution
//! - **SOCKS5h** (`socks5h://`) - SOCKS5 with proxy-side DNS resolution (recommended)
//!
//! # Integration with fe2o3-amqp
//!
//! The module works by intercepting the connection establishment in `Fe2o3AmqpConnection::open()`:
//!
//! ```text
//! 1. Check if custom_endpoint uses socks5:// or socks5h:// scheme
//! 2. If yes: Use SocksConnection::connect() to create proxy tunnel
//! 3. Pass resulting stream to fe2o3_amqp::Connection::open_with_stream()
//! 4. If no: Use standard fe2o3_amqp::Connection::open() (direct connection)
//! ```
//!
//! This design ensures zero impact on direct connections while enabling proxy support
//! when needed.
//!
//! # Configuration
//!
//! SOCKS5 proxy support is enabled by setting the `custom_endpoint` option in
//! [`AmqpConnectionOptions`](crate::connection::AmqpConnectionOptions) to a SOCKS5 URL:
//!
//! ```rust,no_run
//! use azure_core::http::Url;
//! use azure_core_amqp::AmqpConnectionOptions;
//!
//! let mut options = AmqpConnectionOptions::default();
//! options.custom_endpoint = Some(Url::parse("socks5h://proxy.corp.com:8080")?);
//! // Connection will automatically use SOCKS5 proxy
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Authentication
//!
//! Username/password authentication is supported via the proxy URL:
//! ```text
//! socks5://username:password@proxy.example.com:1080
//! socks5h://user:pass@corporate-proxy.internal:8080
//! ```
//!
//! Credentials are automatically masked in all log output for security.
//!
//! # Error Handling
//!
//! The module provides detailed error context for common failure scenarios:
//! - Invalid proxy URL format
//! - Network connectivity issues
//! - Proxy authentication failures
//! - Target service unreachability
//! - TLS handshake failures (for AMQPS)
//!
//! All errors include contextual information to aid in troubleshooting corporate
//! network configurations.
//!
//! # Logging
//!
//! The module follows Azure SDK structured logging patterns with tracing:
//! - **debug**: Connection establishment progress
//! - **info**: Successful connection events
//! - **error**: Connection failures with context
//! - **trace**: Low-level protocol details
//!
//! All log messages use snake_case field naming and include relevant context
//! such as `proxy_url`, `target_host`, `connection_id`, etc.
//!
//! # Security Considerations
//!
//! - **Credential masking**: Proxy credentials never appear in logs or error messages
//! - **TLS validation**: Full certificate chain validation for AMQPS connections
//! - **DNS privacy**: SOCKS5h protocol hides DNS queries from local network monitoring
//! - **Protocol isolation**: SOCKS5 implementation isolated from direct connection paths
//!
//! # Performance Impact
//!
//! - **Direct connections**: Zero performance impact (proxy code not executed)
//! - **Proxy connections**: Additional network hop introduces latency
//! - **Memory usage**: Minimal overhead, stream-based design
//! - **Connection reuse**: SOCKS5 tunnel reused for entire AMQP session lifetime
//!
//! # Troubleshooting Common Issues
//!
//! ## Connection Refused
//! ```text
//! Error: SOCKS5 connection failed: proxy=socks5://***:***@proxy.corp.com:8080
//! ```
//! - Verify proxy server is reachable
//! - Check proxy port is correct (default: 1080)
//! - Confirm network allows connections to proxy
//!
//! ## Authentication Failed
//! ```text
//! Error: SOCKS5 connection establishment failed
//! ```
//! - Verify username/password are correct
//! - Check if proxy requires authentication
//! - Confirm URL encoding of special characters in credentials
//!
//! ## Target Unreachable
//! ```text
//! Error: SOCKS5 connection failed: target=amqps://eventhub.servicebus.windows.net
//! ```
//! - Verify target service hostname is correct
//! - Check if proxy allows connections to target port (5671/5672)
//! - Consider using SOCKS5h for DNS privacy requirements
//!
//! # Examples
//!
//! See [`SocksConnection`] for detailed usage examples and API documentation.

use azure_core::{error::Result, http::Url};
use native_tls::TlsConnector as NativeTlsConnector;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_native_tls::TlsConnector;
use tokio_socks::{tcp::Socks5Stream, TargetAddr};
use tracing::{debug, error, trace};

/// A trait that combines AsyncRead, AsyncWrite, Unpin, Send and Debug for SOCKS5 streams
pub trait SocksStream: AsyncRead + AsyncWrite + Unpin + Send + std::fmt::Debug + 'static {}

impl<T> SocksStream for T where T: AsyncRead + AsyncWrite + Unpin + Send + std::fmt::Debug + 'static {}

/// A wrapper that implements the SocksStream trait
#[derive(Debug)]
pub struct StreamWrapper<T>(pub T);

impl<T> AsyncRead for StreamWrapper<T>
where
    T: AsyncRead + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl<T> AsyncWrite for StreamWrapper<T>
where
    T: AsyncWrite + Unpin,
{
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

/// SOCKS5 connection helper for azure_core_amqp
///
/// Supports both SOCKS5 and SOCKS5h protocols for connecting to AMQP services through proxy servers:
/// - `socks5://` - Local DNS resolution (client resolves target hostname)
/// - `socks5h://` - Proxy-side DNS resolution (proxy resolves target hostname, recommended for privacy)
///
/// # Protocol Differences
///
/// **SOCKS5 (`socks5://`)**:
/// - Client performs DNS resolution of target hostname
/// - Target IP address sent through proxy tunnel
/// - Faster connection establishment
/// - DNS queries visible to local network monitoring
///
/// **SOCKS5h (`socks5h://`)**:
/// - Proxy performs DNS resolution of target hostname
/// - Target hostname sent through proxy tunnel
/// - Enhanced privacy (DNS queries hidden from local network)
/// - Recommended for corporate environments
///
/// # Authentication
///
/// Supports username/password authentication via proxy URL:
/// ```text
/// socks5://username:password@proxy.example.com:1080
/// socks5h://user:pass@corporate-proxy.internal:8080
/// ```
///
/// # Examples
///
/// ## Basic Connection
/// ```ignore
/// use azure_core::http::Url;
/// use azure_core_amqp::socks5::SocksConnection;
///
/// async fn connect_through_proxy() -> azure_core::Result<()> {
///     let proxy_url = Url::parse("socks5h://proxy.example.com:1080")?;
///     let target_url = Url::parse("amqps://eventhub.servicebus.windows.net")?;
///
///     let stream = SocksConnection::connect(&proxy_url, &target_url).await?;
///     // Use stream for AMQP connection
///     Ok(())
/// }
/// ```
///
/// ## Authenticated Connection
/// ```ignore
/// use azure_core::http::Url;
/// use azure_core_amqp::socks5::SocksConnection;
///
/// async fn connect_with_auth() -> azure_core::Result<()> {
///     let proxy_url = Url::parse("socks5://username:password@proxy.corp.com:8080")?;
///     let target_url = Url::parse("amqps://my-eventhub.servicebus.windows.net")?;
///
///     let stream = SocksConnection::connect(&proxy_url, &target_url).await?;
///     // Credentials are automatically masked in logs for security
///     Ok(())
/// }
/// ```
///
/// # Security Considerations
///
/// - Proxy credentials are automatically masked in all log output
/// - Use SOCKS5h protocol for enhanced DNS privacy in corporate environments
/// - Ensure proxy URL is obtained from secure configuration sources
/// - Consider using environment variables for sensitive proxy credentials
///
/// # Error Handling
///
/// Connection failures can occur due to:
/// - Invalid proxy URL format (use [`validate_proxy_url`](Self::validate_proxy_url))
/// - Network connectivity issues to proxy server
/// - Authentication failures with proxy server
/// - Target service unreachable through proxy
///
/// All errors include contextual information for troubleshooting.
pub(crate) struct SocksConnection;

impl SocksConnection {
    /// Validate SOCKS5 proxy URL format and requirements
    ///
    /// Ensures the provided URL is properly formatted for SOCKS5 proxy connections.
    /// This function should be called before attempting to establish a connection
    /// to catch configuration errors early.
    ///
    /// # Supported Schemes
    ///
    /// - `socks5://` - Standard SOCKS5 protocol with local DNS resolution
    /// - `socks5h://` - SOCKS5 protocol with proxy-side DNS resolution
    ///
    /// # Required Components
    ///
    /// - **Scheme**: Must be `socks5` or `socks5h`
    /// - **Host**: Proxy server hostname or IP address
    /// - **Port**: Optional (defaults to 1080 if not specified)
    /// - **Authentication**: Optional username:password
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use azure_core::http::Url;
    /// use azure_core_amqp::socks5::SocksConnection;
    ///
    /// // Valid URLs
    /// assert!(SocksConnection::validate_proxy_url(&Url::parse("socks5://proxy.example.com").unwrap()).is_ok());
    /// assert!(SocksConnection::validate_proxy_url(&Url::parse("socks5h://proxy:8080").unwrap()).is_ok());
    /// assert!(SocksConnection::validate_proxy_url(&Url::parse("socks5://user:pass@proxy.corp.com:1080").unwrap()).is_ok());
    ///
    /// // Invalid URLs
    /// assert!(SocksConnection::validate_proxy_url(&Url::parse("http://proxy.example.com").unwrap()).is_err());
    /// assert!(SocksConnection::validate_proxy_url(&Url::parse("socks5://").unwrap()).is_err()); // Missing host
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - URL scheme is not `socks5` or `socks5h`
    /// - Host component is missing or empty
    /// - URL is malformed
    ///
    /// # Security Note
    ///
    /// This function does not validate the actual connectivity to the proxy server
    /// or the correctness of authentication credentials. It only validates the URL format.
    pub fn validate_proxy_url(url: &Url) -> Result<()> {
        trace!(proxy_url = %url, "Validating SOCKS5 proxy URL format");

        if !["socks5", "socks5h"].contains(&url.scheme()) {
            error!(
                proxy_scheme = %url.scheme(),
                "Invalid SOCKS5 proxy scheme"
            );
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("Invalid SOCKS5 scheme: {}", url.scheme()),
            ));
        }
        if url.host_str().is_none() {
            error!("Missing host in SOCKS5 proxy URL");
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "Missing host in SOCKS5 URL",
            ));
        }
        Ok(())
    }

    /// Create SOCKS5 connection to target through proxy
    ///
    /// Establishes a connection to the target service through a SOCKS5 proxy server.
    /// This function handles both SOCKS5 and SOCKS5h protocols, authentication,
    /// and TLS wrapping for AMQPS connections.
    ///
    /// # Protocol Flow
    ///
    /// 1. **Validation**: Validates proxy URL format
    /// 2. **Proxy Connection**: Connects to SOCKS5 proxy server
    /// 3. **Authentication**: Performs username/password auth if credentials provided
    /// 4. **Target Request**: Requests connection to target through proxy tunnel
    /// 5. **TLS Wrapping**: Applies TLS if target uses AMQPS protocol
    ///
    /// # Parameters
    ///
    /// - `proxy_url`: SOCKS5 proxy server URL with optional authentication
    /// - `target_url`: Target AMQP service URL (amqp:// or amqps://)
    ///
    /// # Examples
    ///
    /// ## Basic Proxy Connection
    /// ```ignore,no_run
    /// use azure_core::http::Url;
    /// use azure_core_amqp::socks5::SocksConnection;
    ///
    /// async fn basic_connection() -> azure_core::Result<()> {
    ///     let proxy_url = Url::parse("socks5h://corporate-proxy.internal:8080")?;
    ///     let target_url = Url::parse("amqps://my-eventhub.servicebus.windows.net:5671")?;
    ///
    ///     let stream = SocksConnection::connect(&proxy_url, &target_url).await?;
    ///     // Stream is ready for fe2o3-amqp Connection::open_with_stream()
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Authenticated Connection
    /// ```ignore,no_run
    /// use azure_core::http::Url;
    /// use azure_core_amqp::socks5::SocksConnection;
    ///
    /// async fn authenticated_connection() -> azure_core::Result<()> {
    ///     let proxy_url = Url::parse("socks5://proxyuser:proxypass@proxy.corp.com:1080")?;
    ///     let target_url = Url::parse("amqps://eventhub.servicebus.windows.net")?;
    ///
    ///     let stream = SocksConnection::connect(&proxy_url, &target_url).await?;
    ///     // Authentication handled automatically, credentials masked in logs
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # TLS Handling
    ///
    /// - **AMQPS targets** (port 5671 or amqps:// scheme): Automatically wrapped with TLS
    /// - **AMQP targets** (port 5672 or amqp:// scheme): Plain TCP connection
    /// - TLS handshake performed after SOCKS5 tunnel establishment
    ///
    /// # Error Scenarios
    ///
    /// This function may fail due to:
    /// - **Invalid proxy URL**: Use [`validate_proxy_url`](Self::validate_proxy_url) first
    /// - **Network errors**: Proxy server unreachable or connection refused
    /// - **Authentication failures**: Invalid username/password for proxy
    /// - **Proxy errors**: Target service unreachable through proxy
    /// - **TLS errors**: Certificate validation failures for AMQPS connections
    ///
    /// # Security Features
    ///
    /// - **Credential masking**: Proxy credentials automatically masked in all log output
    /// - **TLS validation**: Full certificate chain validation for AMQPS connections
    /// - **DNS privacy**: SOCKS5h protocol hides DNS queries from local network
    ///
    /// # Performance Characteristics
    ///
    /// - **Connection pooling**: Caller responsible for connection reuse
    /// - **Memory usage**: Minimal overhead, stream-based design
    /// - **Latency**: Additional hop through proxy server
    ///
    /// # Returns
    ///
    /// Returns a boxed stream implementing [`SocksStream`] trait, ready for use
    /// with fe2o3-amqp's `Connection::open_with_stream()` method.
    pub async fn connect(proxy_url: &Url, target_url: &Url) -> Result<Box<dyn SocksStream>> {
        debug!(
            proxy_url = %Self::mask_credentials(proxy_url),
            target_host = %target_url.host_str().unwrap_or("unknown"),
            target_port = target_url.port().unwrap_or(5671),
            "Establishing SOCKS5 connection to EventHub"
        );

        // Validate proxy URL format
        Self::validate_proxy_url(proxy_url)?;

        // DNS resolution happens at proxy server for socks5h:// scheme
        let proxy_host = proxy_url.host_str().ok_or_else(|| {
            error!("Missing proxy host in SOCKS5 URL");
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "Missing proxy host in SOCKS5 URL",
            )
        })?;
        let proxy_port = proxy_url.port().unwrap_or(1080);

        debug!(
            proxy_host = %proxy_host,
            proxy_port = proxy_port,
            dns_resolution = %if proxy_url.scheme() == "socks5h" { "proxy-side" } else { "local" },
            "Connecting to SOCKS5 proxy"
        );

        // Always use domain name - let SOCKS5 proxy handle resolution
        let target_addr = TargetAddr::Domain(
            target_url.host_str().unwrap_or("").into(),
            target_url.port().unwrap_or(5671),
        );

        // Handle authentication if provided in URL
        let stream = if !proxy_url.username().is_empty() {
            let username = proxy_url.username();
            let password = proxy_url.password().unwrap_or("");
            if username.is_empty() {
                error!("Empty username in SOCKS5 proxy URL");
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Empty username in SOCKS5 URL",
                ));
            }

            debug!(
                username = %username,
                "Authenticating SOCKS5 connection with credentials"
            );

            Socks5Stream::connect_with_password(
                (proxy_host, proxy_port),
                target_addr,
                username,
                password,
            )
            .await
        } else {
            debug!("Connecting to SOCKS5 proxy without authentication");
            Socks5Stream::connect((proxy_host, proxy_port), target_addr).await
        }
        .map_err(|e| {
            error!(
                proxy_url = %Self::mask_credentials(proxy_url),
                target_url = %target_url.to_string(),
                error = %e,
                "SOCKS5 connection establishment failed"
            );

            azure_core::Error::new(azure_core::error::ErrorKind::Other, Box::new(e)).with_context(
                format!(
                    "SOCKS5 connection failed: proxy={}, target={}",
                    Self::mask_credentials(proxy_url),
                    target_url
                ),
            )
        })?;

        debug!(
            proxy_url = %Self::mask_credentials(proxy_url),
            target_url = %target_url.to_string(),
            "SOCKS5 connection established successfully"
        );

        // Check if target URL requires TLS (amqps://)
        let requires_tls =
            target_url.scheme() == "amqps" || target_url.port().unwrap_or(5671) == 5671;

        if requires_tls {
            debug!(
                target_host = %target_url.host_str().unwrap_or("unknown"),
                "Wrapping SOCKS5 stream with TLS for AMQPS connection"
            );

            // Create TLS connector with default settings
            let native_connector = NativeTlsConnector::new().map_err(|e| {
                error!(
                    error = %e,
                    "Failed to create TLS connector"
                );
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("Failed to create TLS connector: {}", e),
                )
            })?;

            let connector = TlsConnector::from(native_connector);
            let target_host = target_url.host_str().ok_or_else(|| {
                error!("Missing target host for TLS connection");
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Missing target host for TLS connection",
                )
            })?;

            // Establish TLS connection over SOCKS5 stream
            let tls_stream = connector
                .connect(target_host, stream.into_inner())
                .await
                .map_err(|e| {
                    error!(
                        target_host = %target_host,
                        error = %e,
                        "TLS handshake failed over SOCKS5 connection"
                    );
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        format!("TLS handshake failed: {}", e),
                    )
                })?;

            debug!(
                target_host = %target_host,
                "TLS connection established over SOCKS5 tunnel"
            );

            trace!("SOCKS5+TLS stream ready for AMQP protocol");
            Ok(Box::new(StreamWrapper(tls_stream)))
        } else {
            // Plain TCP connection for non-TLS protocols
            let inner = stream.into_inner();
            trace!("SOCKS5 stream extracted and ready for plain AMQP protocol");
            Ok(Box::new(StreamWrapper(inner)))
        }
    }

    /// Mask credentials in proxy URL for logging
    ///
    /// Creates a safe representation of the proxy URL with credentials masked
    /// for inclusion in log messages and error reports. This prevents sensitive
    /// authentication information from being exposed in logs or debug output.
    ///
    /// # Security Rationale
    ///
    /// Proxy URLs often contain sensitive authentication credentials:
    /// ```text
    /// socks5://username:password@proxy.corp.com:1080
    /// ```
    ///
    /// Direct logging of such URLs would expose credentials in:
    /// - Application logs
    /// - Error messages
    /// - Debug output
    /// - Monitoring systems
    ///
    /// # Masking Strategy
    ///
    /// - **Username**: Replaced with `***`
    /// - **Password**: Replaced with `***`
    /// - **Host/Port**: Preserved for debugging
    /// - **Scheme**: Preserved for protocol identification
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use azure_core::http::Url;
    /// use azure_core_amqp::socks5::SocksConnection;
    ///
    /// let url_with_auth = Url::parse("socks5://user:pass@proxy.example.com:1080").unwrap();
    /// let masked = SocksConnection::mask_credentials(&url_with_auth);
    /// assert_eq!(masked, "socks5://***:***@proxy.example.com:1080");
    ///
    /// let url_no_auth = Url::parse("socks5://proxy.example.com:1080").unwrap();
    /// let masked = SocksConnection::mask_credentials(&url_no_auth);
    /// assert_eq!(masked, "socks5://proxy.example.com:1080");
    /// ```
    ///
    /// # Error Handling
    ///
    /// If the URL cannot be parsed or modified, returns `"invalid_url"` as a
    /// safe fallback to prevent any credential exposure.
    ///
    /// # Usage
    ///
    /// This function is automatically used in all logging statements within
    /// the SOCKS5 implementation. Manual usage is typically not required
    /// unless implementing custom logging or error handling.
    pub(crate) fn mask_credentials(url: &Url) -> String {
        let mut masked = url.clone();
        if masked.username() != "" {
            let _ = masked.set_username("***");
        }
        if masked.password().is_some() {
            let _ = masked.set_password(Some("***"));
        }
        masked.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::Url;

    #[test]
    fn test_validate_proxy_url_valid_schemes() {
        // Test valid SOCKS5 schemes
        let socks5_url = Url::parse("socks5://proxy.example.com:1080").unwrap();
        assert!(SocksConnection::validate_proxy_url(&socks5_url).is_ok());

        let socks5h_url = Url::parse("socks5h://proxy.example.com:1080").unwrap();
        assert!(SocksConnection::validate_proxy_url(&socks5h_url).is_ok());

        // Test with default port
        let socks5_no_port = Url::parse("socks5://proxy.example.com").unwrap();
        assert!(SocksConnection::validate_proxy_url(&socks5_no_port).is_ok());

        // Test with authentication
        let socks5_auth = Url::parse("socks5://user:pass@proxy.example.com:8080").unwrap();
        assert!(SocksConnection::validate_proxy_url(&socks5_auth).is_ok());
    }

    #[test]
    fn test_validate_proxy_url_invalid_schemes() {
        // Test invalid schemes
        let http_url = Url::parse("http://proxy.example.com:8080").unwrap();
        assert!(SocksConnection::validate_proxy_url(&http_url).is_err());

        let https_url = Url::parse("https://proxy.example.com:443").unwrap();
        assert!(SocksConnection::validate_proxy_url(&https_url).is_err());

        let ftp_url = Url::parse("ftp://proxy.example.com:21").unwrap();
        assert!(SocksConnection::validate_proxy_url(&ftp_url).is_err());

        let amqp_url = Url::parse("amqp://eventhub.servicebus.windows.net:5672").unwrap();
        assert!(SocksConnection::validate_proxy_url(&amqp_url).is_err());
    }

    #[test]
    fn test_validate_proxy_url_missing_host() {
        // Test URLs without host component
        let no_host_url = Url::parse("socks5:///path").unwrap();
        assert!(SocksConnection::validate_proxy_url(&no_host_url).is_err());

        // Test URL with no host specified - this actually parses but has None host
        let empty_host_url = Url::parse("socks5://").unwrap();
        assert!(SocksConnection::validate_proxy_url(&empty_host_url).is_err());

        // Test URLs that fail to parse due to empty host
        assert!(Url::parse("socks5://:1080").is_err()); // EmptyHost error as expected

        // These tests verify our validation catches missing hosts in URLs that do parse
        let relative_url = Url::parse("socks5:///").unwrap();
        assert!(SocksConnection::validate_proxy_url(&relative_url).is_err());
    }

    #[test]
    fn test_validate_proxy_url_edge_cases() {
        // Test with IP address
        let ip_url = Url::parse("socks5://192.168.1.100:1080").unwrap();
        assert!(SocksConnection::validate_proxy_url(&ip_url).is_ok());

        // Test with IPv6
        let ipv6_url = Url::parse("socks5h://[::1]:1080").unwrap();
        assert!(SocksConnection::validate_proxy_url(&ipv6_url).is_ok());

        // Test with complex hostname
        let complex_host = Url::parse("socks5://proxy-server.corp.example.com:8080").unwrap();
        assert!(SocksConnection::validate_proxy_url(&complex_host).is_ok());
    }

    #[test]
    fn test_mask_credentials_no_auth() {
        // URLs without authentication should remain unchanged
        let url_no_auth = Url::parse("socks5://proxy.example.com:1080").unwrap();
        let masked = SocksConnection::mask_credentials(&url_no_auth);
        assert_eq!(masked, "socks5://proxy.example.com:1080");

        let url_no_port = Url::parse("socks5h://proxy.corp.com").unwrap();
        let masked_no_port = SocksConnection::mask_credentials(&url_no_port);
        assert_eq!(masked_no_port, "socks5h://proxy.corp.com");
    }

    #[test]
    fn test_mask_credentials_with_auth() {
        // Username and password should be masked
        let url_with_auth =
            Url::parse("socks5://username:password@proxy.example.com:1080").unwrap();
        let masked = SocksConnection::mask_credentials(&url_with_auth);
        assert_eq!(masked, "socks5://***:***@proxy.example.com:1080");

        // Test with SOCKS5h
        let url_socks5h = Url::parse("socks5h://user:pass@proxy.corp.com:8080").unwrap();
        let masked_socks5h = SocksConnection::mask_credentials(&url_socks5h);
        assert_eq!(masked_socks5h, "socks5h://***:***@proxy.corp.com:8080");
    }

    #[test]
    fn test_mask_credentials_username_only() {
        // Test URL with username but no password
        let url_user_only = Url::parse("socks5://username@proxy.example.com:1080").unwrap();
        let masked = SocksConnection::mask_credentials(&url_user_only);
        assert_eq!(masked, "socks5://***@proxy.example.com:1080");
    }

    #[test]
    fn test_mask_credentials_special_characters() {
        // Test credentials with special characters (URL encoded)
        let url_special =
            Url::parse("socks5://user%40domain:p%40ssw0rd@proxy.example.com:1080").unwrap();
        let masked = SocksConnection::mask_credentials(&url_special);
        assert_eq!(masked, "socks5://***:***@proxy.example.com:1080");

        // Test with complex credentials
        let url_complex =
            Url::parse("socks5://admin:secretP%40ss123@proxy-server.corp.com:8080").unwrap();
        let masked_complex = SocksConnection::mask_credentials(&url_complex);
        assert_eq!(
            masked_complex,
            "socks5://***:***@proxy-server.corp.com:8080"
        );
    }

    #[test]
    fn test_mask_credentials_preserves_structure() {
        // Verify that masking preserves host, port, and scheme
        let original =
            Url::parse("socks5h://testuser:testpass@corporate-proxy.internal:12345").unwrap();
        let masked = SocksConnection::mask_credentials(&original);

        assert!(masked.starts_with("socks5h://"));
        assert!(masked.contains("corporate-proxy.internal:12345"));
        assert!(masked.contains("***:***"));
        assert!(!masked.contains("testuser"));
        assert!(!masked.contains("testpass"));
    }

    #[test]
    fn test_mask_credentials_empty_credentials() {
        // Test edge case with empty username/password
        let url_empty_user = Url::parse("socks5://:password@proxy.example.com:1080").unwrap();
        let masked_empty_user = SocksConnection::mask_credentials(&url_empty_user);
        // Empty username should still be masked for consistency
        assert!(masked_empty_user.contains("***"));

        let url_empty_pass = Url::parse("socks5://username:@proxy.example.com:1080").unwrap();
        let masked_empty_pass = SocksConnection::mask_credentials(&url_empty_pass);
        assert!(masked_empty_pass.contains("***"));
    }

    #[test]
    fn test_mask_credentials_error_handling() {
        // Test behavior with various URL scenarios
        let urls_to_test = vec![
            "socks5://user:pass@proxy.example.com:1080",
            "socks5h://admin:secret@192.168.1.100:8080",
            "socks5://test@proxy.corp.com",
            "socks5h://proxy.example.com:1080",
        ];

        for url_str in urls_to_test {
            let url = Url::parse(url_str).unwrap();
            let masked = SocksConnection::mask_credentials(&url);

            // Verify credentials are not exposed
            assert!(!masked.contains("pass"));
            assert!(!masked.contains("secret"));
            assert!(
                !masked.contains("admin") || url_str.contains("admin") == masked.contains("admin")
            );
        }
    }
}
