use crate::Result;
use autorust_openapi::{MsLongRunningOperationOptions, MsLongRunningOperationOptionsFinalStateVia};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use super::response_code::ResponseCode;

pub struct RequestBuilderIntoFutureCode {
    response_code: ResponseCode,
    lro: bool,
    lro_options: Option<MsLongRunningOperationOptions>,
}

impl RequestBuilderIntoFutureCode {
    pub fn new(response_code: ResponseCode, lro: bool, lro_options: Option<MsLongRunningOperationOptions>) -> Result<Self> {
        Ok(Self {
            response_code,
            lro,
            lro_options,
        })
    }
}

impl ToTokens for RequestBuilderIntoFutureCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Skip generating IntoFuture if response is pageable
        if self.response_code.pageable.is_some() {
            return;
        }

        let into_future = if let Some(response_type) = self.response_code.response_type() {
            let (func, rest) = if self.lro {
                if let Some(lro_options) = &self.lro_options {
                    let final_state = match lro_options.final_state_via {
                        MsLongRunningOperationOptionsFinalStateVia::Location => Some(format_ident!("Location")),
                        MsLongRunningOperationOptionsFinalStateVia::AzureAsyncOperation => Some(format_ident!("AzureAsyncOperation")),
                        MsLongRunningOperationOptionsFinalStateVia::OriginalUri => None,
                    };

                    if let Some(final_state) = final_state {
                        (
                            quote! {
                                use azure_core::{
                                    lro::{
                                        location::{get_location, FinalState, get_provisioning_state},
                                        LroStatus, get_retry_after,
                                    },
                                    error::{Error, ErrorKind},
                                    sleep::sleep
                                };
                                use std::time::Duration;

                                let this = self.clone();
                                let response = this.send().await?;
                                let headers = response.as_raw_response().headers();
                                let location = get_location(headers, FinalState::#final_state)?;
                                if let Some(url) = location {
                                    loop {
                                        let mut req = azure_core::Request::new(url.clone(), azure_core::Method::GET);
                                        let bearer_token = self.client.bearer_token().await?;
                                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                        let response = self.client.send(&mut req).await?;
                                        let headers = response.headers();
                                        let retry_after = get_retry_after(headers);
                                        let bytes = response.into_body().collect().await?;
                                        let provisioning_state = get_provisioning_state(&bytes).ok_or_else(||
                                            Error::message(ErrorKind::Other, "Long running operation failed (missing provisioning state)".to_string())
                                        )?;
                                        log::trace!("current provisioning_state: {provisioning_state:?}");
                                        match provisioning_state {
                                            LroStatus::Succeeded => {
                                                let mut req = azure_core::Request::new(self.url()?, azure_core::Method::GET);
                                                let bearer_token = self.client.bearer_token().await?;
                                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                                let response = self.client.send(&mut req).await?;
                                                return Response(response).into_body().await
                                            }
                                            LroStatus::Failed => return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string())),
                                            LroStatus::Canceled => return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string())),
                                            _ => {
                                                sleep(retry_after).await;
                                            }
                                        }
                                    }
                                } else {
                                    response.into_body().await
                                }
                            },
                            quote! {
                                    #[doc = "Returns a future that polls the long running operation, returning once the operation completes."]
                                    #[doc = ""]
                                    #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
                            },
                        )
                    } else {
                        // println!("unsupported LRO: {lro_options:?}");

                        (
                            quote! {
                                self.send().await?.into_body().await
                            },
                            quote! {
                                    #[doc = "Returns a future that sends the request and returns the parsed response body."]
                                    #[doc = ""]
                                    #[doc = "This operation uses a method of polling the status of a long running operation that is not yet supported.  Only the first response will be fetched."]
                            },
                        )
                    }
                } else {
                    (
                        quote! {
                            use azure_core::{
                                lro::{body_content::get_provisioning_state, LroStatus, get_retry_after},
                                error::{Error, ErrorKind},
                                sleep::sleep
                            };
                            use std::time::Duration;
                            loop {
                                let this = self.clone();
                                let response = this.send().await?;
                                let retry_after = get_retry_after(response.as_raw_response().headers());
                                let status = response.as_raw_response().status();
                                let body = response.into_body().await?;
                                let provisioning_state = get_provisioning_state(status, &body)?;
                                log::trace!("current provisioning_state: {provisioning_state:?}");
                                match provisioning_state {
                                    LroStatus::Succeeded => return Ok(body),
                                    LroStatus::Failed => return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string())),
                                    LroStatus::Canceled => return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string())),
                                    _ => {
                                        sleep(retry_after).await;
                                    }
                                }
                            }
                        },
                        quote! {
                                #[doc = "Returns a future that polls the long running operation and checks for the state via `properties.provisioningState` in the response body."]
                                #[doc = ""]
                                #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
                        },
                    )
                }
            } else {
                (
                    quote! {
                        self.send().await?.into_body().await
                    },
                    quote! {
                            #[doc = "Returns a future that sends the request and returns the parsed response body."]
                    },
                )
            };

            quote! {
                impl std::future::IntoFuture for RequestBuilder {
                    type Output = azure_core::Result<#response_type>;
                    type IntoFuture = BoxFuture<'static, azure_core::Result<#response_type>>;
                    #rest
                    #[doc = ""]
                    #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
                    #[doc = ""]
                    #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
                    fn into_future(self) -> Self::IntoFuture {
                        Box::pin(
                            async move {
                                #func
                            }
                        )
                    }
                }
            }
        } else {
            quote! {}
        };

        tokens.extend(into_future);
    }
}
