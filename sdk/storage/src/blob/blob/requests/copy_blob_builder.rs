use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::PutBlobResponse;
use crate::core::prelude::*;
use crate::{RehydratePriority, RehydratePriorityOption, RehydratePrioritySupport};
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_source_url: PhantomData<SourceUrlNameSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    source_url: Option<&'a str>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    timeout: Option<u64>,
    if_since_condition: Option<IfSinceCondition>,
    if_source_since_condition: Option<IfSinceCondition>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_source_match_condition: Option<IfMatchCondition<'a>>,
    lease_id: Option<&'a LeaseId>,
    source_lease_id: Option<&'a LeaseId>,
    access_tier: Option<AccessTier>,
    rehydrate_priority: Option<RehydratePriority>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> CopyBlobBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> CopyBlobBuilder<'a, C, No, No, No> {
        CopyBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_source_url: PhantomData {},
            source_url: None,
            metadata: None,
            timeout: None,
            if_since_condition: None,
            if_source_since_condition: None,
            if_match_condition: None,
            if_source_match_condition: None,
            lease_id: None,
            source_lease_id: None,
            access_tier: None,
            rehydrate_priority: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> ClientRequired<'a, C>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, SourceUrlNameSet> ContainerNameRequired<'a>
    for CopyBlobBuilder<'a, C, Yes, BlobNameSet, SourceUrlNameSet>
where
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, SourceUrlNameSet> BlobNameRequired<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, Yes, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> SourceUrlRequired<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn source_url(&self) -> &'a str {
        self.source_url.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> MetadataOption<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> TimeoutOption
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> IfSinceConditionOption
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_since_condition(&self) -> Option<IfSinceCondition> {
        self.if_since_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> IfSourceSinceConditionOption
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_source_since_condition(&self) -> Option<IfSinceCondition> {
        self.if_source_since_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> IfMatchConditionOption<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> IfSourceMatchConditionOption<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_source_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_source_match_condition
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> LeaseIdOption<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> SourceLeaseIdOption<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn source_lease_id(&self) -> Option<&'a LeaseId> {
        self.source_lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> AccessTierOption
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn access_tier(&self) -> Option<AccessTier> {
        self.access_tier
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> RehydratePriorityOption
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn rehydrate_priority(&self) -> Option<RehydratePriority> {
        self.rehydrate_priority
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> ClientRequestIdOption<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, SourceUrlNameSet> ContainerNameSupport<'a>
    for CopyBlobBuilder<'a, C, No, BlobNameSet, SourceUrlNameSet>
where
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, Yes, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, SourceUrlNameSet> BlobNameSupport<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, No, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, Yes, SourceUrlNameSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> SourceUrlSupport<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_source_url(self, source_url: &'a str) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: Some(source_url),
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> MetadataSupport<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: Some(metadata),
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> TimeoutSupport
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: Some(timeout),
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> IfSinceConditionSupport
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_if_since_condition(self, if_since_condition: IfSinceCondition) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: Some(if_since_condition),
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> IfSourceSinceConditionSupport
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_if_source_since_condition(
        self,
        if_source_since_condition: IfSinceCondition,
    ) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: Some(if_source_since_condition),
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> IfMatchConditionSupport<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: Some(if_match_condition),
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> IfSourceMatchConditionSupport<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_if_source_match_condition(
        self,
        if_source_match_condition: IfMatchCondition<'a>,
    ) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: Some(if_source_match_condition),
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> LeaseIdSupport<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: Some(lease_id),
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> SourceLeaseIdSupport<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_source_lease_id(self, source_lease_id: &'a LeaseId) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: Some(source_lease_id),
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> AccessTierSupport
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_access_tier(self, access_tier: AccessTier) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: Some(access_tier),
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> RehydratePrioritySupport
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_rehydrate_priority(self, rehydrate_priority: RehydratePriority) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: Some(rehydrate_priority),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet> ClientRequestIdSupport<'a>
    for CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
    type O = CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        CopyBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_source_url: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            source_url: self.source_url,
            metadata: self.metadata,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            if_source_since_condition: self.if_source_since_condition,
            if_match_condition: self.if_match_condition,
            if_source_match_condition: self.if_source_match_condition,
            lease_id: self.lease_id,
            source_lease_id: self.source_lease_id,
            access_tier: self.access_tier,
            rehydrate_priority: self.rehydrate_priority,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
    CopyBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, SourceUrlNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SourceUrlNameSet: ToAssign,
    C: Client,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, C> CopyBlobBuilder<'a, C, Yes, Yes, Yes> where C: Client {}
