fn main() {
    // driver3
    let current = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", current);

    // driver4
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
