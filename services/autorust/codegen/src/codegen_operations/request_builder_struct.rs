use autorust_openapi::{MsLongRunningOperationOptions, MsLongRunningOperationOptionsFinalStateVia};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::function_params::{FunctionParam, FunctionParams};

/// The request builder struct type, not the impl.
#[derive(Clone)]
pub struct RequestBuilderStructCode {
    parameters: FunctionParams,
    in_operation_group: bool,
    lro: bool,
    lro_options: Option<MsLongRunningOperationOptions>,
}

impl RequestBuilderStructCode {
    pub fn new(
        parameters: &FunctionParams,
        in_operation_group: bool,
        lro: bool,
        lro_options: Option<MsLongRunningOperationOptions>,
    ) -> Self {
        Self {
            parameters: parameters.clone(),
            in_operation_group,
            lro,
            lro_options,
        }
    }
}

impl ToTokens for RequestBuilderStructCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params: Vec<TokenStream> = Vec::new();
        if self.in_operation_group {
            params.push(quote! { pub(crate) client: super::super::Client });
        } else {
            params.push(quote! { pub(crate) client: super::Client });
        }
        for param in self.parameters.required_params() {
            let FunctionParam {
                variable_name, type_name, ..
            } = param;
            params.push(quote! { pub(crate) #variable_name: #type_name });
        }
        for param in self.parameters.optional_params() {
            let FunctionParam {
                variable_name, type_name, ..
            } = param;
            let mut type_name = type_name.clone();
            if type_name.is_vec() {
                type_name.optional(false);
            }
            params.push(quote! { pub(crate) #variable_name: #type_name });
        }

        let lro_docs = if self.lro
            && matches!(
                self.lro_options,
                None | Some(MsLongRunningOperationOptions {
                    final_state_via: MsLongRunningOperationOptionsFinalStateVia::AzureAsyncOperation
                        | MsLongRunningOperationOptionsFinalStateVia::Location
                })
            ) {
            quote! {
                /// This `RequestBuilder` implements a Long Running Operation
                /// (LRO).
                ///
                /// To finalize and submit the request, invoke `.await`, which
                /// which will convert the `RequestBuilder` into a future
                /// executes the request and polls the service until the
                /// operation completes.
                ///
                /// In order to execute the request without polling the service
                /// until the operation completes, use
                /// [`RequestBuilder::send()`], which will return a lower-level
                /// [`Response`] value.
            }
        } else if self.lro
            && matches!(
                self.lro_options,
                Some(MsLongRunningOperationOptions {
                    final_state_via: MsLongRunningOperationOptionsFinalStateVia::OriginalUri
                })
            )
        {
            quote! {
                /// This [`RequestBuilder`] implements a request that returns an
                /// unsupported Long Running Operation (LRO).  Currently, the
                /// implementation does not support polling the status of the
                /// operation, however future versions of this crate may include
                /// this support.
                ///
                /// To finalize and submit the request, invoke `.await`, which
                /// which will convert the [`RequestBuilder`] into a future
                /// executes the request.  Future versions may poll the service
                /// until the operation completes.
                ///
                /// In order to execute the request without polling the service
                /// until the operation completes, use
                /// [`RequestBuilder::send()`], which will return a lower-level
                /// [`Response`] value.
            }
        } else {
            quote! {
                /// To finalize and submit the request, invoke `.await`, which
                /// which will convert the [`RequestBuilder`] into a future
                /// executes the request and returns a `Result` with the parsed
                /// response.
                ///
                /// In order to execute the request without polling the service
                /// until the operation completes, use `.send().await` instead.
                ///
                /// If you need lower-level access to the raw response details
                /// (e.g. to inspect response headers or raw body data) then you
                /// can finalize the request using the
                /// [`RequestBuilder::send()`] method which returns a future
                /// that resolves to a lower-level [`Response`] value.
            }
        };

        tokens.extend(quote! {
            #[derive(Clone)]
            /// `RequestBuilder` provides a mechanism for setting optional parameters on a request.
            ///
            /// Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple
            /// parameters can be chained.
            ///
            #lro_docs
            pub struct RequestBuilder {
                #(#params),*
            }
        });
    }
}
