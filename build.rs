fn main() {
    let target = std::env::var("TARGET").expect("Cargo target");
    assert_eq!(
        target, "x86_64-unknown-linux-gnu",
        "rless currently supports Linux x86_64/glibc"
    );
    println!("cargo:rustc-link-lib=tinfo");
}
