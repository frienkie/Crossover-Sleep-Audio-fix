fn main() {
    println!("cargo:rerun-if-changed=src/logging.m");
    cc::Build::new()
        .file("src/logging.m")
        .flag("-fobjc-arc")
        .flag("-std=gnu11")
        .flag("-mmacosx-version-min=14.2")
        .warnings_into_errors(true)
        .compile("crosspuck_logging");
}
