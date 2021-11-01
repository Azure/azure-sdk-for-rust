//! Azure Event Grid crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
mod event;
// TODO update event_grid to use HttpClient https://github.com/Azure/azure-sdk-for-rust/issues/254
// mod event_grid_client;
// mod event_grid_request;
// mod event_grid_request_builder;
// mod event_grid_response;
pub use event::Event;
// pub use event_grid_client::EventGridClient;
