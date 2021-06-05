use rustc_version::version;

fn main() {
    let version = match version() {
        Ok(version) => version.to_string(),
        Err(_) => "unknown".to_owned(),
    };
    println!("cargo:rustc-env=AZSDK_RUSTC_VERSION={}", version);
}
