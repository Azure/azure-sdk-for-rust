// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use tokio::task::LocalSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    spawn_local(async {
        let local_set = LocalSet::new();
        local_set
            .run_until(async {
                local_set_main().await.unwrap();
            })
            .await
    })
}

async fn local_set_main() -> Result<(), Box<dyn std::error::Error>> {
    use azure_messaging_servicebus::prelude::*;
    use web_sys::console;

    console::log_1(&"Hello, world!".into());

    // This must be called within a `LocalSet` to work.
    let mut client =
        ServiceBusClient::new_from_connection_string("<NAMESPACE-CONNECTION-STRING>", Default::default()).await?;
    console::log_1(&"Client created".into());

    // This must be called within a `LocalSet` to work.
    let mut sender = client
        .create_sender("<QUEUE-NAME>", Default::default())
        .await?;
    sender.send_message("hello ServiceBus from WASM!").await?;
    console::log_1(&"Message sent".into());
    sender.dispose().await?;

    // This must be called within a `LocalSet` to work.
    let mut receiver = client
        .create_receiver_for_queue("<QUEUE-NAME>", Default::default())
        .await?;
    let message = receiver.receive_message().await?;
    receiver.complete_message(&message).await?;
    receiver.dispose().await?;

    let body = message.body()?;
    let body = format!("Received message: {:?}", std::str::from_utf8(body)?);
    console::log_1(&body.into());

    client.dispose().await?;

    console::log_1(&"Client disposed".into());

    Ok(())
}
