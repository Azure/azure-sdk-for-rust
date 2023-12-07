use autorust_openapi::CollectionFormat;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::function_params::{FunctionParam, FunctionParams, ParamKind};

/// Sets all of the request parameters.
pub struct SetRequestParamsCode {
    pub content_type: String,
    pub params: FunctionParams,
}

impl ToTokens for SetRequestParamsCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for param in self.params.params() {
            let FunctionParam {
                name: param_name,
                variable_name: param_name_var,
                kind,
                collection_format,
                ..
            } = param;
            let is_vec = param.is_vec();
            match kind {
                ParamKind::Path => {} // handled above
                ParamKind::Query => {
                    let query_body = if is_vec {
                        match collection_format {
                            CollectionFormat::Multi => Some(
                                if param.is_string(){
                                    quote! {
                                        for value in &this.#param_name_var {
                                            req.url_mut().query_pairs_mut().append_pair(#param_name, value);
                                        }
                                    }
                                } else {
                                    quote! {
                                        for value in &this.#param_name_var {
                                            req.url_mut().query_pairs_mut().append_pair(#param_name, &value.to_string());
                                        }
                                    }
                                }
                            ),
                            CollectionFormat::Csv | // TODO #71
                            CollectionFormat::Ssv |
                            CollectionFormat::Tsv |
                            CollectionFormat::Pipes => None,
                        }
                    } else {
                        Some(if param.is_string() {
                            quote! {
                                req.url_mut().query_pairs_mut().append_pair(#param_name, #param_name_var);
                            }
                        } else {
                            quote! {
                                req.url_mut().query_pairs_mut().append_pair(#param_name, &#param_name_var.to_string());
                            }
                        })
                    };
                    if let Some(query_body) = query_body {
                        if !param.is_optional() || is_vec {
                            tokens.extend(quote! {
                                let #param_name_var = &this.#param_name_var;
                                #query_body
                            });
                        } else {
                            tokens.extend(quote! {
                                if let Some(#param_name_var) = &this.#param_name_var {
                                    #query_body
                                }
                            });
                        }
                    }
                }
                ParamKind::Header => {
                    // always use lowercase header names
                    let header_name = param_name.to_lowercase();
                    if !param.is_optional() || is_vec {
                        if param.is_string() {
                            tokens.extend(quote! {
                                req.insert_header(#header_name, &this.#param_name_var);
                            });
                        } else {
                            tokens.extend(quote! {
                                req.insert_header(#header_name, &this.#param_name_var.to_string());
                            });
                        }
                    } else if param.is_string() {
                        tokens.extend(quote! {
                            if let Some(#param_name_var) = &this.#param_name_var {
                                req.insert_header(#header_name, #param_name_var);
                            }
                        });
                    } else {
                        tokens.extend(quote! {
                            if let Some(#param_name_var) = &this.#param_name_var {
                                req.insert_header(#header_name, &#param_name_var.to_string());
                            }
                        });
                    }
                }
                ParamKind::Body => {
                    let set_content_type = if !self.params.has_content_type_header() {
                        let content_type = &self.content_type;
                        quote! {
                            req.insert_header("content-type", #content_type);
                        }
                    } else {
                        quote! {}
                    };

                    // TODO: more work needs to be done to ensure we're using
                    // the right encoder.
                    let encoder = if !self.params.has_content_type_header() && self.content_type.starts_with("application/xml") {
                        quote! {azure_core::xml::to_xml}
                    } else {
                        quote! { azure_core::to_json }
                    };

                    if !param.is_optional() || is_vec {
                        tokens.extend(quote! {
                            #set_content_type
                            let req_body = #encoder(&this.#param_name_var)?;
                        });
                    } else {
                        tokens.extend(quote! {
                            let req_body =
                                if let Some(#param_name_var) = &this.#param_name_var {
                                    #set_content_type
                                    #encoder(#param_name_var)?
                                } else {
                                    azure_core::EMPTY_BODY
                                };
                        });
                    }
                }
                ParamKind::FormData => {
                    tokens.extend(quote! {
                        unimplemented!("form data not yet supported");
                    });
                    // https://github.com/Azure/azure-sdk-for-rust/issues/500
                    // if required {
                    //     ts_request_builder.extend(quote! {
                    //         req.set_body_from_form(&self.#param_name_var)?;
                    //     });
                    // } else {
                    //     ts_request_builder.extend(quote! {
                    //         if let Some(#param_name_var) = &self.#param_name_var {
                    //             req.set_body_from_form(#param_name_var)?;
                    //         }
                    //     });
                    // }
                }
            }
        }
    }
}
