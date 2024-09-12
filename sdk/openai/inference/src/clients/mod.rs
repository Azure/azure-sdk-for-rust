// use azure_core::{
//     headers::{ACCEPT, CONTENT_TYPE},
//     Header, Method, Request, Result, Url,
// };
// use serde::Serialize;

pub mod azure_openai_client;
pub mod chat_completions_client;
pub mod openai_client;

// pub(crate) fn build_request<T>(
//     key_credential: &impl Header,
//     url: Url,
//     method: Method,
//     data: &T,
// ) -> Result<Request>
// where
//     T: ?Sized + Serialize,
// {
//     let mut request = Request::new(url, method);
//     request.add_mandatory_header(key_credential);
//     request.insert_header(CONTENT_TYPE, "application/json");
//     request.insert_header(ACCEPT, "application/json");
//     request.set_json(data)?;
//     Ok(request)
// }

// pub(crate) fn build_multipart_request<F>(
//     key_credential: &impl Header,
//     url: Url,
//     form_generator: F,
// ) -> Result<Request>
// where
//     F: FnOnce() -> Result<MyForm>,
// {
//     let mut request = Request::new(url, Method::Post);
//     request.add_mandatory_header(key_credential);
//     // handled insternally by reqwest
//     // request.insert_header(CONTENT_TYPE, "multipart/form-data");
//     // request.insert_header(ACCEPT, "application/json");
//     request.multipart(form_generator()?);
//     Ok(request)
// }
