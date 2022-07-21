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

request_header!(
    #[doc = "Advertises which content encoding the client is able to understand.

The Accept-Encoding request HTTP header advertises which content
encoding, usually a compression algorithm, the client is able to
understand. Using content negotiation, the server selects one of the
proposals, uses it and informs the client of its choice with the
Content-Encoding response header.

Even if both the client and the server supports the same compression
algorithms, the server may choose not to compress the body of a
response, if the identity value is also acceptable.
"]
    AcceptEncoding,
    ACCEPT_ENCODING,
);
request_header!(
    #[doc = "The (friendly) version identifier for the client making the request"]
    ClientVersion,
    CLIENT_VERSION,
);
request_header!(
    #[doc = "The Content Type indicates the media type of the request body"]
    ContentType,
    CONTENT_TYPE,
    (APPLICATION_JSON, "application/json")
);
request_header!(
    #[doc = "Advertises which content types the client is able to understand.

The Accept request HTTP header advertises which content types, expressed
as MIME types, the client is able to understand. Using content
negotiation, the server then selects one of the proposals, uses it and
informs the client of its choice with the Content-Type response header.
Browsers set adequate values for this header depending of the context
where the request is done: when fetching a CSS stylesheet a different
value is set for the request than when fetching an image, video or a
script.
"]
    Accept,
    ACCEPT,
);
request_header!(ActivityId, ACTIVITY_ID,);
request_header!(App, APP,);
request_header!(ClientRequestId, CLIENT_REQUEST_ID,);
request_header!(ContentDisposition, CONTENT_DISPOSITION,);
request_header!(ContentEncoding, CONTENT_ENCODING,);
request_header!(ContentLanguage, CONTENT_LANGUAGE,);
request_header!(Continuation, CONTINUATION,);
request_header!(IfTags, IF_TAGS,);
request_header!(UserAgent, USER_AGENT,);
request_header!(
    #[doc = "The (friendly) name of the user making the request"]
    User,
    USER,
);
request_header!(Version, VERSION,);

request_query_option!(Prefix, "prefix");
request_query_option!(
    #[doc = "Set delimiter for the request"]
    Delimiter,
    "delimiter"
);
