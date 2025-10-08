// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{BlobTag, BlobTags};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use url::Url;

/// Takes in an offset and a length, verifies alignment to a 512-byte boundary, and
///  returns the HTTP range in String format.
///
/// # Arguments
///
/// * `offset` - Start of the byte range to use for writing to a section of the blob.
///   The offset specified must be a modulus of 512.
/// * `length` - Number of bytes to use for writing to a section of the blob.
///   The length specified must be a modulus of 512.
pub fn format_page_range(offset: u64, length: u64) -> Result<String, Error> {
    if offset % 512 != 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "provided offset {} is not aligned to a 512-byte boundary.",
                offset
            ),
        ));
    }
    if length % 512 != 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "provided length {} is not aligned to a 512-byte boundary.",
                offset
            ),
        ));
    }
    let end_range = offset + length - 1;
    let content_range = format!("bytes={}-{}", offset, end_range);
    Ok(content_range)
}

/// Takes in a HashMap of tag key-value pairs and converts them to a filter expression
/// for use with [`BlobServiceClient::find_blobs_by_tags()`](crate::BlobServiceClient::find_blobs_by_tags) or [`BlobContainerClient::find_blobs_by_tags()`](crate::BlobContainerClient::find_blobs_by_tags).
///
/// # Arguments
///
/// * `tags` - A HashMap containing tag key-value pairs representing the
///   expression to find blobs whose tags match the specified condition.
pub fn format_filter_expression(tags: &HashMap<String, String>) -> Result<String, Error> {
    if tags.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Tags HashMap cannot be empty.".to_string(),
        ));
    }

    let format_expression: Vec<String> = tags
        .iter()
        .map(|(key, value)| format!("\"{}\"='{}'", key, value))
        .collect();

    Ok(format_expression.join(" and "))
}

/// Builds a blob URL from the endpoint, container name, and blob name.
///
/// # Arguments
///
/// * `endpoint` - The base endpoint URL (e.g., `https://myaccount.blob.core.windows.net/`)
/// * `container_name` - The name of the container
/// * `blob_name` - The name of the blob
pub(crate) fn build_blob_url(endpoint: &str, container_name: &str, blob_name: &str) -> String {
    // Remove trailing slash from endpoint if present
    let endpoint = endpoint.trim_end_matches('/');

    // URL-encode the container name
    let encoded_container_name = urlencoding::encode(container_name);

    // Split blob name by '/' and encode each segment individually
    let encoded_blob_segments: Vec<String> = blob_name
        .split('/')
        .map(|segment| urlencoding::encode(segment).into_owned())
        .collect();
    let encoded_blob_name = encoded_blob_segments.join("/");

    format!(
        "{}/{}/{}",
        endpoint, encoded_container_name, encoded_blob_name
    )
}

/// Parses the container name and blob name from a full blob URL.
///
/// # Arguments
///
/// * `url` - The URL to parse.
pub(crate) fn parse_url_name_components(url: &str) -> azure_core::Result<(String, String)> {
    // Find the path part of the URL (after the domain)
    // Look for the third '/' which starts the path
    let mut slash_count = 0;
    let mut path_start = 0;

    for (idx, c) in url.char_indices() {
        if c == '/' {
            slash_count += 1;
            if slash_count == 3 {
                path_start = idx + 1;
                break;
            }
        }
    }

    if slash_count < 3 {
        return Err(azure_core::Error::new(
            azure_core::error::ErrorKind::Other,
            "Invalid URL format: missing path.",
        ));
    }

    // Extract the path, excluding query string (anything after '?')
    let path = &url[path_start..];
    let path = if let Some(query_start) = path.find('?') {
        &path[..query_start]
    } else {
        path
    };

    // Find the first '/' in the path to split container name and blob name
    let slash_pos = path.find('/').ok_or_else(|| {
        azure_core::Error::new(
            azure_core::error::ErrorKind::Other,
            "URL must contain both container name and blob name.",
        )
    })?;

    let container_encoded = &path[..slash_pos];
    let blob_encoded = &path[slash_pos + 1..];

    if blob_encoded.is_empty() {
        return Err(azure_core::Error::new(
            azure_core::error::ErrorKind::Other,
            "Blob name cannot be empty.",
        ));
    }

    // URL decode both components
    let container_name = urlencoding::decode(container_encoded).map_err(|e| {
        azure_core::Error::new(
            azure_core::error::ErrorKind::Other,
            format!("Failed to decode container name: {}", e),
        )
    })?;

    let blob_name = urlencoding::decode(blob_encoded).map_err(|e| {
        azure_core::Error::new(
            azure_core::error::ErrorKind::Other,
            format!("Failed to decode blob name: {}", e),
        )
    })?;

    Ok((container_name.into_owned(), blob_name.into_owned()))
}

// TODO: Remove later, but need these for sanity checking while we discuss how to move forward with this design
#[cfg(test)]
mod tests {
    use super::*;

    // Basic tests for baseline understanding
    #[test]
    fn simple_blob_name() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "myblob.txt",
        );
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/myblob.txt"
        );
    }

    #[test]
    fn blob_name_with_single_path() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder/myblob.txt",
        );
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder/myblob.txt"
        );
    }

    // Path preservation tests - the critical functionality
    #[test]
    fn nested_paths_basic() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder/subfolder/another/myblob.txt",
        );
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder/subfolder/another/myblob.txt"
        );
    }

    #[test]
    fn nested_paths_with_spaces() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "my folder/sub folder/file name.txt",
        );
        // Spaces encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/my%20folder/sub%20folder/file%20name.txt"
        );
    }

    #[test]
    fn nested_paths_with_leading_trailing_spaces() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "  folder  /  subfolder  /  file.txt  ",
        );
        // Spaces encoded everywhere, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/%20%20folder%20%20/%20%20subfolder%20%20/%20%20file.txt%20%20"
        );
    }

    #[test]
    fn nested_paths_with_special_chars() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder1/folder&2/file?name.txt",
        );
        // Special characters encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder1/folder%262/file%3Fname.txt"
        );
    }

    #[test]
    fn nested_paths_with_query_string_chars() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "api/v1/users?id=123&name=test/data.json",
        );
        // Query characters encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/api/v1/users%3Fid%3D123%26name%3Dtest/data.json"
        );
    }

    #[test]
    fn nested_paths_with_hash_fragments() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "docs/v1/page#section1/subsection#2/file.txt",
        );
        // Hash characters encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/docs/v1/page%23section1/subsection%232/file.txt"
        );
    }

    #[test]
    fn nested_paths_with_semicolons() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder;version/subfolder;data/file;v2.txt",
        );
        // Semicolons encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder%3Bversion/subfolder%3Bdata/file%3Bv2.txt"
        );
    }

    #[test]
    fn nested_paths_with_at_signs() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "users/user@domain.com/files/doc@v1.txt",
        );
        // @ signs encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/users/user%40domain.com/files/doc%40v1.txt"
        );
    }

    #[test]
    fn nested_paths_with_pipes() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "data|2024/logs|prod/file|output.txt",
        );
        // Pipes encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/data%7C2024/logs%7Cprod/file%7Coutput.txt"
        );
    }

    #[test]
    fn nested_paths_with_commas() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "data,backups/logs,archive/file,v1.txt",
        );
        // Commas encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/data%2Cbackups/logs%2Carchive/file%2Cv1.txt"
        );
    }

    #[test]
    fn nested_paths_with_encoded_slashes() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder/file%2Fname/subfolder/data%2Fmore.txt",
        );
        // Already encoded slashes re-encoded to %252F, real slashes preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder/file%252Fname/subfolder/data%252Fmore.txt"
        );
    }

    #[test]
    fn nested_paths_with_plus_signs() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder+data/sub+folder/file+name.txt",
        );
        // Plus signs encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder%2Bdata/sub%2Bfolder/file%2Bname.txt"
        );
    }

    #[test]
    fn nested_paths_with_equals_signs() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "data=v1/logs=prod/file=output.txt",
        );
        // Equals signs encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/data%3Dv1/logs%3Dprod/file%3Doutput.txt"
        );
    }

    #[test]
    fn deeply_nested_paths_with_mixed_special_chars() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "level1/level 2/level&3/level?4/level#5/file name & data?test=1#section.txt",
        );
        // All special chars encoded, all paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/level1/level%202/level%263/level%3F4/level%235/file%20name%20%26%20data%3Ftest%3D1%23section.txt"
        );
    }

    #[test]
    fn nested_paths_comprehensive_url_unsafe_chars() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "dir&1/dir?2/dir=3/dir#4/dir;5/dir,6/dir@7/dir|8/file.txt",
        );
        // Every special char encoded, every path preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/dir%261/dir%3F2/dir%3D3/dir%234/dir%3B5/dir%2C6/dir%407/dir%7C8/file.txt"
        );
    }

    #[test]
    fn nested_paths_with_parentheses_and_brackets() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder(1)/subfolder[2]/file{name}.txt",
        );
        // Parentheses and brackets encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder%281%29/subfolder%5B2%5D/file%7Bname%7D.txt"
        );
    }

    #[test]
    fn nested_paths_with_percent_signs() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder%20test/sub%folder/file%name.txt",
        );
        // Percent signs encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder%2520test/sub%25folder/file%25name.txt"
        );
    }

    #[test]
    fn nested_paths_with_backslashes() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder\\windows/sub\\path/file\\name.txt",
        );
        // Backslashes encoded, forward slashes preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder%5Cwindows/sub%5Cpath/file%5Cname.txt"
        );
    }

    #[test]
    fn very_deep_nesting_ten_levels() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "l1/l2/l3/l4/l5/l6/l7/l8/l9/l10/file.txt",
        );
        // All 10 levels of paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/l1/l2/l3/l4/l5/l6/l7/l8/l9/l10/file.txt"
        );
    }

    #[test]
    fn nested_paths_with_unicode_chars() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder/文件夹/unterordner/файл.txt",
        );
        // Unicode chars encoded, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder/%E6%96%87%E4%BB%B6%E5%A4%B9/unterordner/%D1%84%D0%B0%D0%B9%D0%BB.txt"
        );
    }

    #[test]
    fn nested_paths_empty_segments() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "folder//subfolder///file.txt",
        );
        // Empty segments preserved as empty between slashes
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder//subfolder///file.txt"
        );
    }

    #[test]
    fn nested_paths_all_edge_cases_combined() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net",
            "mycontainer",
            "user@domain/logs|2024/file name & data/test?query=val#section/output;v2/data,final/result%2Fencoded/file(1)[2]{3}.txt",
        );
        // Kitchen sink test: all special chars encoded, all 8 path separators preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/user%40domain/logs%7C2024/file%20name%20%26%20data/test%3Fquery%3Dval%23section/output%3Bv2/data%2Cfinal/result%252Fencoded/file%281%29%5B2%5D%7B3%7D.txt"
        );
    }

    #[test]
    fn endpoint_with_trailing_slash() {
        let url = build_blob_url(
            "https://myaccount.blob.core.windows.net/",
            "mycontainer",
            "folder/subfolder/myblob.txt",
        );
        // Trailing slash handled, paths preserved
        assert_eq!(
            url,
            "https://myaccount.blob.core.windows.net/mycontainer/folder/subfolder/myblob.txt"
        );
    }
}
