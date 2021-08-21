#[cfg(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
))]
const ENV_TRANSACTION_NUMBER: &str = "AzureSDKforRustTransactionNumber";
#[cfg(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
))]
const ENV_DUMP_PATH_ROOT: &str = "AzureSDKforRustDumpPathRoot";
#[cfg(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
))]
const ENV_TRANSACTION_NAME: &str = "AzureSDKforRustTransactionName";

#[cfg(not(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
)))]
pub fn start_transaction(_transaction_name: &str) {}

#[cfg(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
))]
pub fn start_transaction(transaction_name: &str) {
    std::env::set_var(ENV_TRANSACTION_NUMBER, "0");
    std::env::set_var(ENV_TRANSACTION_NAME, transaction_name);
}

#[cfg(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
))]
pub(crate) fn get_transaction_num() -> u32 {
    std::env::var(ENV_TRANSACTION_NUMBER)
            .map(|t| t.parse::<u32>())
            .expect("Error reading transaction number from the environment. Is the variable set to a number?")
            .expect("Error reading transaction number from the environment. Is the variable set to a number?")
}

#[cfg(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
))]
pub(crate) fn increment_transaction() {
    std::env::set_var(
        ENV_TRANSACTION_NUMBER,
        format!("{}", get_transaction_num() + 1),
    );
}

#[cfg(any(
    feature = "mock_transport_generate",
    feature = "mock_transport_consume"
))]
pub(crate) fn prepare_and_get_transaction_path() -> std::path::PathBuf {
    let path: std::path::PathBuf = std::path::PathBuf::from(
        std::env::var(ENV_DUMP_PATH_ROOT)
            .expect(&format!(
                "Dump path root environmental variable not set ({})",
                ENV_DUMP_PATH_ROOT
            ))
            .to_owned(),
    )
    .join(
        std::env::var(ENV_TRANSACTION_NAME)
            .expect("Transaction name environmental variable not set")
            .to_owned(),
    );

    if !path.exists() {
        std::fs::create_dir(&path).expect("cannot create transaction directory");
    }

    path
}
