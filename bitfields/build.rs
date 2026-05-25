use std::fs;

/// The tag to start extracting content from readme.
const README_RUST_DOCS_EXTRACT_START_TAG: &str = "<!-- rust-docs-start -->";

/// The tag to stop extracting content from readme.
const README_RUST_DOCS_EXTRACT_END_TAG: &str = "<!-- rust-docs-end -->";

/// The tag to start extracting bitflags content from readme.
const README_RUST_BITFLAG_DOCS_EXTRACT_START_TAG: &str = "<!-- rust-bitflags-docs-start -->";

/// The tag to stop extracting bitflags content from readme.
const README_RUST_BITFLAG_DOCS_EXTRACT_END_TAG: &str = "<!-- rust-bitflags-docs-end -->";

/// The path of the project readme.
const README_PATH: &str = "../README.md";

/// The path of bitfields_impl lib.rs whose docs will be updated.
const IMPL_LIB_RS_PATH: &str = "../bitfields_impl/src/lib.rs";

fn main() {
    // Always tell Cargo to re-run when the env var or README changes.
    println!("cargo:rerun-if-env-changed=UPDATE_DOCS");
    println!("cargo:rerun-if-changed={README_PATH}");
    println!("cargo:rerun-if-changed={IMPL_LIB_RS_PATH}");

    // Only perform the injection when UPDATE_DOCS is set.
    // `UPDATE_DOCS=1 cargo build`
    if std::env::var("UPDATE_DOCS").is_err() {
        return;
    }

    update_lib_docs();
}

/// Extracts all content found between occurrences of `start_tag` and `end_tag`
/// in `source`.
fn extract_content(source: &str, start_tag: &str, end_tag: &str) -> String {
    let mut output = String::new();
    let mut rest = source;

    while let Some(start) = rest.find(start_tag) {
        rest = &rest[start + start_tag.len()..];
        if let Some(end) = rest.find(end_tag) {
            output.push_str(rest[..end].trim());
            output.push_str("\n\n");
            rest = &rest[end + end_tag.len()..];
        }
    }

    output.trim().to_string()
}

/// Inserts `# use bitfields_impl as bitfields;` at the top of every rust code
/// block in `content`.
fn inject_rust_block_prelude(content: &str) -> String {
    let marker = "```rust";
    let prelude = "# use bitfields_impl as bitfields;\n";
    let mut output = String::with_capacity(content.len());
    let mut rest = content;

    while let Some(pos) = rest.find(marker) {
        let after_marker = pos + marker.len();
        output.push_str(&rest[..after_marker]);
        let newline_offset =
            rest[after_marker..].find('\n').map(|i| i + 1).unwrap_or(rest[after_marker..].len());
        let next = after_marker + newline_offset;
        output.push_str(&rest[after_marker..next]);
        output.push_str(prelude);
        rest = &rest[next..];
    }
    output.push_str(rest);
    output
}

/// Formats a block of content as Rust `///` doc-comment lines.
fn format_as_doc_comments(content: &str) -> String {
    if content.is_empty() {
        return String::new();
    }
    content
        .lines()
        .map(|line| if line.is_empty() { "///".to_string() } else { format!("/// {line}") })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Replaces the content between the doc-comment versions of `start_tag` and
/// `end_tag` in `source` with `new_content` (formatted as doc comments).
fn replace_between_tags(source: &str, start_tag: &str, end_tag: &str, new_content: &str) -> String {
    let doc_start = format!("/// {start_tag}");
    let doc_end = format!("/// {end_tag}");

    if let Some(start_pos) = source.rfind(&doc_start) {
        let after_start = start_pos + doc_start.len();
        if let Some(rel_end) = source[after_start..].find(&doc_end) {
            let end_pos = after_start + rel_end;
            let before = &source[..after_start];
            let after = &source[end_pos..];
            let middle = if new_content.is_empty() {
                "\n".to_string()
            } else {
                format!("\n{}\n", format_as_doc_comments(new_content))
            };
            return format!("{before}{middle}{after}");
        }
    }

    source.to_string()
}

/// Reads the README, extracts the documented sections, and writes them back
/// into lib.rs between the matching tags.
fn update_lib_docs() {
    let readme = fs::read_to_string(README_PATH).expect("Unable to find README");
    let impl_lib_rs =
        fs::read_to_string(IMPL_LIB_RS_PATH).expect("Unable to find bitfields_impl lib.rs");

    let docs_content = inject_rust_block_prelude(&extract_content(
        &readme,
        README_RUST_DOCS_EXTRACT_START_TAG,
        README_RUST_DOCS_EXTRACT_END_TAG,
    ));
    let bitflag_content = inject_rust_block_prelude(&extract_content(
        &readme,
        README_RUST_BITFLAG_DOCS_EXTRACT_START_TAG,
        README_RUST_BITFLAG_DOCS_EXTRACT_END_TAG,
    ));

    let impl_updated = replace_between_tags(
        &impl_lib_rs,
        README_RUST_DOCS_EXTRACT_START_TAG,
        README_RUST_DOCS_EXTRACT_END_TAG,
        &docs_content,
    );
    let impl_updated = replace_between_tags(
        &impl_updated,
        README_RUST_BITFLAG_DOCS_EXTRACT_START_TAG,
        README_RUST_BITFLAG_DOCS_EXTRACT_END_TAG,
        &bitflag_content,
    );
    fs::write(IMPL_LIB_RS_PATH, impl_updated).expect("Unable to write bitfields_impl lib.rs");
}
