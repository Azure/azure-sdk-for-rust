pub fn setup_dotenv() {
    dotenv::from_filename("./sdk/messaging_servicebus/tests/.env").ok();
}
