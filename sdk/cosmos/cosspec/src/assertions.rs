// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The assertion vocabulary. Keep this list short: reviewers must be able to hold
//! every helper in their head.

/// Asserts that two collections hold the same items, ignoring order.
///
/// ```rust,ignore
/// assert_items_unordered!(items, container.dataset().all());
/// ```
#[macro_export]
macro_rules! assert_items_unordered {
    ($actual:expr, $expected:expr $(,)?) => {{
        let mut actual: ::std::vec::Vec<_> =
            ::std::iter::IntoIterator::into_iter($actual).collect();
        let mut expected: ::std::vec::Vec<_> =
            ::std::iter::IntoIterator::into_iter($expected).collect();
        actual.sort();
        expected.sort();
        ::std::assert_eq!(actual, expected, "items differ (order ignored)");
    }};
}

/// Asserts that an `ExclusiveContainer` has exactly `n` feed ranges, as observed by
/// a fresh client. Use inside an `async` spec.
///
/// ```rust,ignore
/// assert_feed_ranges!(container, 2);
/// ```
#[macro_export]
macro_rules! assert_feed_ranges {
    ($container:expr, $n:expr $(,)?) => {{
        let count = $container.feed_range_count().await?;
        ::std::assert_eq!(count, $n, "unexpected feed range count");
    }};
}

/// Asserts that an SDK call failed with the given HTTP status, and optionally
/// sub-status.
///
/// ```rust,ignore
/// assert_status!(client.read_item("pk", "missing", None).await, 404);
/// ```
#[macro_export]
macro_rules! assert_status {
    ($result:expr, $status:expr $(,)?) => {{
        match $result {
            ::std::result::Result::Ok(_) => {
                ::std::panic!("expected HTTP {}, but the call succeeded", $status)
            }
            ::std::result::Result::Err(error) => ::std::assert_eq!(
                u16::from(error.status().status_code()),
                $status,
                "unexpected status: {error}"
            ),
        }
    }};
    ($result:expr, $status:expr, $sub_status:expr $(,)?) => {{
        match $result {
            ::std::result::Result::Ok(_) => {
                ::std::panic!(
                    "expected HTTP {}/{}, but the call succeeded",
                    $status,
                    $sub_status
                )
            }
            ::std::result::Result::Err(error) => {
                ::std::assert_eq!(
                    u16::from(error.status().status_code()),
                    $status,
                    "unexpected status: {error}"
                );
                ::std::assert_eq!(
                    error.status().sub_status().map(u32::from),
                    ::std::option::Option::Some($sub_status),
                    "unexpected sub-status: {error}"
                );
            }
        }
    }};
}

/// Asserts how many HTTP exchanges the tap recorded for an operation.
///
/// ```rust,ignore
/// assert_http_count!(tap, "ReadItem", 3);
/// ```
#[macro_export]
macro_rules! assert_http_count {
    ($tap:expr, $operation:expr, $n:expr $(,)?) => {{
        let records = $tap.operation($operation);
        ::std::assert_eq!(
            records.len(),
            $n,
            "unexpected number of {} exchanges: {:#?}",
            $operation,
            records
        );
    }};
}

/// Asserts the region aliases an operation's HTTP exchanges were sent to, in
/// start order. An exchange to a region without an alias shows its real name.
///
/// ```rust,ignore
/// assert_http_regions!(tap, "ReadItem", [PrimaryReadWrite, SecondaryReadOnly]);
/// ```
#[macro_export]
macro_rules! assert_http_regions {
    ($tap:expr, $operation:expr, [$($region:expr),* $(,)?] $(,)?) => {{
        let regions: ::std::vec::Vec<::std::string::String> = $tap
            .operation($operation)
            .into_iter()
            .map(|r| r.region_alias.or(r.region).unwrap_or_default())
            .collect();
        let expected: ::std::vec::Vec<::std::string::String> =
            ::std::vec![$($crate::RegionAlias::name($region).to_owned()),*];
        ::std::assert_eq!(regions, expected, "unexpected regions for {}", $operation);
    }};
}

/// Asserts that two recorded HTTP exchanges were in flight at the same time.
///
/// ```rust,ignore
/// let reads = tap.operation("ReadItem");
/// assert_http_overlap!(reads[0], reads[1]);
/// ```
#[macro_export]
macro_rules! assert_http_overlap {
    ($a:expr, $b:expr $(,)?) => {{
        ::std::assert!(
            $a.overlaps(&$b),
            "expected concurrent exchanges, but they did not overlap:\n{:#?}\n{:#?}",
            $a,
            $b
        );
    }};
}
