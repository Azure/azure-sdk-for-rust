use crate::authorization::AccountCredential;

use super::{
    options::{
        constants, table_account_sas_options::TableAccountSasOptions,
        table_account_sas_permission::TableAccountSasPermissions,
        table_account_sas_resource_type::TableAccountSasResourceTypes,
    },
    table_sas_query_parameters::TableSasQueryParameters,
};
use azure_core::SasError;
use chrono::{DateTime, Utc};
use std::str::FromStr;

pub struct TableAccountSasBuilder {
    /// Gets the time at which the shared access signature becomes invalid.
    expires_on: DateTime<Utc>,

    /// Gets the permissions associated with the shared access signature.
    /// The user is restricted to operations allowed by the permissions.
    /// This field must be omitted if it has been specified in an
    /// associated stored access policy.
    permissions: TableAccountSasPermissions,

    /// Gets which resources are accessible via the shared access signature.
    resource_types: TableAccountSasResourceTypes,
}

impl TableAccountSasBuilder {
    pub fn new(
        expires_on: DateTime<Utc>,
        permissions: TableAccountSasPermissions,
        resource_types: TableAccountSasResourceTypes,
    ) -> Self {
        TableAccountSasBuilder {
            expires_on,
            permissions,
            resource_types,
        }
    }

    pub fn new_from_raw_permissions(
        expires_on: DateTime<Utc>,
        raw_permissions: impl Into<String>,
        resource_types: TableAccountSasResourceTypes,
    ) -> Result<Self, SasError> {
        Ok(TableAccountSasBuilder {
            expires_on,
            resource_types,
            permissions: TableAccountSasPermissions::from_str(&raw_permissions.into())?,
        })
    }

    pub fn new_from_sas_uri(sas_uri: http::Uri) -> Result<Self, SasError> {
        todo!()
    }

    pub fn sign(
        &self,
        options: &TableAccountSasOptions,
        credential: &AccountCredential,
    ) -> Result<TableSasQueryParameters, SasError> {
        let permissions: String = self.permissions.into();
        let resource_types: String = self.resource_types.into();

        let expires_on = self
            .expires_on
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        let start_on = options
            .start_time
            .ok_or(SasError::GeneralError("start_time value not found".into()))?
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        let ip_range: String = options.ip_range.map(|c| c.into()).unwrap_or_default();
        let protocol: String = options.protocol.map(|c| c.into()).unwrap_or_default();

        let signature = credential.sign(
            &[
                credential.account(),
                permissions.as_str(),
                constants::TABLE_ACCOUNT_SERVICES_IDENTIFIER,
                resource_types.as_str(),
                start_on.as_str(),
                expires_on.as_str(),
                ip_range.as_str(),
                protocol.as_str(),
                constants::SAS_VERSION,
            ]
            .join("\n"),
        );

        Ok(TableSasQueryParameters::new(
            constants::SAS_VERSION.into(),
            resource_types,
            protocol,
            start_on,
            expires_on,
            self.permissions.into(),
            signature,
            Some(ip_range),
            None,
            None,
        ))
    }
}
