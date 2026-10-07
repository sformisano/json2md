use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use tempfile::TempDir;

pub(super) struct Assets {
    directory: TempDir,
    pub(super) input: PathBuf,
    pub(super) schema: PathBuf,
    pub(super) template: PathBuf,
}

impl Assets {
    pub(super) fn new() -> Self {
        let directory = tempfile::tempdir().expect("create an owned case directory");
        let assets = Self {
            input: directory.path().join("input.json"),
            schema: directory.path().join("schema.json"),
            template: directory.path().join("view.j2"),
            directory,
        };
        assets.replace(&assets.input, "document.json");
        assets.replace(&assets.schema, "document.schema.json");
        assets.replace(&assets.template, "document.j2");
        assets
    }

    pub(super) fn replace(&self, path: &Path, fixture: &str) {
        fs::copy(fixture_path(fixture), path).expect("copy a committed fixture into the case");
    }

    pub(super) fn path(&self, name: &str) -> PathBuf {
        self.directory.path().join(name)
    }

    pub(super) fn paths(&self) -> [&Path; 3] {
        [&self.input, &self.schema, &self.template]
    }

    pub(super) fn snapshot(&self) -> [Vec<u8>; 3] {
        self.paths().map(read)
    }

    pub(super) fn assert_unchanged(&self, before: &[Vec<u8>; 3]) {
        for (path, expected) in self.paths().into_iter().zip(before) {
            assert_eq!(read(path), *expected, "asset changed: {}", path.display());
        }
    }

    pub(super) fn render(&self, output: Option<&Path>) -> Output {
        render(&self.input, &self.schema, &self.template, output)
    }
}

pub(super) fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

pub(super) fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub(super) fn render(
    input: &Path,
    schema: &Path,
    template: &Path,
    output: Option<&Path>,
) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_json2md"));
    command
        .arg("render")
        .arg(input)
        .arg("--schema")
        .arg(schema)
        .arg("--template")
        .arg(template);
    if let Some(path) = output {
        command.arg("--output").arg(path);
    }
    command.output().expect("execute the actual json2md binary")
}

pub(super) fn read(path: &Path) -> Vec<u8> {
    fs::read(path).expect("read the observed file before comparing its bytes")
}

pub(super) fn assert_success(output: &Output, stdout: &[u8]) {
    assert!(
        output.status.success(),
        "unexpected status {}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, stdout);
    assert!(
        output.stderr.is_empty(),
        "successful rendering emitted diagnostics: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

pub(super) fn assert_failure(output: &Output) -> String {
    assert!(
        !output.status.success(),
        "failure returned a successful status"
    );
    assert!(output.stdout.is_empty(), "failure emitted partial Markdown");
    let stderr = String::from_utf8(output.stderr.clone()).expect("UTF-8 diagnostics");
    assert!(!stderr.is_empty(), "failure omitted its diagnostic");
    stderr
}

pub(super) fn assert_contains(diagnostic: &str, expected: &str) {
    assert!(
        diagnostic.contains(expected),
        "diagnostic omitted {expected:?}: {diagnostic}"
    );
}

pub(super) fn assert_path(diagnostic: &str, path: &Path) {
    assert_contains(diagnostic, &path.display().to_string());
}
