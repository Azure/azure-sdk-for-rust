pub fn setup_dotenv() {
    let _ = dotenv::from_filename("./sdk/messaging_eventhubs/.env");
}
