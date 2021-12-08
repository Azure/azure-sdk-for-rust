use crate::authorization::AccountCredential;

use super::{
    options::{
        constants, table_account_sas_optional_options::TableAccountSasOptionalOptions,
        table_account_sas_permission::TableAccountSasPermissions,
        table_account_sas_resource_type::TableAccountSasResourceTypes,
    },
    table_sas_query_parameters::TableSasQueryParameters,
};
use azure_core::SasError;
use chrono::{DateTime, SecondsFormat, Utc};
use std::str::FromStr;

pub struct TableAccountSasBuilder {
    /// The time at which the shared access signature becomes invalid
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
    /// * expires_on - The time at which the shared access signature becomes invalid
    /// * permissions -
    /// * resource_types -
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

    pub fn new_from_sas_uri(sas_uri: &http::Uri) -> Result<Self, SasError> {
        todo!()
    }

    pub fn sign(
        &self,
        credential: &AccountCredential,
        options: &TableAccountSasOptionalOptions,
    ) -> Result<TableSasQueryParameters, SasError> {
        let permissions: String = self.permissions.into();
        let resource_types: String = self.resource_types.into();
        let expires_on = self
            .expires_on
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        let mut parts_to_sign = vec![
            credential.account(),
            permissions.as_str(),
            constants::TABLE_ACCOUNT_SERVICES_IDENTIFIER,
            resource_types.as_str(),
        ];

        let start_on = options
            .start_time
            .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Secs, true));
        if start_on.is_some() {
            parts_to_sign.push(&start_on.as_ref().unwrap());
        } else {
            parts_to_sign.push("");
        }

        parts_to_sign.push(&expires_on);

        let ip: Option<String> = options.ip.map(|ip| ip.into());
        if ip.is_some() {
            parts_to_sign.push(&ip.as_ref().unwrap());
        } else {
            parts_to_sign.push("");
        }

        let protocol: Option<String> = options.protocol.map(|protocol| protocol.into());
        if protocol.is_some() {
            parts_to_sign.push(&protocol.as_ref().unwrap());
        } else {
            parts_to_sign.push("");
        }

        parts_to_sign.push(&constants::SAS_VERSION);
        parts_to_sign.push("");

        let to_sign = parts_to_sign.join("\n");
        println!("{}", to_sign);

        Ok(TableSasQueryParameters::new(
            constants::SAS_VERSION.into(),
            credential.sign(to_sign),
            expires_on,
            permissions,
            resource_types,
            ip,
            protocol,
            start_on,
        ))
    }
}
