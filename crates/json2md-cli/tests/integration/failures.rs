use std::{fs, path::Path};

use super::support::{Assets, assert_contains, assert_failure, assert_path, read};

const EXISTING_MARKDOWN: &[u8] = b"# Existing output\n\nKeep these bytes.\n";

fn rejection_diagnostics(assets: &Assets) -> [String; 2] {
    let before = assets.snapshot();
    let destination = assets.path("existing.md");
    fs::write(&destination, EXISTING_MARKDOWN).expect("prepare existing Markdown");

    [None, Some(destination.as_path())].map(|path| {
        let output = assets.render(path);
        let stderr = assert_failure(&output);
        assert_eq!(read(&destination), EXISTING_MARKDOWN);
        assets.assert_unchanged(&before);
        stderr
    })
}

#[test]
fn json_syntax_errors_identify_the_file_line_column_and_cause() {
    for is_schema in [false, true] {
        let assets = Assets::new();
        let invalid = if is_schema {
            &assets.schema
        } else {
            &assets.input
        };
        assets.replace(invalid, "invalid-json.json");

        for stderr in rejection_diagnostics(&assets) {
            assert_path(&stderr, invalid);
            for token in ["JSON", "line", "column", "expected"] {
                assert_contains(&stderr, token);
            }
        }
    }
}

#[test]
fn validation_reports_every_field_and_schema_pointer_before_rendering() {
    let assets = Assets::new();
    assets.replace(&assets.input, "invalid-document.json");
    assets.replace(&assets.template, "runtime-error.j2");

    for stderr in rejection_diagnostics(&assets) {
        assert_path(&stderr, &assets.input);
        assert_path(&stderr, &assets.schema);
        for pointer in ["/count", "/tags/0", "/nested/a~1b~0c"] {
            assert_contains(&stderr, &format!("{}#{pointer}", assets.input.display()));
        }
        assert_contains(
            &stderr,
            &format!("{}# (document root)", assets.input.display()),
        );
        for pointer in [
            "/required",
            "/properties/count/type",
            "/properties/tags/items/type",
            "/properties/nested/properties/a~1b~0c/type",
        ] {
            assert_contains(&stderr, &format!("#{pointer}"));
        }
        for cause in ["title", "integer", "string"] {
            assert_contains(&stderr, cause);
        }
    }
}

#[test]
fn invalid_schema_reports_its_compilation_cause() {
    let assets = Assets::new();
    assets.replace(&assets.schema, "invalid-schema.json");

    for stderr in rejection_diagnostics(&assets) {
        assert_path(&stderr, &assets.schema);
        assert_contains(&stderr, "Schema");
        assert_contains(&stderr, "42");
    }
}

#[test]
fn invalid_template_reports_its_syntax_cause() {
    let assets = Assets::new();
    assets.replace(&assets.template, "invalid-template.j2");

    for stderr in rejection_diagnostics(&assets) {
        assert_path(&stderr, &assets.template);
        assert_contains(&stderr, "template");
        assert_contains(&stderr, "unexpected");
    }
}

#[test]
fn runtime_failure_discards_an_already_rendered_template_prefix() {
    let assets = Assets::new();
    assets.replace(&assets.template, "runtime-error.j2");

    for stderr in rejection_diagnostics(&assets) {
        assert_path(&stderr, &assets.template);
        assert_contains(&stderr, "render");
        assert_contains(&stderr, "undefined");
    }
}

#[test]
fn missing_input_schema_and_template_files_keep_existing_output() {
    for missing_index in 0..3 {
        let assets = Assets::new();
        let before = assets.snapshot();
        let paths = assets.paths();
        let missing = paths[missing_index];
        fs::remove_file(missing).expect("remove only the selected fixture asset");
        let destination = assets.path("existing.md");
        fs::write(&destination, EXISTING_MARKDOWN).expect("prepare existing Markdown");

        for output_path in [None, Some(destination.as_path())] {
            let output = assets.render(output_path);
            let stderr = assert_failure(&output);
            assert_path(&stderr, missing);
            assert_contains(&stderr, "read");
            assert_contains(&stderr, "os error");
            assert!(!missing.exists(), "missing input was recreated");
            assert_eq!(read(&destination), EXISTING_MARKDOWN);
            for (index, path) in paths.into_iter().enumerate() {
                if index != missing_index {
                    assert_eq!(read(path), before[index]);
                }
            }
        }
    }
}

#[test]
fn output_directory_and_missing_parent_fail_without_markdown_or_asset_changes() {
    let assets = Assets::new();
    let before = assets.snapshot();
    let directory = assets.path("destination-directory");
    fs::create_dir(&directory).expect("prepare a directory as an invalid destination");
    let existing = directory.join("existing.md");
    fs::write(&existing, EXISTING_MARKDOWN).expect("prepare Markdown inside the directory");
    let missing_parent = assets.path("missing-parent/output.md");

    for destination in [&directory, &missing_parent] {
        let output = assets.render(Some(destination));
        let stderr = assert_failure(&output);
        assert_path(&stderr, destination);
        assert_contains(&stderr, "output");
        assert_contains(&stderr, "os error");
        assert_eq!(read(&existing), EXISTING_MARKDOWN);
        assets.assert_unchanged(&before);
    }
    assert!(directory.is_dir());
    assert!(!missing_parent.exists());
}

fn assert_output_alias_preserves_assets(assets: &Assets, destination: &Path) {
    let before = assets.snapshot();
    let original_output = read(destination);

    let output = assets.render(Some(destination));

    let stderr = assert_failure(&output);
    assert_path(&stderr, destination);
    assert_contains(&stderr, "output");
    assert_contains(&stderr, "supplied file");
    assert_eq!(read(destination), original_output);
    assets.assert_unchanged(&before);
}

#[test]
fn output_cannot_overwrite_the_input_schema_or_template() {
    let assets = Assets::new();

    for destination in assets.paths() {
        assert_output_alias_preserves_assets(&assets, destination);
    }
}

#[cfg(unix)]
#[test]
fn output_symlinks_cannot_overwrite_the_input_schema_or_template() {
    use std::os::unix::fs::symlink;

    let assets = Assets::new();

    for (index, target) in assets.paths().into_iter().enumerate() {
        let destination = assets.path(&format!("alias-{index}.md"));
        symlink(target, &destination).expect("create an output symlink to the supplied asset");
        assert_output_alias_preserves_assets(&assets, &destination);
        assert!(destination.is_symlink());
    }
}
