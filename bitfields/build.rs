use std::path::Path;

/// Out dir infix for Rust being built by a publish command.
const PUBLISH_OUT_DIR_INFIX: &str = "target/package/";

/// I have doc tests in the README file, but the README file is outside the
/// bitfields crate and cargo refuses to include it in the crate. So I copy the
/// README file to the target directory and include it from there.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the target directory
    let out_dir = std::env::var("OUT_DIR")?;

    // Source path of the README file
    let src_path = if out_dir.contains(PUBLISH_OUT_DIR_INFIX) {
        println!("cargo:rerun-if-changed=../../../README.md");
        Path::new("../../../README.md")
    } else {
        println!("cargo:rerun-if-changed=../README.md");
        Path::new("../README.md")
    };

    // Destination path in the target directory
    let dst_path = Path::new(&out_dir).join("README.md");

    // Copy the README to the target directory
    std::fs::copy(src_path, dst_path)?;

    Ok(())
}
