#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn hsm_security_domain_client(&self) -> hsm_security_domain::Client {
        hsm_security_domain::Client(self.clone())
    }
    pub fn role_assignments_client(&self) -> role_assignments::Client {
        role_assignments::Client(self.clone())
    }
    pub fn role_definitions_client(&self) -> role_definitions::Client {
        role_definitions::Client(self.clone())
    }
}
impl Client {
    #[doc = "List certificates in a specified key vault"]
    #[doc = "The GetCertificates operation returns the set of certificates resources in the specified key vault. This operation requires the certificates/list permission."]
    pub fn get_certificates(&self) -> get_certificates::Builder {
        get_certificates::Builder {
            client: self.clone(),
            maxresults: None,
            include_pending: None,
        }
    }
    #[doc = "Deletes a certificate from a specified key vault."]
    #[doc = "Deletes all versions of a certificate object along with its associated policy. Delete certificate cannot be used to remove individual versions of a certificate object. This operation requires the certificates/delete permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    pub fn delete_certificate(&self, certificate_name: impl Into<String>) -> delete_certificate::Builder {
        delete_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
        }
    }
    #[doc = "Lists the certificate contacts for a specified key vault."]
    #[doc = "The GetCertificateContacts operation returns the set of certificate contact resources in the specified key vault. This operation requires the certificates/managecontacts permission."]
    pub fn get_certificate_contacts(&self) -> get_certificate_contacts::Builder {
        get_certificate_contacts::Builder { client: self.clone() }
    }
    #[doc = "Sets the certificate contacts for the specified key vault."]
    #[doc = "Sets the certificate contacts for the specified key vault. This operation requires the certificates/managecontacts permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `contacts`: The contacts for the key vault certificate."]
    pub fn set_certificate_contacts(&self, contacts: impl Into<models::Contacts>) -> set_certificate_contacts::Builder {
        set_certificate_contacts::Builder {
            client: self.clone(),
            contacts: contacts.into(),
        }
    }
    #[doc = "Deletes the certificate contacts for a specified key vault."]
    #[doc = "Deletes the certificate contacts for a specified key vault certificate. This operation requires the certificates/managecontacts permission."]
    pub fn delete_certificate_contacts(&self) -> delete_certificate_contacts::Builder {
        delete_certificate_contacts::Builder { client: self.clone() }
    }
    #[doc = "List certificate issuers for a specified key vault."]
    #[doc = "The GetCertificateIssuers operation returns the set of certificate issuer resources in the specified key vault. This operation requires the certificates/manageissuers/getissuers permission."]
    pub fn get_certificate_issuers(&self) -> get_certificate_issuers::Builder {
        get_certificate_issuers::Builder {
            client: self.clone(),
            maxresults: None,
        }
    }
    #[doc = "Lists the specified certificate issuer."]
    #[doc = "The GetCertificateIssuer operation returns the specified certificate issuer resources in the specified key vault. This operation requires the certificates/manageissuers/getissuers permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `issuer_name`: The name of the issuer."]
    pub fn get_certificate_issuer(&self, issuer_name: impl Into<String>) -> get_certificate_issuer::Builder {
        get_certificate_issuer::Builder {
            client: self.clone(),
            issuer_name: issuer_name.into(),
        }
    }
    #[doc = "Sets the specified certificate issuer."]
    #[doc = "The SetCertificateIssuer operation adds or updates the specified certificate issuer. This operation requires the certificates/setissuers permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `issuer_name`: The name of the issuer."]
    #[doc = "* `parameter`: Certificate issuer set parameter."]
    pub fn set_certificate_issuer(
        &self,
        issuer_name: impl Into<String>,
        parameter: impl Into<models::CertificateIssuerSetParameters>,
    ) -> set_certificate_issuer::Builder {
        set_certificate_issuer::Builder {
            client: self.clone(),
            issuer_name: issuer_name.into(),
            parameter: parameter.into(),
        }
    }
    #[doc = "Updates the specified certificate issuer."]
    #[doc = "The UpdateCertificateIssuer operation performs an update on the specified certificate issuer entity. This operation requires the certificates/setissuers permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `issuer_name`: The name of the issuer."]
    #[doc = "* `parameter`: Certificate issuer update parameter."]
    pub fn update_certificate_issuer(
        &self,
        issuer_name: impl Into<String>,
        parameter: impl Into<models::CertificateIssuerUpdateParameters>,
    ) -> update_certificate_issuer::Builder {
        update_certificate_issuer::Builder {
            client: self.clone(),
            issuer_name: issuer_name.into(),
            parameter: parameter.into(),
        }
    }
    #[doc = "Deletes the specified certificate issuer."]
    #[doc = "The DeleteCertificateIssuer operation permanently removes the specified certificate issuer from the vault. This operation requires the certificates/manageissuers/deleteissuers permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `issuer_name`: The name of the issuer."]
    pub fn delete_certificate_issuer(&self, issuer_name: impl Into<String>) -> delete_certificate_issuer::Builder {
        delete_certificate_issuer::Builder {
            client: self.clone(),
            issuer_name: issuer_name.into(),
        }
    }
    #[doc = "Creates a new certificate."]
    #[doc = "If this is the first version, the certificate resource is created. This operation requires the certificates/create permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    #[doc = "* `parameters`: The parameters to create a certificate."]
    pub fn create_certificate(
        &self,
        certificate_name: impl Into<String>,
        parameters: impl Into<models::CertificateCreateParameters>,
    ) -> create_certificate::Builder {
        create_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Imports a certificate into a specified key vault."]
    #[doc = "Imports an existing valid certificate, containing a private key, into Azure Key Vault. The certificate to be imported can be in either PFX or PEM format. If the certificate is in PEM format the PEM file must contain the key as well as x509 certificates. This operation requires the certificates/import permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    #[doc = "* `parameters`: The parameters to import the certificate."]
    pub fn import_certificate(
        &self,
        certificate_name: impl Into<String>,
        parameters: impl Into<models::CertificateImportParameters>,
    ) -> import_certificate::Builder {
        import_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "List the versions of a certificate."]
    #[doc = "The GetCertificateVersions operation returns the versions of a certificate in the specified key vault. This operation requires the certificates/list permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    pub fn get_certificate_versions(&self, certificate_name: impl Into<String>) -> get_certificate_versions::Builder {
        get_certificate_versions::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
            maxresults: None,
        }
    }
    #[doc = "Lists the policy for a certificate."]
    #[doc = "The GetCertificatePolicy operation returns the specified certificate policy resources in the specified key vault. This operation requires the certificates/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate in a given key vault."]
    pub fn get_certificate_policy(&self, certificate_name: impl Into<String>) -> get_certificate_policy::Builder {
        get_certificate_policy::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
        }
    }
    #[doc = "Updates the policy for a certificate."]
    #[doc = "Set specified members in the certificate policy. Leave others as null. This operation requires the certificates/update permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate in the given vault."]
    #[doc = "* `certificate_policy`: The policy for the certificate."]
    pub fn update_certificate_policy(
        &self,
        certificate_name: impl Into<String>,
        certificate_policy: impl Into<models::CertificatePolicy>,
    ) -> update_certificate_policy::Builder {
        update_certificate_policy::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
            certificate_policy: certificate_policy.into(),
        }
    }
    #[doc = "Gets information about a certificate."]
    #[doc = "Gets information about a specific certificate. This operation requires the certificates/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate in the given vault."]
    #[doc = "* `certificate_version`: The version of the certificate. This URI fragment is optional. If not specified, the latest version of the certificate is returned."]
    pub fn get_certificate(&self, certificate_name: impl Into<String>, certificate_version: impl Into<String>) -> get_certificate::Builder {
        get_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
            certificate_version: certificate_version.into(),
        }
    }
    #[doc = "Updates the specified attributes associated with the given certificate."]
    #[doc = "The UpdateCertificate operation applies the specified update on the given certificate; the only elements updated are the certificate's attributes. This operation requires the certificates/update permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate in the given key vault."]
    #[doc = "* `certificate_version`: The version of the certificate."]
    #[doc = "* `parameters`: The parameters for certificate update."]
    pub fn update_certificate(
        &self,
        certificate_name: impl Into<String>,
        certificate_version: impl Into<String>,
        parameters: impl Into<models::CertificateUpdateParameters>,
    ) -> update_certificate::Builder {
        update_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
            certificate_version: certificate_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Gets the creation operation of a certificate."]
    #[doc = "Gets the creation operation associated with a specified certificate. This operation requires the certificates/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    pub fn get_certificate_operation(&self, certificate_name: impl Into<String>) -> get_certificate_operation::Builder {
        get_certificate_operation::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
        }
    }
    #[doc = "Updates a certificate operation."]
    #[doc = "Updates a certificate creation operation that is already in progress. This operation requires the certificates/update permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    #[doc = "* `certificate_operation`: The certificate operation response."]
    pub fn update_certificate_operation(
        &self,
        certificate_name: impl Into<String>,
        certificate_operation: impl Into<models::CertificateOperationUpdateParameter>,
    ) -> update_certificate_operation::Builder {
        update_certificate_operation::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
            certificate_operation: certificate_operation.into(),
        }
    }
    #[doc = "Deletes the creation operation for a specific certificate."]
    #[doc = "Deletes the creation operation for a specified certificate that is in the process of being created. The certificate is no longer created. This operation requires the certificates/update permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    pub fn delete_certificate_operation(&self, certificate_name: impl Into<String>) -> delete_certificate_operation::Builder {
        delete_certificate_operation::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
        }
    }
    #[doc = "Merges a certificate or a certificate chain with a key pair existing on the server."]
    #[doc = "The MergeCertificate operation performs the merging of a certificate or certificate chain with a key pair currently available in the service. This operation requires the certificates/create permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    #[doc = "* `parameters`: The parameters to merge certificate."]
    pub fn merge_certificate(
        &self,
        certificate_name: impl Into<String>,
        parameters: impl Into<models::CertificateMergeParameters>,
    ) -> merge_certificate::Builder {
        merge_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Backs up the specified certificate."]
    #[doc = "Requests that a backup of the specified certificate be downloaded to the client. All versions of the certificate will be downloaded. This operation requires the certificates/backup permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate."]
    pub fn backup_certificate(&self, certificate_name: impl Into<String>) -> backup_certificate::Builder {
        backup_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
        }
    }
    #[doc = "Restores a backed up certificate to a vault."]
    #[doc = "Restores a backed up certificate, and all its versions, to a vault. This operation requires the certificates/restore permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `parameters`: The parameters to restore the certificate."]
    pub fn restore_certificate(&self, parameters: impl Into<models::CertificateRestoreParameters>) -> restore_certificate::Builder {
        restore_certificate::Builder {
            client: self.clone(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Lists the deleted certificates in the specified vault currently available for recovery."]
    #[doc = "The GetDeletedCertificates operation retrieves the certificates in the current vault which are in a deleted state and ready for recovery or purging. This operation includes deletion-specific information. This operation requires the certificates/get/list permission. This operation can only be enabled on soft-delete enabled vaults."]
    pub fn get_deleted_certificates(&self) -> get_deleted_certificates::Builder {
        get_deleted_certificates::Builder {
            client: self.clone(),
            maxresults: None,
            include_pending: None,
        }
    }
    #[doc = "Retrieves information about the specified deleted certificate."]
    #[doc = "The GetDeletedCertificate operation retrieves the deleted certificate information plus its attributes, such as retention interval, scheduled permanent deletion and the current deletion recovery level. This operation requires the certificates/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate"]
    pub fn get_deleted_certificate(&self, certificate_name: impl Into<String>) -> get_deleted_certificate::Builder {
        get_deleted_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
        }
    }
    #[doc = "Permanently deletes the specified deleted certificate."]
    #[doc = "The PurgeDeletedCertificate operation performs an irreversible deletion of the specified certificate, without possibility for recovery. The operation is not available if the recovery level does not specify 'Purgeable'. This operation requires the certificate/purge permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the certificate"]
    pub fn purge_deleted_certificate(&self, certificate_name: impl Into<String>) -> purge_deleted_certificate::Builder {
        purge_deleted_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
        }
    }
    #[doc = "Recovers the deleted certificate back to its current version under /certificates."]
    #[doc = "The RecoverDeletedCertificate operation performs the reversal of the Delete operation. The operation is applicable in vaults enabled for soft-delete, and must be issued during the retention interval (available in the deleted certificate's attributes). This operation requires the certificates/recover permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `certificate_name`: The name of the deleted certificate"]
    pub fn recover_deleted_certificate(&self, certificate_name: impl Into<String>) -> recover_deleted_certificate::Builder {
        recover_deleted_certificate::Builder {
            client: self.clone(),
            certificate_name: certificate_name.into(),
        }
    }
    #[doc = "Creates a new key, stores it, then returns key parameters and attributes to the client."]
    #[doc = "The create key operation can be used to create any key type in Azure Key Vault. If the named key already exists, Azure Key Vault creates a new version of the key. It requires the keys/create permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name for the new key. The system will generate the version name for the new key."]
    #[doc = "* `parameters`: The parameters to create a key."]
    pub fn create_key(&self, key_name: impl Into<String>, parameters: impl Into<models::KeyCreateParameters>) -> create_key::Builder {
        create_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Imports an externally created key, stores it, and returns key parameters and attributes to the client."]
    #[doc = "The import key operation may be used to import any key type into an Azure Key Vault. If the named key already exists, Azure Key Vault creates a new version of the key. This operation requires the keys/import permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: Name for the imported key."]
    #[doc = "* `parameters`: The parameters to import a key."]
    pub fn import_key(&self, key_name: impl Into<String>, parameters: impl Into<models::KeyImportParameters>) -> import_key::Builder {
        import_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Deletes a key of any type from storage in Azure Key Vault."]
    #[doc = "The delete key operation cannot be used to remove individual versions of a key. This operation removes the cryptographic material associated with the key, which means the key is not usable for Sign/Verify, Wrap/Unwrap or Encrypt/Decrypt operations. This operation requires the keys/delete permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key to delete."]
    pub fn delete_key(&self, key_name: impl Into<String>) -> delete_key::Builder {
        delete_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
        }
    }
    #[doc = "Gets the public part of a stored key."]
    #[doc = "The get key operation is applicable to all key types. If the requested key is symmetric, then no key material is released in the response. This operation requires the keys/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key to get."]
    #[doc = "* `key_version`: Adding the version parameter retrieves a specific version of a key. This URI fragment is optional. If not specified, the latest version of the key is returned."]
    pub fn get_key(&self, key_name: impl Into<String>, key_version: impl Into<String>) -> get_key::Builder {
        get_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            key_version: key_version.into(),
        }
    }
    #[doc = "The update key operation changes specified attributes of a stored key and can be applied to any key type and key version stored in Azure Key Vault."]
    #[doc = "In order to perform this operation, the key must already exist in the Key Vault. Note: The cryptographic material of a key itself cannot be changed. This operation requires the keys/update permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of key to update."]
    #[doc = "* `key_version`: The version of the key to update."]
    #[doc = "* `parameters`: The parameters of the key to update."]
    pub fn update_key(
        &self,
        key_name: impl Into<String>,
        key_version: impl Into<String>,
        parameters: impl Into<models::KeyUpdateParameters>,
    ) -> update_key::Builder {
        update_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            key_version: key_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Retrieves a list of individual key versions with the same key name."]
    #[doc = "The full key identifier, attributes, and tags are provided in the response. This operation requires the keys/list permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    pub fn get_key_versions(&self, key_name: impl Into<String>) -> get_key_versions::Builder {
        get_key_versions::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            maxresults: None,
        }
    }
    #[doc = "List keys in the specified vault."]
    #[doc = "Retrieves a list of the keys in the Key Vault as JSON Web Key structures that contain the public part of a stored key. The LIST operation is applicable to all key types, however only the base key identifier, attributes, and tags are provided in the response. Individual versions of a key are not listed in the response. This operation requires the keys/list permission."]
    pub fn get_keys(&self) -> get_keys::Builder {
        get_keys::Builder {
            client: self.clone(),
            maxresults: None,
        }
    }
    #[doc = "Requests that a backup of the specified key be downloaded to the client."]
    #[doc = "The Key Backup operation exports a key from Azure Key Vault in a protected form. Note that this operation does NOT return key material in a form that can be used outside the Azure Key Vault system, the returned key material is either protected to a Azure Key Vault HSM or to Azure Key Vault itself. The intent of this operation is to allow a client to GENERATE a key in one Azure Key Vault instance, BACKUP the key, and then RESTORE it into another Azure Key Vault instance. The BACKUP operation may be used to export, in protected form, any key type from Azure Key Vault. Individual versions of a key cannot be backed up. BACKUP / RESTORE can be performed within geographical boundaries only; meaning that a BACKUP from one geographical area cannot be restored to another geographical area. For example, a backup from the US geographical area cannot be restored in an EU geographical area. This operation requires the key/backup permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    pub fn backup_key(&self, key_name: impl Into<String>) -> backup_key::Builder {
        backup_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
        }
    }
    #[doc = "Restores a backed up key to a vault."]
    #[doc = "Imports a previously backed up key into Azure Key Vault, restoring the key, its key identifier, attributes and access control policies. The RESTORE operation may be used to import a previously backed up key. Individual versions of a key cannot be restored. The key is restored in its entirety with the same key name as it had when it was backed up. If the key name is not available in the target Key Vault, the RESTORE operation will be rejected. While the key name is retained during restore, the final key identifier will change if the key is restored to a different vault. Restore will restore all versions and preserve version identifiers. The RESTORE operation is subject to security constraints: The target Key Vault must be owned by the same Microsoft Azure Subscription as the source Key Vault The user must have RESTORE permission in the target Key Vault. This operation requires the keys/restore permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `parameters`: The parameters to restore the key."]
    pub fn restore_key(&self, parameters: impl Into<models::KeyRestoreParameters>) -> restore_key::Builder {
        restore_key::Builder {
            client: self.clone(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Encrypts an arbitrary sequence of bytes using an encryption key that is stored in a key vault."]
    #[doc = "The ENCRYPT operation encrypts an arbitrary sequence of bytes using an encryption key that is stored in Azure Key Vault. Note that the ENCRYPT operation only supports a single block of data, the size of which is dependent on the target key and the encryption algorithm to be used. The ENCRYPT operation is only strictly necessary for symmetric keys stored in Azure Key Vault since protection with an asymmetric key can be performed using public portion of the key. This operation is supported for asymmetric keys as a convenience for callers that have a key-reference but do not have access to the public key material. This operation requires the keys/encrypt permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    #[doc = "* `key_version`: The version of the key."]
    #[doc = "* `parameters`: The parameters for the encryption operation."]
    pub fn encrypt(
        &self,
        key_name: impl Into<String>,
        key_version: impl Into<String>,
        parameters: impl Into<models::KeyOperationsParameters>,
    ) -> encrypt::Builder {
        encrypt::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            key_version: key_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Decrypts a single block of encrypted data."]
    #[doc = "The DECRYPT operation decrypts a well-formed block of ciphertext using the target encryption key and specified algorithm. This operation is the reverse of the ENCRYPT operation; only a single block of data may be decrypted, the size of this block is dependent on the target key and the algorithm to be used. The DECRYPT operation applies to asymmetric and symmetric keys stored in Azure Key Vault since it uses the private portion of the key. This operation requires the keys/decrypt permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    #[doc = "* `key_version`: The version of the key."]
    #[doc = "* `parameters`: The parameters for the decryption operation."]
    pub fn decrypt(
        &self,
        key_name: impl Into<String>,
        key_version: impl Into<String>,
        parameters: impl Into<models::KeyOperationsParameters>,
    ) -> decrypt::Builder {
        decrypt::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            key_version: key_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Creates a signature from a digest using the specified key."]
    #[doc = "The SIGN operation is applicable to asymmetric and symmetric keys stored in Azure Key Vault since this operation uses the private portion of the key. This operation requires the keys/sign permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    #[doc = "* `key_version`: The version of the key."]
    #[doc = "* `parameters`: The parameters for the signing operation."]
    pub fn sign(
        &self,
        key_name: impl Into<String>,
        key_version: impl Into<String>,
        parameters: impl Into<models::KeySignParameters>,
    ) -> sign::Builder {
        sign::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            key_version: key_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Verifies a signature using a specified key."]
    #[doc = "The VERIFY operation is applicable to symmetric keys stored in Azure Key Vault. VERIFY is not strictly necessary for asymmetric keys stored in Azure Key Vault since signature verification can be performed using the public portion of the key but this operation is supported as a convenience for callers that only have a key-reference and not the public portion of the key. This operation requires the keys/verify permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    #[doc = "* `key_version`: The version of the key."]
    #[doc = "* `parameters`: The parameters for verify operations."]
    pub fn verify(
        &self,
        key_name: impl Into<String>,
        key_version: impl Into<String>,
        parameters: impl Into<models::KeyVerifyParameters>,
    ) -> verify::Builder {
        verify::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            key_version: key_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Wraps a symmetric key using a specified key."]
    #[doc = "The WRAP operation supports encryption of a symmetric key using a key encryption key that has previously been stored in an Azure Key Vault. The WRAP operation is only strictly necessary for symmetric keys stored in Azure Key Vault since protection with an asymmetric key can be performed using the public portion of the key. This operation is supported for asymmetric keys as a convenience for callers that have a key-reference but do not have access to the public key material. This operation requires the keys/wrapKey permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    #[doc = "* `key_version`: The version of the key."]
    #[doc = "* `parameters`: The parameters for wrap operation."]
    pub fn wrap_key(
        &self,
        key_name: impl Into<String>,
        key_version: impl Into<String>,
        parameters: impl Into<models::KeyOperationsParameters>,
    ) -> wrap_key::Builder {
        wrap_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            key_version: key_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Unwraps a symmetric key using the specified key that was initially used for wrapping that key."]
    #[doc = "The UNWRAP operation supports decryption of a symmetric key using the target key encryption key. This operation is the reverse of the WRAP operation. The UNWRAP operation applies to asymmetric and symmetric keys stored in Azure Key Vault since it uses the private portion of the key. This operation requires the keys/unwrapKey permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    #[doc = "* `key_version`: The version of the key."]
    #[doc = "* `parameters`: The parameters for the key operation."]
    pub fn unwrap_key(
        &self,
        key_name: impl Into<String>,
        key_version: impl Into<String>,
        parameters: impl Into<models::KeyOperationsParameters>,
    ) -> unwrap_key::Builder {
        unwrap_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            key_version: key_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Lists the deleted keys in the specified vault."]
    #[doc = "Retrieves a list of the keys in the Key Vault as JSON Web Key structures that contain the public part of a deleted key. This operation includes deletion-specific information. The Get Deleted Keys operation is applicable for vaults enabled for soft-delete. While the operation can be invoked on any vault, it will return an error if invoked on a non soft-delete enabled vault. This operation requires the keys/list permission."]
    pub fn get_deleted_keys(&self) -> get_deleted_keys::Builder {
        get_deleted_keys::Builder {
            client: self.clone(),
            maxresults: None,
        }
    }
    #[doc = "Gets the public part of a deleted key."]
    #[doc = "The Get Deleted Key operation is applicable for soft-delete enabled vaults. While the operation can be invoked on any vault, it will return an error if invoked on a non soft-delete enabled vault. This operation requires the keys/get permission. "]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key."]
    pub fn get_deleted_key(&self, key_name: impl Into<String>) -> get_deleted_key::Builder {
        get_deleted_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
        }
    }
    #[doc = "Permanently deletes the specified key."]
    #[doc = "The Purge Deleted Key operation is applicable for soft-delete enabled vaults. While the operation can be invoked on any vault, it will return an error if invoked on a non soft-delete enabled vault. This operation requires the keys/purge permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key"]
    pub fn purge_deleted_key(&self, key_name: impl Into<String>) -> purge_deleted_key::Builder {
        purge_deleted_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
        }
    }
    #[doc = "Recovers the deleted key to its latest version."]
    #[doc = "The Recover Deleted Key operation is applicable for deleted keys in soft-delete enabled vaults. It recovers the deleted key back to its latest version under /keys. An attempt to recover an non-deleted key will return an error. Consider this the inverse of the delete operation on soft-delete enabled vaults. This operation requires the keys/recover permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the deleted key."]
    pub fn recover_deleted_key(&self, key_name: impl Into<String>) -> recover_deleted_key::Builder {
        recover_deleted_key::Builder {
            client: self.clone(),
            key_name: key_name.into(),
        }
    }
    #[doc = "Sets a secret in a specified key vault."]
    #[doc = " The SET operation adds a secret to the Azure Key Vault. If the named secret already exists, Azure Key Vault creates a new version of that secret. This operation requires the secrets/set permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the secret."]
    #[doc = "* `parameters`: The parameters for setting the secret."]
    pub fn set_secret(&self, secret_name: impl Into<String>, parameters: impl Into<models::SecretSetParameters>) -> set_secret::Builder {
        set_secret::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Deletes a secret from a specified key vault."]
    #[doc = "The DELETE operation applies to any secret stored in Azure Key Vault. DELETE cannot be applied to an individual version of a secret. This operation requires the secrets/delete permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the secret."]
    pub fn delete_secret(&self, secret_name: impl Into<String>) -> delete_secret::Builder {
        delete_secret::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
        }
    }
    #[doc = "Get a specified secret from a given key vault."]
    #[doc = "The GET operation is applicable to any secret stored in Azure Key Vault. This operation requires the secrets/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the secret."]
    #[doc = "* `secret_version`: The version of the secret. This URI fragment is optional. If not specified, the latest version of the secret is returned."]
    pub fn get_secret(&self, secret_name: impl Into<String>, secret_version: impl Into<String>) -> get_secret::Builder {
        get_secret::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
            secret_version: secret_version.into(),
        }
    }
    #[doc = "Updates the attributes associated with a specified secret in a given key vault."]
    #[doc = "The UPDATE operation changes specified attributes of an existing stored secret. Attributes that are not specified in the request are left unchanged. The value of a secret itself cannot be changed. This operation requires the secrets/set permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the secret."]
    #[doc = "* `secret_version`: The version of the secret."]
    #[doc = "* `parameters`: The parameters for update secret operation."]
    pub fn update_secret(
        &self,
        secret_name: impl Into<String>,
        secret_version: impl Into<String>,
        parameters: impl Into<models::SecretUpdateParameters>,
    ) -> update_secret::Builder {
        update_secret::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
            secret_version: secret_version.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "List secrets in a specified key vault."]
    #[doc = "The Get Secrets operation is applicable to the entire vault. However, only the base secret identifier and its attributes are provided in the response. Individual secret versions are not listed in the response. This operation requires the secrets/list permission."]
    pub fn get_secrets(&self) -> get_secrets::Builder {
        get_secrets::Builder {
            client: self.clone(),
            maxresults: None,
        }
    }
    #[doc = "List all versions of the specified secret."]
    #[doc = "The full secret identifier and attributes are provided in the response. No values are returned for the secrets. This operations requires the secrets/list permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the secret."]
    pub fn get_secret_versions(&self, secret_name: impl Into<String>) -> get_secret_versions::Builder {
        get_secret_versions::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
            maxresults: None,
        }
    }
    #[doc = "Lists deleted secrets for the specified vault."]
    #[doc = "The Get Deleted Secrets operation returns the secrets that have been deleted for a vault enabled for soft-delete. This operation requires the secrets/list permission."]
    pub fn get_deleted_secrets(&self) -> get_deleted_secrets::Builder {
        get_deleted_secrets::Builder {
            client: self.clone(),
            maxresults: None,
        }
    }
    #[doc = "Gets the specified deleted secret."]
    #[doc = "The Get Deleted Secret operation returns the specified deleted secret along with its attributes. This operation requires the secrets/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the secret."]
    pub fn get_deleted_secret(&self, secret_name: impl Into<String>) -> get_deleted_secret::Builder {
        get_deleted_secret::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
        }
    }
    #[doc = "Permanently deletes the specified secret."]
    #[doc = "The purge deleted secret operation removes the secret permanently, without the possibility of recovery. This operation can only be enabled on a soft-delete enabled vault. This operation requires the secrets/purge permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the secret."]
    pub fn purge_deleted_secret(&self, secret_name: impl Into<String>) -> purge_deleted_secret::Builder {
        purge_deleted_secret::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
        }
    }
    #[doc = "Recovers the deleted secret to the latest version."]
    #[doc = "Recovers the deleted secret in the specified vault. This operation can only be performed on a soft-delete enabled vault. This operation requires the secrets/recover permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the deleted secret."]
    pub fn recover_deleted_secret(&self, secret_name: impl Into<String>) -> recover_deleted_secret::Builder {
        recover_deleted_secret::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
        }
    }
    #[doc = "Backs up the specified secret."]
    #[doc = "Requests that a backup of the specified secret be downloaded to the client. All versions of the secret will be downloaded. This operation requires the secrets/backup permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `secret_name`: The name of the secret."]
    pub fn backup_secret(&self, secret_name: impl Into<String>) -> backup_secret::Builder {
        backup_secret::Builder {
            client: self.clone(),
            secret_name: secret_name.into(),
        }
    }
    #[doc = "Restores a backed up secret to a vault."]
    #[doc = "Restores a backed up secret, and all its versions, to a vault. This operation requires the secrets/restore permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `parameters`: The parameters to restore the secret."]
    pub fn restore_secret(&self, parameters: impl Into<models::SecretRestoreParameters>) -> restore_secret::Builder {
        restore_secret::Builder {
            client: self.clone(),
            parameters: parameters.into(),
        }
    }
    #[doc = "List storage accounts managed by the specified key vault. This operation requires the storage/list permission."]
    pub fn get_storage_accounts(&self) -> get_storage_accounts::Builder {
        get_storage_accounts::Builder {
            client: self.clone(),
            maxresults: None,
        }
    }
    #[doc = "Lists deleted storage accounts for the specified vault."]
    #[doc = "The Get Deleted Storage Accounts operation returns the storage accounts that have been deleted for a vault enabled for soft-delete. This operation requires the storage/list permission."]
    pub fn get_deleted_storage_accounts(&self) -> get_deleted_storage_accounts::Builder {
        get_deleted_storage_accounts::Builder {
            client: self.clone(),
            maxresults: None,
        }
    }
    #[doc = "Gets the specified deleted storage account."]
    #[doc = "The Get Deleted Storage Account operation returns the specified deleted storage account along with its attributes. This operation requires the storage/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    pub fn get_deleted_storage_account(&self, storage_account_name: impl Into<String>) -> get_deleted_storage_account::Builder {
        get_deleted_storage_account::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
        }
    }
    #[doc = "Permanently deletes the specified storage account."]
    #[doc = "The purge deleted storage account operation removes the secret permanently, without the possibility of recovery. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/purge permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    pub fn purge_deleted_storage_account(&self, storage_account_name: impl Into<String>) -> purge_deleted_storage_account::Builder {
        purge_deleted_storage_account::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
        }
    }
    #[doc = "Recovers the deleted storage account."]
    #[doc = "Recovers the deleted storage account in the specified vault. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/recover permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    pub fn recover_deleted_storage_account(&self, storage_account_name: impl Into<String>) -> recover_deleted_storage_account::Builder {
        recover_deleted_storage_account::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
        }
    }
    #[doc = "Backs up the specified storage account."]
    #[doc = "Requests that a backup of the specified storage account be downloaded to the client. This operation requires the storage/backup permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    pub fn backup_storage_account(&self, storage_account_name: impl Into<String>) -> backup_storage_account::Builder {
        backup_storage_account::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
        }
    }
    #[doc = "Restores a backed up storage account to a vault."]
    #[doc = "Restores a backed up storage account to a vault. This operation requires the storage/restore permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `parameters`: The parameters to restore the storage account."]
    pub fn restore_storage_account(&self, parameters: impl Into<models::StorageRestoreParameters>) -> restore_storage_account::Builder {
        restore_storage_account::Builder {
            client: self.clone(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Gets information about a specified storage account. This operation requires the storage/get permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    pub fn get_storage_account(&self, storage_account_name: impl Into<String>) -> get_storage_account::Builder {
        get_storage_account::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
        }
    }
    #[doc = "Creates or updates a new storage account. This operation requires the storage/set permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `parameters`: The parameters to create a storage account."]
    pub fn set_storage_account(
        &self,
        storage_account_name: impl Into<String>,
        parameters: impl Into<models::StorageAccountCreateParameters>,
    ) -> set_storage_account::Builder {
        set_storage_account::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Updates the specified attributes associated with the given storage account. This operation requires the storage/set/update permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `parameters`: The parameters to update a storage account."]
    pub fn update_storage_account(
        &self,
        storage_account_name: impl Into<String>,
        parameters: impl Into<models::StorageAccountUpdateParameters>,
    ) -> update_storage_account::Builder {
        update_storage_account::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Deletes a storage account. This operation requires the storage/delete permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    pub fn delete_storage_account(&self, storage_account_name: impl Into<String>) -> delete_storage_account::Builder {
        delete_storage_account::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
        }
    }
    #[doc = "Regenerates the specified key value for the given storage account. This operation requires the storage/regeneratekey permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `parameters`: The parameters to regenerate storage account key."]
    pub fn regenerate_storage_account_key(
        &self,
        storage_account_name: impl Into<String>,
        parameters: impl Into<models::StorageAccountRegenerteKeyParameters>,
    ) -> regenerate_storage_account_key::Builder {
        regenerate_storage_account_key::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "List storage SAS definitions for the given storage account. This operation requires the storage/listsas permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    pub fn get_sas_definitions(&self, storage_account_name: impl Into<String>) -> get_sas_definitions::Builder {
        get_sas_definitions::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            maxresults: None,
        }
    }
    #[doc = "Lists deleted SAS definitions for the specified vault and storage account."]
    #[doc = "The Get Deleted Sas Definitions operation returns the SAS definitions that have been deleted for a vault enabled for soft-delete. This operation requires the storage/listsas permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    pub fn get_deleted_sas_definitions(&self, storage_account_name: impl Into<String>) -> get_deleted_sas_definitions::Builder {
        get_deleted_sas_definitions::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            maxresults: None,
        }
    }
    #[doc = "Gets the specified deleted sas definition."]
    #[doc = "The Get Deleted SAS Definition operation returns the specified deleted SAS definition along with its attributes. This operation requires the storage/getsas permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `sas_definition_name`: The name of the SAS definition."]
    pub fn get_deleted_sas_definition(
        &self,
        storage_account_name: impl Into<String>,
        sas_definition_name: impl Into<String>,
    ) -> get_deleted_sas_definition::Builder {
        get_deleted_sas_definition::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            sas_definition_name: sas_definition_name.into(),
        }
    }
    #[doc = "Recovers the deleted SAS definition."]
    #[doc = "Recovers the deleted SAS definition for the specified storage account. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/recover permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `sas_definition_name`: The name of the SAS definition."]
    pub fn recover_deleted_sas_definition(
        &self,
        storage_account_name: impl Into<String>,
        sas_definition_name: impl Into<String>,
    ) -> recover_deleted_sas_definition::Builder {
        recover_deleted_sas_definition::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            sas_definition_name: sas_definition_name.into(),
        }
    }
    #[doc = "Gets information about a SAS definition for the specified storage account. This operation requires the storage/getsas permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `sas_definition_name`: The name of the SAS definition."]
    pub fn get_sas_definition(
        &self,
        storage_account_name: impl Into<String>,
        sas_definition_name: impl Into<String>,
    ) -> get_sas_definition::Builder {
        get_sas_definition::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            sas_definition_name: sas_definition_name.into(),
        }
    }
    #[doc = "Creates or updates a new SAS definition for the specified storage account. This operation requires the storage/setsas permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `sas_definition_name`: The name of the SAS definition."]
    #[doc = "* `parameters`: The parameters to create a SAS definition."]
    pub fn set_sas_definition(
        &self,
        storage_account_name: impl Into<String>,
        sas_definition_name: impl Into<String>,
        parameters: impl Into<models::SasDefinitionCreateParameters>,
    ) -> set_sas_definition::Builder {
        set_sas_definition::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            sas_definition_name: sas_definition_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Updates the specified attributes associated with the given SAS definition. This operation requires the storage/setsas permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `sas_definition_name`: The name of the SAS definition."]
    #[doc = "* `parameters`: The parameters to update a SAS definition."]
    pub fn update_sas_definition(
        &self,
        storage_account_name: impl Into<String>,
        sas_definition_name: impl Into<String>,
        parameters: impl Into<models::SasDefinitionUpdateParameters>,
    ) -> update_sas_definition::Builder {
        update_sas_definition::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            sas_definition_name: sas_definition_name.into(),
            parameters: parameters.into(),
        }
    }
    #[doc = "Deletes a SAS definition from a specified storage account. This operation requires the storage/deletesas permission."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `storage_account_name`: The name of the storage account."]
    #[doc = "* `sas_definition_name`: The name of the SAS definition."]
    pub fn delete_sas_definition(
        &self,
        storage_account_name: impl Into<String>,
        sas_definition_name: impl Into<String>,
    ) -> delete_sas_definition::Builder {
        delete_sas_definition::Builder {
            client: self.clone(),
            storage_account_name: storage_account_name.into(),
            sas_definition_name: sas_definition_name.into(),
        }
    }
    #[doc = "Creates a full backup using a user-provided SAS token to an Azure blob storage container."]
    pub fn full_backup(&self) -> full_backup::Builder {
        full_backup::Builder {
            client: self.clone(),
            azure_storage_blob_container_uri: None,
        }
    }
    #[doc = "Returns the status of full backup operation"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `job_id`: The id returned as part of the backup request"]
    pub fn full_backup_status(&self, job_id: impl Into<String>) -> full_backup_status::Builder {
        full_backup_status::Builder {
            client: self.clone(),
            job_id: job_id.into(),
        }
    }
    #[doc = "Restores all key materials using the SAS token pointing to a previously stored Azure Blob storage backup folder"]
    pub fn full_restore_operation(&self) -> full_restore_operation::Builder {
        full_restore_operation::Builder {
            client: self.clone(),
            restore_blob_details: None,
        }
    }
    #[doc = "Returns the status of restore operation"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `job_id`: The Job Id returned part of the restore operation"]
    pub fn restore_status(&self, job_id: impl Into<String>) -> restore_status::Builder {
        restore_status::Builder {
            client: self.clone(),
            job_id: job_id.into(),
        }
    }
    #[doc = "Restores all key versions of a given key using user supplied SAS token pointing to a previously stored Azure Blob storage backup folder"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key_name`: The name of the key to be restored from the user supplied backup"]
    pub fn selective_key_restore_operation(&self, key_name: impl Into<String>) -> selective_key_restore_operation::Builder {
        selective_key_restore_operation::Builder {
            client: self.clone(),
            key_name: key_name.into(),
            restore_blob_details: None,
        }
    }
}
pub mod get_certificates {
    use super::models;
    type Response = models::CertificateListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
        pub(crate) include_pending: Option<bool>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        #[doc = "Specifies whether to include certificates which are not completely provisioned."]
        pub fn include_pending(mut self, include_pending: bool) -> Self {
            self.include_pending = Some(include_pending);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/certificates", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            if let Some(include_pending) = &this.include_pending {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair("includePending", &include_pending.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod delete_certificate {
    use super::models;
    type Response = models::DeletedCertificateBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/{}", this.client.endpoint(), &this.certificate_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedCertificateBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_certificate_contacts {
    use super::models;
    type Response = models::Contacts;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/contacts", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::Contacts = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod set_certificate_contacts {
    use super::models;
    type Response = models::Contacts;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) contacts: models::Contacts,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/contacts", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.contacts)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::Contacts = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod delete_certificate_contacts {
    use super::models;
    type Response = models::Contacts;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/contacts", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::Contacts = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_certificate_issuers {
    use super::models;
    type Response = models::CertificateIssuerListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/certificates/issuers", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateIssuerListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_certificate_issuer {
    use super::models;
    type Response = models::IssuerBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) issuer_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/issuers/{}", this.client.endpoint(), &this.issuer_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::IssuerBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod set_certificate_issuer {
    use super::models;
    type Response = models::IssuerBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) issuer_name: String,
        pub(crate) parameter: models::CertificateIssuerSetParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/issuers/{}", this.client.endpoint(), &this.issuer_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameter)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::IssuerBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod update_certificate_issuer {
    use super::models;
    type Response = models::IssuerBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) issuer_name: String,
        pub(crate) parameter: models::CertificateIssuerUpdateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/issuers/{}", this.client.endpoint(), &this.issuer_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameter)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::IssuerBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod delete_certificate_issuer {
    use super::models;
    type Response = models::IssuerBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) issuer_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/issuers/{}", this.client.endpoint(), &this.issuer_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::IssuerBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod create_certificate {
    use super::models;
    type Response = models::CertificateOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
        pub(crate) parameters: models::CertificateCreateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/create",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Accepted => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod import_certificate {
    use super::models;
    type Response = models::CertificateBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
        pub(crate) parameters: models::CertificateImportParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/import",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_certificate_versions {
    use super::models;
    type Response = models::CertificateListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/versions",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_certificate_policy {
    use super::models;
    type Response = models::CertificatePolicy;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/policy",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificatePolicy = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod update_certificate_policy {
    use super::models;
    type Response = models::CertificatePolicy;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
        pub(crate) certificate_policy: models::CertificatePolicy,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/policy",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.certificate_policy)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificatePolicy = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_certificate {
    use super::models;
    type Response = models::CertificateBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
        pub(crate) certificate_version: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/{}",
                        this.client.endpoint(),
                        &this.certificate_name,
                        &this.certificate_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod update_certificate {
    use super::models;
    type Response = models::CertificateBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
        pub(crate) certificate_version: String,
        pub(crate) parameters: models::CertificateUpdateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/{}",
                        this.client.endpoint(),
                        &this.certificate_name,
                        &this.certificate_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_certificate_operation {
    use super::models;
    type Response = models::CertificateOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/pending",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod update_certificate_operation {
    use super::models;
    type Response = models::CertificateOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
        pub(crate) certificate_operation: models::CertificateOperationUpdateParameter,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/pending",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.certificate_operation)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod delete_certificate_operation {
    use super::models;
    type Response = models::CertificateOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/pending",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod merge_certificate {
    use super::models;
    type Response = models::CertificateBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
        pub(crate) parameters: models::CertificateMergeParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/pending/merge",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Created => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod backup_certificate {
    use super::models;
    type Response = models::BackupCertificateResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/certificates/{}/backup",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::BackupCertificateResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod restore_certificate {
    use super::models;
    type Response = models::CertificateBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) parameters: models::CertificateRestoreParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/certificates/restore", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_deleted_certificates {
    use super::models;
    type Response = models::DeletedCertificateListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
        pub(crate) include_pending: Option<bool>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        #[doc = "Specifies whether to include certificates which are not completely provisioned."]
        pub fn include_pending(mut self, include_pending: bool) -> Self {
            self.include_pending = Some(include_pending);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/deletedcertificates", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            if let Some(include_pending) = &this.include_pending {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair("includePending", &include_pending.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedCertificateListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_deleted_certificate {
    use super::models;
    type Response = models::DeletedCertificateBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/deletedcertificates/{}",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedCertificateBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod purge_deleted_certificate {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/deletedcertificates/{}",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::NoContent => Ok(()),
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod recover_deleted_certificate {
    use super::models;
    type Response = models::CertificateBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) certificate_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/deletedcertificates/{}/recover",
                        this.client.endpoint(),
                        &this.certificate_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CertificateBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod create_key {
    use super::models;
    type Response = models::KeyBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) parameters: models::KeyCreateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/keys/{}/create", this.client.endpoint(), &this.key_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod import_key {
    use super::models;
    type Response = models::KeyBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) parameters: models::KeyImportParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/keys/{}", this.client.endpoint(), &this.key_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod delete_key {
    use super::models;
    type Response = models::DeletedKeyBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/keys/{}", this.client.endpoint(), &this.key_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedKeyBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_key {
    use super::models;
    type Response = models::KeyBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) key_version: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/keys/{}/{}", this.client.endpoint(), &this.key_name, &this.key_version))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod update_key {
    use super::models;
    type Response = models::KeyBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) key_version: String,
        pub(crate) parameters: models::KeyUpdateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/keys/{}/{}", this.client.endpoint(), &this.key_name, &this.key_version))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_key_versions {
    use super::models;
    type Response = models::KeyListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/keys/{}/versions", this.client.endpoint(), &this.key_name))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_keys {
    use super::models;
    type Response = models::KeyListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/keys", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod backup_key {
    use super::models;
    type Response = models::BackupKeyResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/keys/{}/backup", this.client.endpoint(), &this.key_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::BackupKeyResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod restore_key {
    use super::models;
    type Response = models::KeyBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) parameters: models::KeyRestoreParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/keys/restore", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod encrypt {
    use super::models;
    type Response = models::KeyOperationResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) key_version: String,
        pub(crate) parameters: models::KeyOperationsParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/keys/{}/{}/encrypt",
                        this.client.endpoint(),
                        &this.key_name,
                        &this.key_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyOperationResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod decrypt {
    use super::models;
    type Response = models::KeyOperationResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) key_version: String,
        pub(crate) parameters: models::KeyOperationsParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/keys/{}/{}/decrypt",
                        this.client.endpoint(),
                        &this.key_name,
                        &this.key_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyOperationResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod sign {
    use super::models;
    type Response = models::KeyOperationResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) key_version: String,
        pub(crate) parameters: models::KeySignParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/keys/{}/{}/sign",
                        this.client.endpoint(),
                        &this.key_name,
                        &this.key_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyOperationResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod verify {
    use super::models;
    type Response = models::KeyVerifyResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) key_version: String,
        pub(crate) parameters: models::KeyVerifyParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/keys/{}/{}/verify",
                        this.client.endpoint(),
                        &this.key_name,
                        &this.key_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyVerifyResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod wrap_key {
    use super::models;
    type Response = models::KeyOperationResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) key_version: String,
        pub(crate) parameters: models::KeyOperationsParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/keys/{}/{}/wrapkey",
                        this.client.endpoint(),
                        &this.key_name,
                        &this.key_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyOperationResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod unwrap_key {
    use super::models;
    type Response = models::KeyOperationResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) key_version: String,
        pub(crate) parameters: models::KeyOperationsParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/keys/{}/{}/unwrapkey",
                        this.client.endpoint(),
                        &this.key_name,
                        &this.key_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyOperationResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_deleted_keys {
    use super::models;
    type Response = models::DeletedKeyListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/deletedkeys", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedKeyListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_deleted_key {
    use super::models;
    type Response = models::DeletedKeyBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/deletedkeys/{}", this.client.endpoint(), &this.key_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedKeyBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod purge_deleted_key {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/deletedkeys/{}", this.client.endpoint(), &this.key_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::NoContent => Ok(()),
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod recover_deleted_key {
    use super::models;
    type Response = models::KeyBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/deletedkeys/{}/recover", this.client.endpoint(), &this.key_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::KeyBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod set_secret {
    use super::models;
    type Response = models::SecretBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
        pub(crate) parameters: models::SecretSetParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/secrets/{}", this.client.endpoint(), &this.secret_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SecretBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod delete_secret {
    use super::models;
    type Response = models::DeletedSecretBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/secrets/{}", this.client.endpoint(), &this.secret_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedSecretBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_secret {
    use super::models;
    type Response = models::SecretBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
        pub(crate) secret_version: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/secrets/{}/{}",
                        this.client.endpoint(),
                        &this.secret_name,
                        &this.secret_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SecretBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod update_secret {
    use super::models;
    type Response = models::SecretBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
        pub(crate) secret_version: String,
        pub(crate) parameters: models::SecretUpdateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/secrets/{}/{}",
                        this.client.endpoint(),
                        &this.secret_name,
                        &this.secret_version
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SecretBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_secrets {
    use super::models;
    type Response = models::SecretListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified, the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/secrets", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SecretListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_secret_versions {
    use super::models;
    type Response = models::SecretListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified, the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/secrets/{}/versions", this.client.endpoint(), &this.secret_name))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SecretListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_deleted_secrets {
    use super::models;
    type Response = models::DeletedSecretListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/deletedsecrets", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedSecretListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_deleted_secret {
    use super::models;
    type Response = models::DeletedSecretBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/deletedsecrets/{}", this.client.endpoint(), &this.secret_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedSecretBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod purge_deleted_secret {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/deletedsecrets/{}", this.client.endpoint(), &this.secret_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::NoContent => Ok(()),
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod recover_deleted_secret {
    use super::models;
    type Response = models::SecretBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/deletedsecrets/{}/recover", this.client.endpoint(), &this.secret_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SecretBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod backup_secret {
    use super::models;
    type Response = models::BackupSecretResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) secret_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/secrets/{}/backup", this.client.endpoint(), &this.secret_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::BackupSecretResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod restore_secret {
    use super::models;
    type Response = models::SecretBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) parameters: models::SecretRestoreParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/secrets/restore", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SecretBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_storage_accounts {
    use super::models;
    type Response = models::StorageListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/storage", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::StorageListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_deleted_storage_accounts {
    use super::models;
    type Response = models::DeletedStorageListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!("{}/deletedstorage", this.client.endpoint(),))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedStorageListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_deleted_storage_account {
    use super::models;
    type Response = models::DeletedStorageBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/deletedstorage/{}", this.client.endpoint(), &this.storage_account_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedStorageBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod purge_deleted_storage_account {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/deletedstorage/{}", this.client.endpoint(), &this.storage_account_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::NoContent => Ok(()),
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod recover_deleted_storage_account {
    use super::models;
    type Response = models::StorageBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/deletedstorage/{}/recover",
                        this.client.endpoint(),
                        &this.storage_account_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::StorageBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod backup_storage_account {
    use super::models;
    type Response = models::BackupStorageResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/storage/{}/backup", this.client.endpoint(), &this.storage_account_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::BackupStorageResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod restore_storage_account {
    use super::models;
    type Response = models::StorageBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) parameters: models::StorageRestoreParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/storage/restore", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::StorageBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_storage_account {
    use super::models;
    type Response = models::StorageBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/storage/{}", this.client.endpoint(), &this.storage_account_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::StorageBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod set_storage_account {
    use super::models;
    type Response = models::StorageBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) parameters: models::StorageAccountCreateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/storage/{}", this.client.endpoint(), &this.storage_account_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::StorageBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod update_storage_account {
    use super::models;
    type Response = models::StorageBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) parameters: models::StorageAccountUpdateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/storage/{}", this.client.endpoint(), &this.storage_account_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::StorageBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod delete_storage_account {
    use super::models;
    type Response = models::DeletedStorageBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/storage/{}", this.client.endpoint(), &this.storage_account_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedStorageBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod regenerate_storage_account_key {
    use super::models;
    type Response = models::StorageBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) parameters: models::StorageAccountRegenerteKeyParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/storage/{}/regeneratekey",
                        this.client.endpoint(),
                        &this.storage_account_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::StorageBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_sas_definitions {
    use super::models;
    type Response = models::SasDefinitionListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url =
                        azure_core::Url::parse(&format!("{}/storage/{}/sas", this.client.endpoint(), &this.storage_account_name))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SasDefinitionListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_deleted_sas_definitions {
    use super::models;
    type Response = models::DeletedSasDefinitionListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) maxresults: Option<i32>,
    }
    impl Builder {
        #[doc = "Maximum number of results to return in a page. If not specified the service will return up to 25 results."]
        pub fn maxresults(mut self, maxresults: i32) -> Self {
            self.maxresults = Some(maxresults);
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/deletedstorage/{}/sas",
                        this.client.endpoint(),
                        &this.storage_account_name
                    ))?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                            if let Some(maxresults) = &this.maxresults {
                                req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedSasDefinitionListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod get_deleted_sas_definition {
    use super::models;
    type Response = models::DeletedSasDefinitionBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) sas_definition_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/deletedstorage/{}/sas/{}",
                        this.client.endpoint(),
                        &this.storage_account_name,
                        &this.sas_definition_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedSasDefinitionBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod recover_deleted_sas_definition {
    use super::models;
    type Response = models::SasDefinitionBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) sas_definition_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/deletedstorage/{}/sas/{}/recover",
                        this.client.endpoint(),
                        &this.storage_account_name,
                        &this.sas_definition_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SasDefinitionBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod get_sas_definition {
    use super::models;
    type Response = models::SasDefinitionBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) sas_definition_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/storage/{}/sas/{}",
                        this.client.endpoint(),
                        &this.storage_account_name,
                        &this.sas_definition_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SasDefinitionBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod set_sas_definition {
    use super::models;
    type Response = models::SasDefinitionBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) sas_definition_name: String,
        pub(crate) parameters: models::SasDefinitionCreateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/storage/{}/sas/{}",
                        this.client.endpoint(),
                        &this.storage_account_name,
                        &this.sas_definition_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SasDefinitionBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod update_sas_definition {
    use super::models;
    type Response = models::SasDefinitionBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) sas_definition_name: String,
        pub(crate) parameters: models::SasDefinitionUpdateParameters,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/storage/{}/sas/{}",
                        this.client.endpoint(),
                        &this.storage_account_name,
                        &this.sas_definition_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SasDefinitionBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod delete_sas_definition {
    use super::models;
    type Response = models::DeletedSasDefinitionBundle;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) storage_account_name: String,
        pub(crate) sas_definition_name: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/storage/{}/sas/{}",
                        this.client.endpoint(),
                        &this.storage_account_name,
                        &this.sas_definition_name
                    ))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DeletedSasDefinitionBundle = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod full_backup {
    use super::models;
    type Response = models::FullBackupOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) azure_storage_blob_container_uri: Option<models::SasTokenParameter>,
    }
    impl Builder {
        #[doc = "Azure blob shared access signature token pointing to a valid Azure blob container where full backup needs to be stored. This token needs to be valid for at least next 24 hours from the time of making this call"]
        pub fn azure_storage_blob_container_uri(mut self, azure_storage_blob_container_uri: impl Into<models::SasTokenParameter>) -> Self {
            self.azure_storage_blob_container_uri = Some(azure_storage_blob_container_uri.into());
            self
        }
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/backup", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = if let Some(azure_storage_blob_container_uri) = &this.azure_storage_blob_container_uri {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(azure_storage_blob_container_uri)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Accepted => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::FullBackupOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod full_backup_status {
    use super::models;
    type Response = models::FullBackupOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) job_id: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/backup/{}/pending", this.client.endpoint(), &this.job_id))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::FullBackupOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod full_restore_operation {
    use super::models;
    type Response = models::RestoreOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) restore_blob_details: Option<models::RestoreOperationParameters>,
    }
    impl Builder {
        #[doc = "The Azure blob SAS token pointing to a folder where the previous successful full backup was stored"]
        pub fn restore_blob_details(mut self, restore_blob_details: impl Into<models::RestoreOperationParameters>) -> Self {
            self.restore_blob_details = Some(restore_blob_details.into());
            self
        }
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/restore", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = if let Some(restore_blob_details) = &this.restore_blob_details {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(restore_blob_details)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Accepted => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::RestoreOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod restore_status {
    use super::models;
    type Response = models::RestoreOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) job_id: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/restore/{}/pending", this.client.endpoint(), &this.job_id))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::RestoreOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod selective_key_restore_operation {
    use super::models;
    type Response = models::SelectiveKeyRestoreOperation;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) key_name: String,
        pub(crate) restore_blob_details: Option<models::SelectiveKeyRestoreOperationParameters>,
    }
    impl Builder {
        #[doc = "The Azure blob SAS token pointing to a folder where the previous successful full backup was stored"]
        pub fn restore_blob_details(mut self, restore_blob_details: impl Into<models::SelectiveKeyRestoreOperationParameters>) -> Self {
            self.restore_blob_details = Some(restore_blob_details.into());
            self
        }
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/keys/{}/restore", this.client.endpoint(), &this.key_name))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                    let req_body = if let Some(restore_blob_details) = &this.restore_blob_details {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(restore_blob_details)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Accepted => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SelectiveKeyRestoreOperation = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod role_definitions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the specified role definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `scope`: The scope of the role definition to get. Managed HSM only supports '/'."]
        #[doc = "* `role_definition_name`: The name of the role definition to get."]
        pub fn get(&self, scope: impl Into<String>, role_definition_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                scope: scope.into(),
                role_definition_name: role_definition_name.into(),
            }
        }
        #[doc = "Creates or updates a custom role definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `scope`: The scope of the role definition to create or update. Managed HSM only supports '/'."]
        #[doc = "* `role_definition_name`: The name of the role definition to create or update. It can be any valid GUID."]
        #[doc = "* `parameters`: Parameters for the role definition."]
        pub fn create_or_update(
            &self,
            scope: impl Into<String>,
            role_definition_name: impl Into<String>,
            parameters: impl Into<models::RoleDefinitionCreateParameters>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                scope: scope.into(),
                role_definition_name: role_definition_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes a custom role definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `scope`: The scope of the role definition to delete. Managed HSM only supports '/'."]
        #[doc = "* `role_definition_name`: The name (GUID) of the role definition to delete."]
        pub fn delete(&self, scope: impl Into<String>, role_definition_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                scope: scope.into(),
                role_definition_name: role_definition_name.into(),
            }
        }
        #[doc = "Get all role definitions that are applicable at scope and above."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `scope`: The scope of the role definition."]
        pub fn list(&self, scope: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                scope: scope.into(),
                filter: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::RoleDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) scope: String,
            pub(crate) role_definition_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.Authorization/roleDefinitions/{}",
                            this.client.endpoint(),
                            &this.scope,
                            &this.role_definition_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RoleDefinition = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        type Response = models::RoleDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) scope: String,
            pub(crate) role_definition_name: String,
            pub(crate) parameters: models::RoleDefinitionCreateParameters,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.Authorization/roleDefinitions/{}",
                            this.client.endpoint(),
                            &this.scope,
                            &this.role_definition_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RoleDefinition = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        type Response = models::RoleDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) scope: String,
            pub(crate) role_definition_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.Authorization/roleDefinitions/{}",
                            this.client.endpoint(),
                            &this.scope,
                            &this.role_definition_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RoleDefinition = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::RoleDefinitionListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) scope: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply on the operation. Use atScopeAndBelow filter to search below the given scope as well."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.Authorization/roleDefinitions",
                            this.client.endpoint(),
                            &this.scope
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RoleDefinitionListResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod role_assignments {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the specified role assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `scope`: The scope of the role assignment."]
        #[doc = "* `role_assignment_name`: The name of the role assignment to get."]
        pub fn get(&self, scope: impl Into<String>, role_assignment_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                scope: scope.into(),
                role_assignment_name: role_assignment_name.into(),
            }
        }
        #[doc = "Creates a role assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `scope`: The scope of the role assignment to create."]
        #[doc = "* `role_assignment_name`: The name of the role assignment to create. It can be any valid GUID."]
        #[doc = "* `parameters`: Parameters for the role assignment."]
        pub fn create(
            &self,
            scope: impl Into<String>,
            role_assignment_name: impl Into<String>,
            parameters: impl Into<models::RoleAssignmentCreateParameters>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                scope: scope.into(),
                role_assignment_name: role_assignment_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Deletes a role assignment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `scope`: The scope of the role assignment to delete."]
        #[doc = "* `role_assignment_name`: The name of the role assignment to delete."]
        pub fn delete(&self, scope: impl Into<String>, role_assignment_name: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                scope: scope.into(),
                role_assignment_name: role_assignment_name.into(),
            }
        }
        #[doc = "Gets role assignments for a scope."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `scope`: The scope of the role assignments."]
        pub fn list_for_scope(&self, scope: impl Into<String>) -> list_for_scope::Builder {
            list_for_scope::Builder {
                client: self.0.clone(),
                scope: scope.into(),
                filter: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::RoleAssignment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) scope: String,
            pub(crate) role_assignment_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.Authorization/roleAssignments/{}",
                            this.client.endpoint(),
                            &this.scope,
                            &this.role_assignment_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RoleAssignment = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::RoleAssignment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) scope: String,
            pub(crate) role_assignment_name: String,
            pub(crate) parameters: models::RoleAssignmentCreateParameters,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.Authorization/roleAssignments/{}",
                            this.client.endpoint(),
                            &this.scope,
                            &this.role_assignment_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RoleAssignment = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        type Response = models::RoleAssignment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) scope: String,
            pub(crate) role_assignment_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.Authorization/roleAssignments/{}",
                            this.client.endpoint(),
                            &this.scope,
                            &this.role_assignment_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RoleAssignment = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_for_scope {
        use super::models;
        type Response = models::RoleAssignmentListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) scope: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply on the operation. Use $filter=atScope() to return all role assignments at or above the scope. Use $filter=principalId eq {id} to return all role assignments at, above or below the scope for the specified principal."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/providers/Microsoft.Authorization/roleAssignments",
                            this.client.endpoint(),
                            &this.scope
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RoleAssignmentListResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod hsm_security_domain {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieves the Security Domain download operation status"]
        pub fn download_pending(&self) -> download_pending::Builder {
            download_pending::Builder { client: self.0.clone() }
        }
        #[doc = "Retrieves the Security Domain from the managed HSM. Calling this endpoint can be used to activate a provisioned managed HSM resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `certificate_info_object`: The Security Domain download operation requires customer to provide N certificates (minimum 3 and maximum 10) containing a public key in JWK format."]
        pub fn download(&self, certificate_info_object: impl Into<models::CertificateInfoObject>) -> download::Builder {
            download::Builder {
                client: self.0.clone(),
                certificate_info_object: certificate_info_object.into(),
            }
        }
        #[doc = "Retrieve Security Domain transfer key"]
        pub fn transfer_key(&self) -> transfer_key::Builder {
            transfer_key::Builder { client: self.0.clone() }
        }
        #[doc = "Restore the provided Security Domain."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `security_domain`: The Security Domain to be restored."]
        pub fn upload(&self, security_domain: impl Into<models::SecurityDomainObject>) -> upload::Builder {
            upload::Builder {
                client: self.0.clone(),
                security_domain: security_domain.into(),
            }
        }
        #[doc = "Get Security Domain upload operation status"]
        pub fn upload_pending(&self) -> upload_pending::Builder {
            upload_pending::Builder { client: self.0.clone() }
        }
    }
    pub mod download_pending {
        use super::models;
        type Response = models::SecurityDomainOperationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/securitydomain/download/pending", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityDomainOperationStatus = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod download {
        use super::models;
        type Response = models::SecurityDomainObject;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) certificate_info_object: models::CertificateInfoObject,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/securitydomain/download", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.certificate_info_object)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityDomainObject = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod transfer_key {
        use super::models;
        type Response = models::TransferKey;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/securitydomain/upload", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.2-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TransferKey = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod upload {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Accepted202(models::SecurityDomainOperationStatus),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) security_domain: models::SecurityDomainObject,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/securitydomain/upload", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.security_domain)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityDomainOperationStatus = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
                            }
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod upload_pending {
        use super::models;
        type Response = models::SecurityDomainOperationStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/securitydomain/upload/pending", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SecurityDomainOperationStatus = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
