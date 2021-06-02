use rustc_version::version;

fn main() {
    println!("cargo:rustc-env=AZSDK_RUSTC_VERSION={}", version().unwrap());
}
