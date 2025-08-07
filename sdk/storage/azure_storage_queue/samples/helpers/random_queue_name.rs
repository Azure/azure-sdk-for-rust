/// Generates a random queue name with a suffix to ensure uniqueness.
pub fn get_random_queue_name() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let random_suffix: u32 = rng.random_range(1000..9999);
    format!("sdk-test-queue-{}", random_suffix)
}
