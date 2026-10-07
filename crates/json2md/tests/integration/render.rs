use json2md::{Error, View};
use serde_json::{Value, json};

#[test]
fn renders_nested_review_fixture_as_exact_markdown() {
    let schema: Value = serde_json::from_str(include_str!("../fixtures/review.schema.json"))
        .expect("valid review schema fixture");
    let input: Value = serde_json::from_str(include_str!("../fixtures/review.json"))
        .expect("valid review data fixture");
    let view =
        View::new(&schema, include_str!("../fixtures/review.j2")).expect("compile the review view");

    let markdown = view.render(&input).expect("render a valid review");

    assert_eq!(markdown, include_str!("../fixtures/review.md"));
    assert!(markdown.ends_with('\n'));
}

#[test]
fn reuses_a_compiled_view_for_full_and_empty_reports() {
    let schema: Value = serde_json::from_str(include_str!("../fixtures/review.schema.json"))
        .expect("valid review schema fixture");
    let full: Value = serde_json::from_str(include_str!("../fixtures/review.json"))
        .expect("valid review data fixture");
    let empty: Value = serde_json::from_str(include_str!("../fixtures/empty-review.json"))
        .expect("valid empty review fixture");
    let view =
        View::new(&schema, include_str!("../fixtures/review.j2")).expect("compile the review view");

    assert_eq!(
        view.render(&full).unwrap(),
        include_str!("../fixtures/review.md")
    );
    assert_eq!(
        view.render(&empty).unwrap(),
        include_str!("../fixtures/empty-review.md")
    );
    assert_eq!(
        view.render(&full).unwrap(),
        include_str!("../fixtures/review.md")
    );
}

#[test]
fn owns_the_schema_and_template_after_construction() {
    let view = {
        let schema = json!({"type": "object", "required": ["name"]});
        let template = String::from("Hello {{ name }}!\n");
        View::new(&schema, &template).expect("compile owned resources")
    };

    assert_eq!(
        view.render(&json!({"name": "Ada"})).unwrap(),
        "Hello Ada!\n"
    );
}

#[test]
fn preserves_markdown_and_html_characters_without_autoescaping() {
    let view = View::new(
        &json!({"type": "object", "required": ["text"]}),
        "{{ text }}\n",
    )
    .unwrap();
    let text = "**bold** | `code` <em>HTML</em> & _italic_ [link](https://example.test)";

    assert_eq!(
        view.render(&json!({"text": text})).unwrap(),
        format!("{text}\n")
    );
}

#[test]
fn missing_nested_template_value_is_a_render_error() {
    let view = View::new(
        &json!({"type": "object", "properties": {"user": {"type": "object"}}}),
        "{{ user.name }}",
    )
    .unwrap();
    let input = json!({"user": {}});
    view.validate(&input)
        .expect("the schema allows an empty user");

    assert!(matches!(view.render(&input), Err(Error::Render { .. })));
}

#[test]
fn distinguishes_explicit_null_from_a_missing_variable() {
    let view = View::new(
        &json!({"type": "object"}),
        "{% if value is none %}null{% else %}{{ value }}{% endif %}",
    )
    .unwrap();

    assert_eq!(view.render(&json!({"value": null})).unwrap(), "null");
    assert!(matches!(view.render(&json!({})), Err(Error::Render { .. })));
}

#[test]
fn validation_failure_precedes_template_execution() {
    let view = View::new(
        &json!({"type": "object", "required": ["name"]}),
        "{{ missing.deep }}",
    )
    .unwrap();

    assert!(matches!(view.render(&json!({})), Err(Error::Validation(_))));
    assert!(matches!(
        view.render(&json!({"name": "Ada"})),
        Err(Error::Render { .. })
    ));
}

#[test]
fn renders_non_ascii_keys_and_strings_through_bracket_access() {
    let view = View::new(
        &json!({
            "type": "object",
            "required": ["record"],
            "properties": {
                "record": {
                    "type": "object",
                    "required": ["città"],
                    "properties": {"città": {"type": "string"}}
                }
            }
        }),
        "{{ record[\"città\"] }}\n",
    )
    .unwrap();

    assert_eq!(
        view.render(&json!({"record": {"città": "München, 東京 🌍"}}))
            .unwrap(),
        "München, 東京 🌍\n"
    );
}

#[test]
fn invalid_runtime_operation_is_a_render_error() {
    let view = View::new(
        &json!({"type": "object", "properties": {"divisor": {"type": "integer"}}}),
        "{{ 1 // divisor }}",
    )
    .unwrap();

    assert!(matches!(
        view.render(&json!({"divisor": 0})),
        Err(Error::Render { .. })
    ));
    assert!(view.render(&json!({"divisor": 2})).is_ok());
}
