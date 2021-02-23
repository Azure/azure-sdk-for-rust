pub use crate::core::blob_sas_builder::BlobSASBuilder;
pub use crate::core::client::HttpHeaderAdder;
pub use crate::core::container_sas_builder::ContainerSASBuilder;
pub use crate::core::{client, Client, CopyId, IPRange};

pub use crate::clients::{AsContainerClient, AsStorageClient, AsTableClient};

pub use crate::core::{
    ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContinuationOption,
    ContinuationSupport, MaxResultsOption, MaxResultsSupport, PrefixOption, PrefixSupport,
    TimeoutOption, TimeoutSupport,
};

pub use crate::core::shared_access_signature::{
    ClientSharedAccessSignature, SasExpirySupport, SasIpSupport, SasPermissions,
    SasPermissionsSupport, SasProtocol, SasProtocolSupport, SasResource, SasResourceSupport,
    SasResourceType, SasResourceTypeSupport, SasService, SasStartSupport, SasVersion,
};

pub use crate::clients::*;
