pub fn setup() {
    println!("Setting up tests...");
    let _ = env_logger::builder().is_test(true).try_init();
}
