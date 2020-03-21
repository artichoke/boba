#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
#[should_panic] // Not supported yet since not on crates.io
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
