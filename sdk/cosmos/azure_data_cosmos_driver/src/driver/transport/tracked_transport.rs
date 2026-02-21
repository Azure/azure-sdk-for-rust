// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Transport send-status inference utilities.
//!
//! This module determines whether a request was definitely sent on the wire before
//! a transport error occurred. The information is used by retry safety gates.

/// Indicates whether a request was definitely sent on the wire before failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RequestSentStatus {
    /// Request was definitely sent.
    Sent,
    /// Request was definitely not sent.
    NotSent,
    /// Could not determine whether request was sent.
    Unknown,
}

impl RequestSentStatus {
    pub(crate) fn definitely_not_sent(self) -> bool {
        matches!(self, RequestSentStatus::NotSent)
    }
}

/// Extension trait for determining request sent status from errors.
pub(crate) trait RequestSentExt {
    /// Returns the request sent status based on error analysis.
    fn request_sent_status(&self) -> RequestSentStatus;
}

impl RequestSentExt for azure_core::Error {
    fn request_sent_status(&self) -> RequestSentStatus {
        use azure_core::error::ErrorKind;

        // TODO @fabianm: this is a temporary hack - will need to be revisited when making the transport layer more robust
        // and adaptable via feature flags (reqwest vs. fetch).
        // The idea is to classify certain types of errors (like DNS failures or credential errors) for fetch and reqwest.
        match self.kind() {
            ErrorKind::Io => {
                let msg = self.to_string().to_lowercase();
                if msg.contains("dns")
                    || msg.contains("resolve")
                    || msg.contains("connection refused")
                    || msg.contains("no route to host")
                    || msg.contains("network unreachable")
                    || msg.contains("connection reset")
                        && (msg.contains("before") || msg.contains("establish"))
                {
                    return RequestSentStatus::NotSent;
                }
                RequestSentStatus::Unknown
            }
            ErrorKind::Credential => RequestSentStatus::NotSent,
            ErrorKind::DataConversion => RequestSentStatus::NotSent,
            ErrorKind::HttpResponse { .. } => RequestSentStatus::Sent,
            _ => RequestSentStatus::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind;

    #[test]
    fn dns_error_not_sent() {
        let err = azure_core::Error::new(ErrorKind::Io, "dns resolution failed");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn connection_refused_not_sent() {
        let err = azure_core::Error::new(ErrorKind::Io, "connection refused");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn timeout_is_unknown() {
        let err = azure_core::Error::new(ErrorKind::Io, "operation timed out");
        assert_eq!(err.request_sent_status(), RequestSentStatus::Unknown);
    }

    #[test]
    fn credential_error_not_sent() {
        let err = azure_core::Error::new(ErrorKind::Credential, "invalid token");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn data_conversion_error_not_sent() {
        let err = azure_core::Error::new(ErrorKind::DataConversion, "serialization failed");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn unknown_error_is_unknown() {
        let err = azure_core::Error::new(ErrorKind::Other, "something went wrong");
        assert_eq!(err.request_sent_status(), RequestSentStatus::Unknown);
    }
}
