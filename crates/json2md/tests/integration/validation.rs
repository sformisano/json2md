use std::error::Error as _;

use json2md::{Error, View};
use serde_json::json;

#[test]
fn reports_all_validation_issues_with_instance_and_schema_paths() {
    let view = View::new(
        &json!({
            "type": "object",
            "required": ["name"],
            "properties": {
                "age": {"type": "integer"},
                "tags": {"type": "array", "items": {"type": "string"}}
            }
        }),
        "unused",
    )
    .unwrap();

    let errors = view
        .validate(&json!({"age": "old", "tags": [42]}))
        .expect_err("three independent schema violations");
    let mut paths: Vec<_> = errors
        .issues()
        .iter()
        .map(|issue| (issue.instance_path(), issue.schema_path()))
        .collect();
    paths.sort_unstable();

    assert_eq!(
        paths,
        [
            ("", "/required"),
            ("/age", "/properties/age/type"),
            ("/tags/0", "/properties/tags/items/type"),
        ]
    );
}

#[test]
fn uses_json_pointer_escaping_in_validation_paths() {
    let view = View::new(
        &json!({
            "type": "object",
            "properties": {"a/b~c": {"type": "integer"}}
        }),
        "unused",
    )
    .unwrap();

    let errors = view.validate(&json!({"a/b~c": false})).unwrap_err();

    assert_eq!(errors.issues()[0].instance_path(), "/a~1b~0c");
    assert_eq!(errors.issues()[0].schema_path(), "/properties/a~1b~0c/type");
}

#[test]
fn owns_validation_diagnostics_after_input_and_view_are_dropped() {
    let errors = {
        let view = View::new(
            &json!({"type": "array", "items": {"type": "integer"}}),
            "unused",
        )
        .unwrap();
        let input = json!(["bad"]);
        view.validate(&input).unwrap_err()
    };

    assert_eq!(errors.issues()[0].instance_path(), "/0");
    assert_eq!(errors.issues()[0].schema_path(), "/items/type");
    assert!(!errors.to_string().is_empty());
    assert_eq!(
        errors
            .source()
            .expect("first validation issue as source")
            .to_string(),
        errors.issues()[0].to_string()
    );
}

#[test]
fn rendering_preserves_the_validation_error_source_chain() {
    let view = View::new(&json!({"type": "integer"}), "unreachable").unwrap();
    let error = view.render(&json!("bad")).unwrap_err();

    assert!(matches!(error, Error::Validation(_)));
    let validation = error.source().expect("validation collection");
    assert!(validation.source().is_some());
}

#[test]
fn embedded_schema_ids_make_keyword_paths_relative_to_the_resource() {
    let schema = json!({
        "$defs": {"entry": {"$id": "https://example.test/entry", "type": "integer"}},
        "$ref": "https://example.test/entry"
    });
    let view = View::new(&schema, "valid").unwrap();
    assert_eq!(view.render(&json!(7)).unwrap(), "valid");

    let errors = view.validate(&json!("bad")).unwrap_err();
    assert_eq!(errors.issues()[0].instance_path(), "");
    assert_eq!(errors.issues()[0].schema_path(), "/type");
    assert!(schema.pointer(errors.issues()[0].schema_path()).is_none());
}

#[test]
fn validates_known_formats_instead_of_treating_them_as_annotations() {
    for (format, valid, invalid) in [
        ("email", "ada@example.test", "missing-at-sign"),
        ("date-time", "2026-10-07T12:30:00Z", "2026-99-99"),
        ("uuid", "123e4567-e89b-12d3-a456-426614174000", "not-a-uuid"),
    ] {
        let view = View::new(&json!({"type": "string", "format": format}), "valid")
            .expect("compile a schema with a known format");

        view.validate(&json!(valid))
            .expect("valid formatted string");
        assert!(matches!(
            view.render(&json!(invalid)),
            Err(Error::Validation(_))
        ));
    }
}

#[test]
fn resolves_local_definitions_for_valid_and_invalid_input() {
    let view = View::new(
        &json!({
            "$defs": {"name": {"type": "string", "minLength": 1}},
            "type": "object",
            "required": ["name"],
            "properties": {"name": {"$ref": "#/$defs/name"}}
        }),
        "{{ name }}",
    )
    .unwrap();

    assert_eq!(view.render(&json!({"name": "Ada"})).unwrap(), "Ada");
    assert!(matches!(
        view.render(&json!({"name": ""})),
        Err(Error::Validation(_))
    ));
}

#[test]
fn boolean_schemas_accept_or_reject_any_input() {
    let accepts = View::new(&json!(true), "accepted").unwrap();
    let rejects = View::new(&json!(false), "unreachable").unwrap();

    for input in [json!(null), json!({}), json!([1]), json!("text")] {
        assert_eq!(accepts.render(&input).unwrap(), "accepted");
        assert!(matches!(rejects.render(&input), Err(Error::Validation(_))));
    }
}

#[test]
fn rejected_input_does_not_poison_a_reusable_view() {
    let view = View::new(
        &json!({"type": "object", "required": ["name"]}),
        "{{ name }}",
    )
    .unwrap();

    assert!(matches!(view.render(&json!({})), Err(Error::Validation(_))));
    assert_eq!(view.render(&json!({"name": "Ada"})).unwrap(), "Ada");
}

#[test]
fn schema_defaults_do_not_populate_missing_template_values_or_mutate_input() {
    let view = View::new(
        &json!({
            "type": "object",
            "properties": {"name": {"type": "string", "default": "Ada"}}
        }),
        "{{ name }}",
    )
    .unwrap();
    let input = json!({});
    let original = input.clone();

    view.validate(&input)
        .expect("defaulted properties remain optional");
    assert_eq!(input, original);
    assert!(matches!(view.render(&input), Err(Error::Render { .. })));
    assert_eq!(input, original);
}

#[test]
fn validates_internationalized_hostname_format() {
    let view = View::new(
        &json!({"type": "string", "format": "idn-hostname"}),
        "accepted",
    )
    .expect("compile a supported internationalized hostname format");

    assert_eq!(view.render(&json!("münchen.de")).unwrap(), "accepted");
    assert!(matches!(
        view.render(&json!("münchen..de")),
        Err(Error::Validation(_))
    ));
}
