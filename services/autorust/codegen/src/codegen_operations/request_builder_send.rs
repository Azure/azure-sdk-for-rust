use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::codegen::PARAM_RE;
use crate::Result;
use crate::{codegen::parse_path_params, identifier::SnakeCaseIdent};

use super::{new_request_code::NewRequestCode, response_code::ResponseCode, set_request_code::SetRequestCode};
/// The `send` function of the request builder.
pub struct RequestBuilderSendCode {
    new_request_code: NewRequestCode,
    request_builder: SetRequestCode,
    response_code: ResponseCode,
    url_args: Vec<Ident>,
}

impl RequestBuilderSendCode {
    pub fn new(new_request_code: NewRequestCode, request_builder: SetRequestCode, response_code: ResponseCode) -> Result<Self> {
        let params = parse_path_params(&new_request_code.path);
        let url_args: Result<Vec<_>> = params.iter().map(|s| s.to_snake_case_ident()).collect();
        let url_args = url_args?;
        Ok(Self {
            new_request_code,
            request_builder,
            response_code,
            url_args,
        })
    }
}

impl ToTokens for RequestBuilderSendCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_request_code = &self.new_request_code;
        let request_builder = &self.request_builder;

        let url_args = self.url_args.iter().map(|url_arg| {
            quote! { &self.#url_arg }
        });
        let url_str_args = quote! { #(#url_args),* };

        let fpath = format!("{{}}{}", &format_path(&new_request_code.path));

        let mut match_status = TokenStream::new();
        for status_response in &self.response_code.status_responses {
            let status_code_name = &status_response.status_code_name;
            match_status.extend(quote! {
                azure_core::StatusCode::#status_code_name => Ok(Response(rsp)),
            });
        }
        match_status.extend(quote! {
            status_code => {
                Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse { status: status_code, error_code: None }))
            }
        });

        let urlfn = if self.request_builder.has_param_api_version {
            let api_version = &self.request_builder.api_version;
            quote! {
                fn url(&self) -> azure_core::Result<azure_core::Url> {
                    let mut url = azure_core::Url::parse(&format!(#fpath, self.client.endpoint(), #url_str_args))?;

                    let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                    if !has_api_version_already {
                        url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, #api_version);
                    }
                    Ok(url)
                }
            }
        } else {
            quote! {
                fn url(&self) -> azure_core::Result<azure_core::Url> {
                    let url = azure_core::Url::parse(&format!(#fpath, self.client.endpoint(), #url_str_args))?;
                    Ok(url)
                }
            }
        };

        let send_future = quote! {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        #new_request_code
                        #request_builder
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        };

        let fut = if let Some(pageable) = &self.response_code.pageable {
            // TODO: Pageable requires the values to be part of the response schema,
            // however, some schemas do this via the header x-ms-continuation rather than
            // provide a next_link_name.  For now, those cases get documented that we don't
            // poll and move on.
            if let Some(next_link_name) = pageable.next_link_name.as_ref() {
                let mut stream_api_version = quote! {};

                // per discussion in SDK meeting, we should always set the
                // api-version on the request if we have a version.
                if request_builder.has_param_api_version {
                    let api_version = &request_builder.api_version;
                    stream_api_version.extend(quote! {
                        let has_api_version_already = req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                        if !has_api_version_already {
                            req.url_mut().query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, #api_version);
                        }
                    });
                }

                if request_builder.has_param_x_ms_version {
                    let api_version = &request_builder.api_version;
                    stream_api_version.extend(quote! {
                        req.insert_header(azure_core::headers::VERSION, #api_version);
                    });
                }
                let response_type = self.response_code.response_type().expect("pageable response has a body");

                // some of the pageable requests specify the continuation token
                // as a parameter.  In this case, use the basic request builder,
                // but insert the continuation parameter
                if let Some(continuable_param) = get_continuable_param(next_link_name, request_builder) {
                    quote! {
                        pub fn into_stream(self) -> azure_core::Pageable<#response_type, azure_core::error::Error> {
                            let make_request = move |continuation: Option<String>| {
                                let this = self.clone();
                                async move {
                                    let mut url = this.url()?;
                                    #new_request_code
                                    #request_builder
                                    if let Some(value) = continuation.as_ref() {
                                        req.url_mut().query_pairs_mut().append_pair(#continuable_param, value);
                                    }
                                    req.set_body(req_body);
                                    let rsp = this.client.send(&mut req).await?;
                                    let rsp =
                                        match rsp.status() {
                                            #match_status
                                        };
                                    rsp?.into_body().await
                                }
                            };

                            azure_core::Pageable::new(make_request)
                        }
                    }
                } else {
                    quote! {
                        pub fn into_stream(self) -> azure_core::Pageable<#response_type, azure_core::error::Error> {
                            let make_request = move |continuation: Option<String>| {
                                let this = self.clone();
                                async move {
                                    let mut url = this.url()?;

                                    let rsp = match continuation {
                                        Some(value) => {
                                            url.set_path("");
                                            url = url.join(&value)?;
                                            #new_request_code
                                            #stream_api_version
                                            let req_body = azure_core::EMPTY_BODY;
                                            req.set_body(req_body);
                                            this.client.send(&mut req).await?
                                        }
                                        None => {
                                            #new_request_code
                                            #request_builder
                                            req.set_body(req_body);
                                            this.client.send(&mut req).await?
                                        }
                                    };
                                    let rsp =
                                        match rsp.status() {
                                            #match_status
                                        };
                                    rsp?.into_body().await
                                }
                            };

                            azure_core::Pageable::new(make_request)
                        }
                    }
                }
            } else {
                // most often when this happens, the continuation token is provided
                // by an HTTP Header x-ms-continuation, which should be extracted
                // from the response.
                //
                // Note, this is only *sometimes* this is specified in the spec.
                //
                // Ref: https://github.com/Azure/azure-sdk-for-rust/issues/446
                let mut fut = quote! {
                    #[doc = "Only the first response will be fetched as the continuation token is not part of the response schema"]
                    #[doc = ""]
                };
                fut.extend(send_future);
                fut
            }
        } else {
            send_future
        };
        tokens.extend(fut);
        tokens.extend(urlfn)
    }
}

fn format_path(path: &str) -> String {
    PARAM_RE.replace_all(path, "{}").to_string()
}

fn get_continuable_param(next_link_name: &str, request_builder: &SetRequestCode) -> Option<String> {
    let next_link_name = next_link_name.to_snake_case();
    let link_name = next_link_name.strip_prefix("next_");

    for param in request_builder.parameters.params() {
        let param_name = param.variable_name.to_string();
        if param_name == next_link_name {
            return Some(param_name);
        }
        if let Some(link_name) = link_name {
            if param_name == link_name {
                return Some(param_name);
            }
        }
    }
    None
}
