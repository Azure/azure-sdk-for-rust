mod content_length;
mod content_range;
mod if_match_condition;
mod if_modified_since;
mod if_modified_since_condition;
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
mod sequence_number_condition;
mod source_lease_id;
mod timeout;

pub use content_length::ContentLength;
pub use content_range::ContentRange;
pub use if_match_condition::IfMatchCondition;
pub use if_modified_since::IfModifiedSince;
pub use if_modified_since_condition::IfModifiedSinceCondition;
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
pub use sequence_number_condition::SequenceNumberCondition;
pub use source_lease_id::SourceLeaseId;
pub use timeout::Timeout;

create_request_header_cow!(
    AcceptEncoding,
    ACCEPT_ENCODING,
    "Advertises which content encoding the client is able to understand.

The Accept-Encoding request HTTP header advertises which content
encoding, usually a compression algorithm, the client is able to
understand. Using content negotiation, the server selects one of the
proposals, uses it and informs the client of its choice with the
Content-Encoding response header.

Even if both the client and the server supports the same compression
algorithms, the server may choose not to compress the body of a
response, if the identity value is also acceptable.
",
);
create_request_header_cow!(
    ClientVersion,
    CLIENT_VERSION,
    "The (friendly) version identifier for the client making the request",
);
create_request_header_cow!(
    ContentType,
    CONTENT_TYPE,
    "The Content Type indicates the media type of the request body",
    (APPLICATION_JSON, "application/json")
);
create_request_header_cow!(
    Accept,
    ACCEPT,
    "Advertises which content types the client is able to understand.

The Accept request HTTP header advertises which content types, expressed
as MIME types, the client is able to understand. Using content
negotiation, the server then selects one of the proposals, uses it and
informs the client of its choice with the Content-Type response header.
Browsers set adequate values for this header depending of the context
where the request is done: when fetching a CSS stylesheet a different
value is set for the request than when fetching an image, video or a
script.
",
);
create_request_header_cow!(ActivityId, ACTIVITY_ID,);
create_request_header_cow!(App, APP,);
create_request_header_cow!(ClientRequestId, CLIENT_REQUEST_ID,);
create_request_header_cow!(ContentDisposition, CONTENT_DISPOSITION,);
create_request_header_cow!(ContentEncoding, CONTENT_ENCODING,);
create_request_header_cow!(ContentLanguage, CONTENT_LANGUAGE,);
create_request_header_cow!(Continuation, CONTINUATION,);
create_request_header_cow!(IfTags, IF_TAGS,);
create_request_header_cow!(UserAgent, USER_AGENT,);
create_request_header_cow!(User, USER,);
create_request_header_cow!(Version, VERSION,);

create_request_query_cow!(Prefix, "prefix");
create_request_query_cow!(Delimiter, "delimiter");
