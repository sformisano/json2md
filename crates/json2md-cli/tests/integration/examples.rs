use super::support::{assert_success, read, render, repository_root};

#[test]
fn committed_gitlab_finding_and_summary_examples_render_byte_for_byte() {
    let root = repository_root().join("examples/gitlab-review");

    for (name, pair) in [
        ("finding", "finding"),
        ("initial", "summary"),
        ("follow-up", "summary"),
        ("pending", "summary"),
        ("no-findings", "summary"),
    ] {
        let directory = tempfile::tempdir().expect("create an owned example-output directory");
        let input = root.join(format!("{name}.json"));
        let schema = root.join(format!("{pair}.schema.json"));
        let template = root.join(format!("{pair}.j2"));
        let expected = read(&root.join(format!("{name}.md")));
        let before = [&input, &schema, &template].map(|path| read(path));

        let stdout = render(&input, &schema, &template, None);
        assert_success(&stdout, &expected);

        let destination = directory.path().join("output.md");
        let output = render(&input, &schema, &template, Some(&destination));
        assert_success(&output, b"");
        assert_eq!(read(&destination), expected, "{name}");

        for (path, original) in [&input, &schema, &template].into_iter().zip(before) {
            assert_eq!(read(path), original, "{name}: {}", path.display());
        }
    }
}
