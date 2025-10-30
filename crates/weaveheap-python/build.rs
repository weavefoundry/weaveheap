#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-arg=-undefined");
    println!("cargo:rustc-link-arg=dynamic_lookup");
}

#[cfg(not(target_os = "macos"))]
fn main() {}
