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

    let mut client = ServiceBusClient::new(
        "Endpoint=sb://fe2o3-amqp-example.servicebus.windows.net/;SharedAccessKeyName=RootManageSharedAccessKey;SharedAccessKey=NwTiLlqS0fqL56yr+oqYdzSOJWqckVzyyqLyZITwox0=",
        Default::default()
    ).await?;

    console::log_1(&"Client created".into());

    client.dispose().await?;

    console::log_1(&"Client disposed".into());

    Ok(())
}
