use json2md::{Error, View};
use serde_json::{Value, json};

const SUMMARY_CASES: [(&str, &str, &str); 4] = [
    (
        "initial",
        include_str!("../../../../examples/gitlab-review/initial.json"),
        include_str!("../../../../examples/gitlab-review/initial.md"),
    ),
    (
        "follow-up",
        include_str!("../../../../examples/gitlab-review/follow-up.json"),
        include_str!("../../../../examples/gitlab-review/follow-up.md"),
    ),
    (
        "pending",
        include_str!("../../../../examples/gitlab-review/pending.json"),
        include_str!("../../../../examples/gitlab-review/pending.md"),
    ),
    (
        "no-findings",
        include_str!("../../../../examples/gitlab-review/no-findings.json"),
        include_str!("../../../../examples/gitlab-review/no-findings.md"),
    ),
];

fn finding_view() -> View {
    let schema = serde_json::from_str(include_str!(
        "../../../../examples/gitlab-review/finding.schema.json"
    ))
    .unwrap();
    View::new(
        &schema,
        include_str!("../../../../examples/gitlab-review/finding.j2"),
    )
    .unwrap()
}

fn summary_view() -> View {
    let schema = serde_json::from_str(include_str!(
        "../../../../examples/gitlab-review/summary.schema.json"
    ))
    .unwrap();
    View::new(
        &schema,
        include_str!("../../../../examples/gitlab-review/summary.j2"),
    )
    .unwrap()
}

fn markers(markdown: &str, kind: &str) -> Vec<Value> {
    let prefix = format!("<!-- {kind} ");
    markdown
        .lines()
        .filter_map(|line| line.strip_prefix(&prefix))
        .map(|json| serde_json::from_str(json.strip_suffix(" -->").unwrap()).unwrap())
        .collect()
}

#[test]
fn discovery_markers_are_serialized_as_json_inside_markdown() {
    let marker = json!({
        "finding_id": "F7",
        "round_id": "R3",
        "action_id": "R3-F7-create"
    });
    let view = View::new(
        &json!({"type": "object", "required": ["marker"]}),
        "<!-- review-finding {{ marker | tojson }} -->\n",
    )
    .unwrap();

    let markdown = view.render(&json!({"marker": marker})).unwrap();
    let serialized = markdown
        .strip_prefix("<!-- review-finding ")
        .unwrap()
        .strip_suffix(" -->\n")
        .unwrap();
    assert_eq!(serde_json::from_str::<Value>(serialized).unwrap(), marker);
}

#[test]
fn finding_files_preserve_the_comment_body_footer_and_discovery_marker() {
    let input: Value = serde_json::from_str(include_str!(
        "../../../../examples/gitlab-review/finding.json"
    ))
    .unwrap();
    let original = input.clone();
    let view = finding_view();
    let markdown = view.render(&input).unwrap();

    assert_eq!(
        markdown,
        include_str!("../../../../examples/gitlab-review/finding.md")
    );
    assert_eq!(
        markers(&markdown, "review-finding"),
        [input["marker"].clone()]
    );
    assert_eq!(input, original);
}

#[test]
fn summary_files_cover_initial_follow_up_pending_and_empty_reviews() {
    let view = summary_view();
    for (name, data, expected) in SUMMARY_CASES {
        let input: Value = serde_json::from_str(data).unwrap();
        let original = input.clone();
        let markdown = view.render(&input).unwrap();

        assert_eq!(markdown, expected, "{name}");
        assert_eq!(
            markers(&markdown, "review-summary"),
            [input["marker"].clone()]
        );
        let expected_rounds: Vec<_> = input["rounds"]
            .as_array()
            .unwrap()
            .iter()
            .map(|round| round["marker"].clone())
            .collect();
        for round in &expected_rounds {
            let ids: Vec<_> = round["policy_ids"]
                .as_array()
                .unwrap()
                .iter()
                .map(|id| id.as_str().unwrap())
                .collect();
            let mut sorted = ids.clone();
            sorted.sort_unstable();
            assert_eq!(
                ids, sorted,
                "{name}: supplied policy IDs must follow the review contract"
            );
        }
        assert_eq!(
            markers(&markdown, "review-round"),
            expected_rounds,
            "{name}"
        );
        assert_eq!(input, original, "{name}");
    }
}

#[test]
fn unchanged_input_renders_identically_after_other_review_states() {
    let view = summary_view();
    let first: Value = serde_json::from_str(SUMMARY_CASES[0].1).unwrap();
    let initial = view.render(&first).unwrap();

    for (_, data, _) in SUMMARY_CASES {
        view.render(&serde_json::from_str(data).unwrap()).unwrap();
    }

    assert_eq!(view.render(&first).unwrap(), initial);
}

#[test]
fn invalid_review_data_fails_before_a_comment_body_is_returned() {
    let finding = finding_view();
    let mut input: Value = serde_json::from_str(include_str!(
        "../../../../examples/gitlab-review/finding.json"
    ))
    .unwrap();
    finding.render(&input).expect("valid control");
    input["severity"] = json!("critical");
    let Error::Validation(errors) = finding.render(&input).unwrap_err() else {
        panic!("invalid severity must fail validation");
    };
    assert_eq!(errors.issues()[0].instance_path(), "/severity");

    let summary = summary_view();
    let input: Value = serde_json::from_str(SUMMARY_CASES[1].1).unwrap();
    summary.render(&input).expect("valid control");
    let mut missing_depth = input.clone();
    missing_depth["metadata"]
        .as_object_mut()
        .unwrap()
        .remove("depth");
    let Error::Validation(errors) = summary.render(&missing_depth).unwrap_err() else {
        panic!("missing metadata must fail validation");
    };
    assert_eq!(errors.issues()[0].instance_path(), "/metadata");
    let mut empty_rounds = input;
    empty_rounds["rounds"] = json!([]);
    assert!(matches!(
        summary.render(&empty_rounds),
        Err(Error::Validation(_))
    ));
}

#[test]
fn finding_markdown_and_explicit_disclosure_survive_the_note_payload() {
    let body = "The caller loses the rejected value. Retain its diagnostic.\n\n```rust\nlet label = \"café <ready> & review\";\n```";
    let mut input: Value = serde_json::from_str(include_str!(
        "../../../../examples/gitlab-review/finding.json"
    ))
    .unwrap();
    input["body"] = json!(body);
    let view = finding_view();
    let plain = view.render(&input).unwrap();
    assert!(plain.contains(&format!("\n\n{body}\n\n---\n")));
    assert!(!plain.contains("Review disclosure:"));

    input["disclosure"] = json!("Review disclosure: local test fixture.");
    let disclosed = view.render(&input).unwrap();
    assert_eq!(
        disclosed,
        plain.replacen(
            "\n\n---\n",
            "\n\nReview disclosure: local test fixture.\n\n---\n",
            1
        )
    );
    let payload = serde_json::to_vec(&json!({"body": disclosed})).unwrap();
    let decoded: Value = serde_json::from_slice(&payload).unwrap();
    assert_eq!(decoded["body"].as_str().unwrap(), disclosed);
    assert_eq!(
        markers(&disclosed, "review-finding"),
        [input["marker"].clone()]
    );
}

#[test]
fn summary_markdown_survives_serialization_into_a_gitlab_note_body() {
    let view = summary_view();
    for (name, data, expected) in SUMMARY_CASES {
        let markdown = view.render(&serde_json::from_str(data).unwrap()).unwrap();
        let payload = serde_json::to_vec(&json!({"body": markdown})).unwrap();
        let decoded: Value = serde_json::from_slice(&payload).unwrap();
        assert_eq!(decoded["body"].as_str().unwrap(), expected, "{name}");
    }
}

#[test]
fn history_and_metadata_are_separate_balanced_sibling_blocks() {
    let input: Value = serde_json::from_str(SUMMARY_CASES[1].1).unwrap();
    let markdown = summary_view().render(&input).unwrap();
    let mut depth = 0;
    let mut blocks = 0;
    for line in markdown.lines() {
        match line {
            "<details>" => {
                depth += 1;
                blocks += 1;
                assert_eq!(depth, 1, "history must not be nested");
            }
            "</details>" => {
                assert_eq!(depth, 1, "closing tag must have an opening tag");
                depth -= 1;
            }
            _ => {}
        }
    }
    assert_eq!(depth, 0);
    assert_eq!(blocks, 3);
}
