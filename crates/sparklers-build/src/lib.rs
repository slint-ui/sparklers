/// The path to the framework directory
pub const SPARKLE_FRAMEWORK_DIR: &str = env!("SPARKLE_FRAMEWORK_DIR");

pub fn emit_rpath() {
    println!("cargo::rustc-link-arg=-Wl,-rpath,{}", SPARKLE_FRAMEWORK_DIR);
}
