// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// These tests assume that the AmqpTestBroker is running on localhost:25672.
// The AmqpTestBroker can be installed by the following steps:
// 1. Clone the repository:
//      git clone https://github.com/Azure/azure-amqp
// 2. Build the project:
//      dotnet build
// 3. Run the broker:
//      dotnet run --project .\test\TestAmqpBroker\TestAmqpBroker.csproj --framework net462 amqp://localhost:25672
// 4. Run the tests (from the root of the azure-sdk-for-rust repository):
//      cargo run --example connection --package azure_core_amqp

use azure_core::Url;
use azure_core_amqp::{
    connection::{AmqpConnection, AmqpConnectionApis},
    value::AmqpSymbol,
};

async fn amqp_connection_open() {
    let connection = AmqpConnection::new();

    let url = Url::parse("amqp://localhost:25672").unwrap();
    connection
        .open("test".to_string(), url, None)
        .await
        .unwrap();
}

async fn amqp_connection_open_with_error() {
    let connection = AmqpConnection::new();
    let url = Url::parse("amqp://localhost:32767").unwrap();
    assert!(connection
        .open("test".to_string(), url, None)
        .await
        .is_err());
}

async fn amqp_connection_close() {
    let connection = AmqpConnection::new();
    let url = Url::parse("amqp://localhost:25672").unwrap();
    connection
        .open("test".to_string(), url, None)
        .await
        .unwrap();
    connection.close().await.unwrap();
}

async fn amqp_connection_close_with_error() {
    let connection = AmqpConnection::new();
    let url = Url::parse("amqp://localhost:25672").unwrap();
    connection
        .open("test".to_string(), url, None)
        .await
        .unwrap();
    let res = connection
        .close_with_error(
            AmqpSymbol::from("amqp:internal-error"),
            Some("Internal error.".to_string()),
            None,
        )
        .await;
    match res {
        Ok(_) => {}
        Err(err) => {
            assert!(err.to_string().contains("Internal error."));
        }
    }
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    tokio::join!(
        amqp_connection_open(),
        amqp_connection_open_with_error(),
        amqp_connection_close(),
        amqp_connection_close_with_error()
    );
}
