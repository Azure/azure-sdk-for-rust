// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::Url;

/// Appends an already-signed SAS token to a resource URL.
///
/// [`SasBuilder::token`](crate::SasBuilder) returns the SAS as a ready-to-use
/// query string (without a leading `?`). This function attaches that token to a
/// resource URL, returning the authenticated URL you can hand to an
/// unauthenticated client such as `BlobClient::new`.
///
/// Two hazards make a bare `Url::set_query` the wrong tool, and this function
/// avoids both:
///
/// - **It preserves any existing query string.** Some resource URLs already
///   carry a query parameter that must be kept alongside the signature — for
///   example a version SAS appends `versionid=` to the blob URL. `set_query`
///   would overwrite it; this function joins the existing query and the token
///   with `&`.
/// - **It does not re-encode the token.** The token is already
///   percent-encoded. Building it through `Url::query_pairs_mut` (or any helper
///   that percent-encodes its input) would double-encode it (for example
///   `%2F` becomes `%252F`), corrupting the signature and producing a `403`.
///   This function assigns the token verbatim.
///
/// The `token` must be the raw output of [`SasBuilder::token`](crate::SasBuilder)
/// with no leading `?`.
///
/// # Examples
///
/// ```
/// use azure_core::http::Url;
/// use azure_storage_sas::append_token;
///
/// // A version SAS URL already carries `versionid=`, which must be preserved.
/// let url = Url::parse(
///     "https://account.blob.core.windows.net/container/blob?versionid=2024-01-01T00%3A00%3A00Z",
/// )
/// .unwrap();
/// let token = "sv=2026-04-06&sr=bv&sig=a%2Fb";
///
/// let sas_url = append_token(url, token);
/// let query = sas_url.query().unwrap();
/// assert!(query.starts_with("versionid=2024-01-01T00%3A00%3A00Z&"));
/// assert!(query.ends_with("sv=2026-04-06&sr=bv&sig=a%2Fb"));
/// // The already-encoded `%2F` in the signature is left untouched.
/// assert!(query.contains("sig=a%2Fb"));
/// ```
pub fn append_token(mut url: Url, token: &str) -> Url {
    match url.query() {
        Some(existing) if !existing.is_empty() => {
            url.set_query(Some(&format!("{existing}&{token}")));
        }
        _ => url.set_query(Some(token)),
    }
    url
}

#[cfg(test)]
mod tests {
    use super::append_token;
    use azure_core::http::Url;

    #[test]
    fn appends_to_url_without_query() {
        let url = Url::parse("https://account.blob.core.windows.net/container/blob").unwrap();
        let sas_url = append_token(url, "sv=2026-04-06&sig=abc");
        assert_eq!(sas_url.query(), Some("sv=2026-04-06&sig=abc"));
    }

    #[test]
    fn preserves_existing_query() {
        let url =
            Url::parse("https://account.blob.core.windows.net/container/blob?versionid=2024-01-01")
                .unwrap();
        let sas_url = append_token(url, "sv=2026-04-06&sr=bv&sig=abc");
        assert_eq!(
            sas_url.query(),
            Some("versionid=2024-01-01&sv=2026-04-06&sr=bv&sig=abc")
        );
    }

    #[test]
    fn treats_empty_query_as_absent() {
        let url = Url::parse("https://account.blob.core.windows.net/container/blob?").unwrap();
        let sas_url = append_token(url, "sv=2026-04-06&sig=abc");
        assert_eq!(sas_url.query(), Some("sv=2026-04-06&sig=abc"));
    }

    #[test]
    fn does_not_re_encode_token() {
        let url = Url::parse("https://account.blob.core.windows.net/container/blob").unwrap();
        let sas_url = append_token(url, "sig=a%2Fb%2Bc");
        // The pre-encoded token must survive verbatim, not become `%252F`.
        assert_eq!(sas_url.query(), Some("sig=a%2Fb%2Bc"));
    }
}
