//! This build script extracts content from the project README.md into its own
//! file so that it can be included in the bitfield library.

use std::fs;
use std::path::Path;

/// The tag to start extracting content from readme.
const README_RUST_DOCS_EXTRACT_START_TAG: &str = "<!-- rust-docs-start -->";

/// The tag to stop extracting content from readme.
const README_RUST_DOCS_EXTRACT_END_TAG: &str = "<!-- rust-docs-end -->";

/// The path of the project readme.
const README_PATH: &str = "../README.md";

/// The output path of the extracted readme fragment content.
const README_FRAGMENT_OUTPUT_PATH: &str = "../target/docs_fragment.md";

fn main() {
    if !Path::new(README_FRAGMENT_OUTPUT_PATH).exists() {
        // force regeneration by not emitting rerun-if-unchanged
    }

    let readme = fs::read_to_string(README_PATH).unwrap();

    let mut output = String::new();
    let mut rest = readme.as_str();

    while let Some(start) = rest.find(README_RUST_DOCS_EXTRACT_START_TAG) {
        rest = &rest[start + README_RUST_DOCS_EXTRACT_START_TAG.len()..];
        if let Some(end) = rest.find(README_RUST_DOCS_EXTRACT_END_TAG) {
            output.push_str(rest[..end].trim());
            output.push_str("\n\n");
            rest = &rest[end + README_RUST_DOCS_EXTRACT_END_TAG.len()..];
        }
    }

    fs::write(README_FRAGMENT_OUTPUT_PATH, output.trim()).unwrap();
    println!("cargo:rerun-if-changed={}", README_PATH);
}
