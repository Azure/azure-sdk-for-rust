// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod content_range;
mod if_sequence_number;
mod if_source_match_condition;
mod if_source_modified_since_condition;
mod lease;
mod lease_break_period;
mod lease_duration;
mod max_item_count;
mod max_results;
mod metadata;
mod next_marker;
mod proposed_lease_id;
mod range;
mod sequence_number;
mod source_lease_id;
mod timeout;

use crate::headers::{
    ACTIVITY_ID, APP, CLIENT_REQUEST_ID, CLIENT_VERSION, CONTENT_DISPOSITION, CONTINUATION,
    IF_TAGS, USER, VERSION,
};
pub use content_range::ContentRange;
pub use if_sequence_number::IfSequenceNumber;
pub use if_source_match_condition::IfSourceMatchCondition;
pub use if_source_modified_since_condition::IfSourceModifiedSinceCondition;
pub use lease::LeaseId;
pub use lease_break_period::LeaseBreakPeriod;
pub use lease_duration::LeaseDuration;
pub use max_item_count::MaxItemCount;
pub use max_results::MaxResults;
pub use metadata::Metadata;
pub use next_marker::NextMarker;
pub use proposed_lease_id::ProposedLeaseId;
pub use range::Range;
pub use sequence_number::SequenceNumber;
pub use source_lease_id::SourceLeaseId;
pub use timeout::Timeout;
pub use typespec_client_core::http::request::options::*;
use typespec_client_core::{request_header, request_query};

request_header!(
    /// The (friendly) version identifier for the client making the request
    ClientVersion,
    CLIENT_VERSION
);

request_header!(
    /// The (friendly) name of the user making the request
    User,
    USER,
);

request_header!(ActivityId, ACTIVITY_ID);
request_header!(App, APP);
request_header!(ClientRequestId, CLIENT_REQUEST_ID);
request_header!(ContentDisposition, CONTENT_DISPOSITION);
request_header!(Continuation, CONTINUATION);
request_header!(IfTags, IF_TAGS);
request_header!(Version, VERSION);

request_query!(
    /// Set delimiter for the request
    Delimiter,
    "delimiter"
);
request_query!(Prefix, "prefix");
