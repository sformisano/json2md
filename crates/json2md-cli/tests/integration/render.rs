use std::fs;

use super::support::{Assets, assert_success, fixture_path, read};

#[test]
fn stdout_preserves_unicode_quotes_multiline_markdown_and_trailing_newlines() {
    let assets = Assets::new();
    let before = assets.snapshot();
    let expected = read(&fixture_path("document.md"));

    let output = assets.render(None);

    assert_success(&output, &expected);
    assert!(output.stdout.ends_with(b"\n\n"));
    assets.assert_unchanged(&before);
}

#[test]
fn file_output_is_exact_and_replaces_existing_content_without_stdout() {
    let assets = Assets::new();
    let before = assets.snapshot();
    let destination = assets.path("résultat 東京.md");
    let expected = read(&fixture_path("document.md"));

    let output = assets.render(Some(&destination));

    assert_success(&output, b"");
    assert_eq!(read(&destination), expected);
    assets.assert_unchanged(&before);

    fs::write(&destination, b"existing Markdown that must be replaced\n")
        .expect("prepare an existing destination");
    let output = assets.render(Some(&destination));

    assert_success(&output, b"");
    assert_eq!(read(&destination), expected);
    assets.assert_unchanged(&before);
}

#[test]
fn rendering_does_not_add_a_newline() {
    let assets = Assets::new();
    assets.replace(&assets.template, "no-newline.j2");
    let before = assets.snapshot();
    let expected = read(&fixture_path("no-newline.md"));

    let stdout = assets.render(None);
    assert_success(&stdout, &expected);
    assert!(!stdout.stdout.ends_with(b"\n"));

    let destination = assets.path("output.md");
    let output = assets.render(Some(&destination));
    assert_success(&output, b"");
    assert_eq!(read(&destination), expected);
    assets.assert_unchanged(&before);
}
