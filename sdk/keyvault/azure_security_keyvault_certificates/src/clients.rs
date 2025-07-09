// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::*;
use crate::models::{
    CertificateClientBeginCreateCertificateOptions,
    CertificateClientResumeCertificateOperationOptions, CertificateOperation,
    CreateCertificateParameters,
};
use azure_core::{
    http::{
        poller::{get_retry_after, PollerResult, StatusMonitor as _},
        Body, Method, Poller, PollerStatus, RawResponse, Request, RequestContent, Url,
    },
    json, Result,
};

pub trait CertificateClientExt: private::Sealed {
    /// Creates a new certificate and returns a [`Poller<CertificateOperation>`] to monitor the status.
    ///
    /// If this is the first version, the certificate resource is created. This operation requires the certificates/create permission.
    ///
    /// # Arguments
    ///
    /// * `certificate_name` - The name of the certificate. The value you provide may be copied globally for the purpose of running
    ///   the service. The value provided should not include personally identifiable or sensitive information.
    /// * `parameters` - The parameters to create a certificate.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_identity::DefaultAzureCredential;
    /// use azure_security_keyvault_certificates::{
    ///     CertificateClient, CertificateClientExt,
    ///     models::{CreateCertificateParameters, DEFAULT_POLICY},
    /// };
    ///
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DefaultAzureCredential::new()?;
    /// let client = CertificateClient::new(
    ///     "https://your-key-vault-name.vault.azure.net/",
    ///     credential.clone(),
    ///     None,
    /// )?;
    ///
    /// // Create a self-signed certificate.
    /// let body = CreateCertificateParameters {
    ///     certificate_policy: Some(DEFAULT_POLICY.clone()),
    ///     ..Default::default()
    /// };
    ///
    /// // Wait for the certificate operation to complete and get the last status monitor.
    /// let operation = client
    ///     .begin_create_certificate("certificate-name", body.try_into()?, None)?
    ///     .wait()
    ///     .await?
    ///     // Deserialize the CertificateOperation:
    ///     .into_body()
    ///     .await?;
    ///
    /// if matches!(operation.status, Some(status) if status == "completed") {
    ///     let target = operation.target.ok_or("expected target")?;
    ///     println!("Created certificate {}", target);
    /// }
    ///
    /// # Ok(()) }
    /// ```
    fn begin_create_certificate(
        &self,
        certificate_name: &str,
        parameters: RequestContent<CreateCertificateParameters>,
        options: Option<CertificateClientBeginCreateCertificateOptions<'_>>,
    ) -> Result<Poller<CertificateOperation>>;

    /// Resumes the [`CertificateClientExt::begin_create_certificate`] operation by returning a [`Poller<CertificateOperation>`] already in progress or completed.
    ///
    /// Gets the creation operation associated with a specified certificate. This operation requires the certificates/get permission.
    ///
    /// # Arguments
    ///
    /// * `certificate_name` - The name of the certificate.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_identity::DefaultAzureCredential;
    /// use azure_security_keyvault_certificates::{
    ///     CertificateClient, CertificateClientExt,
    ///     models::{CreateCertificateParameters, DEFAULT_POLICY},
    /// };
    ///
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DefaultAzureCredential::new()?;
    /// let client = CertificateClient::new(
    ///     "https://your-key-vault-name.vault.azure.net/",
    ///     credential.clone(),
    ///     None,
    /// )?;
    ///
    /// // Create a self-signed certificate.
    /// let body = CreateCertificateParameters {
    ///     certificate_policy: Some(DEFAULT_POLICY.clone()),
    ///     ..Default::default()
    /// };
    ///
    /// // Wait for the certificate operation to complete and get the last status monitor.
    /// let operation = client
    ///     .resume_certificate_operation("certificate-name",  None)?
    ///     .wait()
    ///     .await?
    ///     // Deserialize the CertificateOperation:
    ///     .into_body()
    ///     .await?;
    ///
    /// if matches!(operation.status, Some(status) if status == "completed") {
    ///     let target = operation.target.ok_or("expected target")?;
    ///     println!("Created certificate {}", target);
    /// }
    ///
    /// # Ok(()) }
    /// ```
    fn resume_certificate_operation(
        &self,
        certificate_name: &str,
        options: Option<CertificateClientResumeCertificateOperationOptions<'_>>,
    ) -> Result<Poller<CertificateOperation>>;
}

mod private {
    pub trait Sealed {}
    impl Sealed for super::CertificateClient {}
}

impl CertificateClientExt for CertificateClient {
    fn begin_create_certificate(
        &self,
        certificate_name: &str,
        parameters: RequestContent<CreateCertificateParameters>,
        options: Option<CertificateClientBeginCreateCertificateOptions<'_>>,
    ) -> Result<Poller<CertificateOperation>> {
        let options = options.unwrap_or_default().into_owned();
        let pipeline = self.pipeline.clone();

        let mut url = self.endpoint.clone();
        let mut path = String::from("certificates/{certificate-name}/create");
        path = path.replace("{certificate-name}", certificate_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);

        let api_version = self.api_version.clone();
        let certificate_name = certificate_name.to_owned();
        let parameters: Body = parameters.into();

        Ok(Poller::from_callback(
            move |next_link: Option<Url>| {
                let (mut request, next_link) = match next_link {
                    Some(next_link) => {
                        // Make sure the `api-version` is set appropriately.
                        let qp = next_link
                            .query_pairs()
                            .filter(|(name, _)| name.ne("api-version"));
                        let mut next_link = next_link.clone();
                        next_link
                            .query_pairs_mut()
                            .clear()
                            .extend_pairs(qp)
                            .append_pair("api-version", &api_version);

                        let mut request = Request::new(next_link.clone(), Method::Get);
                        request.insert_header("accept", "application/json");

                        (request, next_link)
                    }
                    None => {
                        let mut request = Request::new(url.clone(), Method::Post);
                        request.insert_header("accept", "application/json");
                        request.insert_header("content-type", "application/json");
                        request.set_body(&parameters);

                        let mut url = url.clone();
                        let mut path = String::from("certificates/{certificate-name}/pending");
                        path = path.replace("{certificate-name}", &certificate_name);
                        url.set_path(&path);

                        (request, url)
                    }
                };

                let ctx = options.method_options.context.clone();
                let pipeline = pipeline.clone();
                async move {
                    let rsp: RawResponse = pipeline.send(&ctx, &mut request).await?;
                    let (status, headers, body) = rsp.deconstruct();
                    let retry_after = get_retry_after(&headers, &options.poller_options);
                    let bytes = body.collect().await?;
                    let res: CertificateOperation = json::from_json(&bytes)?;
                    let rsp = RawResponse::from_bytes(status, headers, bytes).into();

                    Ok(match res.status() {
                        PollerStatus::InProgress => PollerResult::InProgress {
                            response: rsp,
                            retry_after,
                            next: next_link,
                        },
                        _ => PollerResult::Done { response: rsp },
                    })
                }
            },
            None,
        ))
    }

    fn resume_certificate_operation(
        &self,
        certificate_name: &str,
        options: Option<CertificateClientResumeCertificateOperationOptions<'_>>,
    ) -> Result<Poller<CertificateOperation>> {
        let options = options.unwrap_or_default().into_owned();
        let pipeline = self.pipeline.clone();
        let mut url = self.endpoint.clone();
        let mut path = String::from("certificates/{certificate-name}/pending");
        path = path.replace("{certificate-name}", certificate_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        Ok(Poller::from_callback(
            move |_| {
                let url = url.clone();
                let mut request = Request::new(url.clone(), Method::Get);
                request.insert_header("accept", "application/json");
                let ctx = options.method_options.context.clone();
                let pipeline = pipeline.clone();
                async move {
                    let rsp: RawResponse = pipeline.send(&ctx, &mut request).await?;
                    let (status, headers, body) = rsp.deconstruct();
                    let retry_after = get_retry_after(&headers, &options.poller_options);
                    let bytes = body.collect().await?;
                    let res: CertificateOperation = json::from_json(&bytes)?;
                    let rsp = RawResponse::from_bytes(status, headers, bytes).into();

                    Ok(match res.status() {
                        PollerStatus::InProgress => PollerResult::InProgress {
                            response: rsp,
                            retry_after,
                            next: url,
                        },
                        _ => PollerResult::Done { response: rsp },
                    })
                }
            },
            None,
        ))
    }
}
