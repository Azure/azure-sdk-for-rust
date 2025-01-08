//    #[tokio::test]
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

//    #[tokio::test]
async fn amqp_connection_open_with_error() {
    let connection = AmqpConnection::new();
    let url = Url::parse("amqp://localhost:32767").unwrap();
    assert!(connection
        .open("test".to_string(), url, None)
        .await
        .is_err());
}

//    #[tokio::test]
async fn amqp_connection_close() {
    let connection = AmqpConnection::new();
    let url = Url::parse("amqp://localhost:25672").unwrap();
    connection
        .open("test".to_string(), url, None)
        .await
        .unwrap();
    connection.close().await.unwrap();
}

//    #[tokio::test]
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

    amqp_connection_open().await;
    amqp_connection_open_with_error().await;
    amqp_connection_close().await;
    amqp_connection_close_with_error().await;
}
