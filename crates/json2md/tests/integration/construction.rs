use std::error::Error as StdError;

use json2md::{Error, View};
use serde_json::json;

#[test]
fn invalid_schema_is_a_schema_error_with_a_source() {
    let error = match View::new(&json!({"type": 42}), "valid template") {
        Err(error) => error,
        Ok(_) => panic!("invalid schema must fail compilation"),
    };

    assert!(matches!(error, Error::Schema { .. }));
    assert!(StdError::source(&error).is_some());
}

#[test]
fn invalid_template_is_a_template_error_with_a_source() {
    let error = match View::new(&json!(true), "{% if %}") {
        Err(error) => error,
        Ok(_) => panic!("invalid template must fail compilation"),
    };

    assert!(matches!(error, Error::Template { .. }));
    assert!(StdError::source(&error).is_some());
}

#[test]
fn schema_compilation_failure_precedes_template_compilation() {
    assert!(matches!(
        View::new(&json!({"type": 42}), "{% if %}"),
        Err(Error::Schema { .. })
    ));
}

#[test]
fn render_failure_keeps_its_underlying_source() {
    let view = View::new(&json!(true), "{{ absent }}").unwrap();
    let error = view.render(&json!({})).unwrap_err();

    assert!(matches!(error, Error::Render { .. }));
    assert!(StdError::source(&error).is_some());
}

#[test]
fn unknown_formats_are_rejected_during_schema_compilation() {
    assert!(matches!(
        View::new(
            &json!({"type": "string", "format": "invented-format"}),
            "text"
        ),
        Err(Error::Schema { .. })
    ));
}

#[test]
fn external_schema_references_are_rejected() {
    for reference in [
        "https://example.invalid/schema.json",
        "http://127.0.0.1:1/schema.json",
        "relative-schema.json",
        "urn:example:external-schema",
    ] {
        assert!(
            matches!(
                View::new(&json!({"$ref": reference}), "text"),
                Err(Error::Schema { .. })
            ),
            "external reference {reference} must not be retrievable"
        );
    }
}

#[test]
fn file_reference_cannot_load_an_existing_local_schema() {
    let reference = format!(
        "file://{}/tests/fixtures/review.schema.json",
        env!("CARGO_MANIFEST_DIR")
    );

    assert!(matches!(
        View::new(&json!({"$ref": reference}), "text"),
        Err(Error::Schema { .. })
    ));
}

#[test]
fn detects_the_schema_draft_and_defaults_to_2020_12() {
    let explicit = View::new(
        &json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "integer",
            "minimum": 1
        }),
        "accepted",
    )
    .unwrap();
    let default = View::new(
        &json!({
            "type": "array",
            "prefixItems": [{"type": "integer"}],
            "items": false
        }),
        "accepted",
    )
    .unwrap();

    assert_eq!(explicit.render(&json!(1)).unwrap(), "accepted");
    assert!(matches!(
        explicit.render(&json!(0)),
        Err(Error::Validation(_))
    ));
    assert_eq!(default.render(&json!([1])).unwrap(), "accepted");
    assert!(matches!(
        default.render(&json!([1, 2])),
        Err(Error::Validation(_))
    ));
}

#[test]
fn detects_draft_4_boolean_exclusive_minimum() {
    let view = View::new(
        &json!({
            "$schema": "http://json-schema.org/draft-04/schema#",
            "type": "number",
            "minimum": 1,
            "exclusiveMinimum": true
        }),
        "accepted",
    )
    .unwrap();

    assert_eq!(view.render(&json!(2)).unwrap(), "accepted");
    assert!(matches!(view.render(&json!(1)), Err(Error::Validation(_))));
}

#[test]
fn detects_draft_6_and_ignores_reference_siblings() {
    let view = View::new(
        &json!({
            "$schema": "http://json-schema.org/draft-06/schema#",
            "$ref": "#/definitions/base",
            "type": "string",
            "definitions": {"base": {"type": "integer"}}
        }),
        "accepted",
    )
    .unwrap();

    assert_eq!(view.render(&json!(1)).unwrap(), "accepted");
    assert!(matches!(
        view.render(&json!("one")),
        Err(Error::Validation(_))
    ));
}

#[test]
fn detects_draft_2019_09_unevaluated_properties() {
    let view = View::new(
        &json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "type": "object",
            "properties": {"name": {"type": "string"}},
            "unevaluatedProperties": false
        }),
        "{{ name }}",
    )
    .unwrap();

    assert_eq!(view.render(&json!({"name": "Ada"})).unwrap(), "Ada");
    assert!(matches!(
        view.render(&json!({"name": "Ada", "extra": true})),
        Err(Error::Validation(_))
    ));
}

#[test]
fn invalid_or_custom_schema_declarations_are_schema_errors() {
    for declaration in [
        json!(42),
        json!("not a schema URI"),
        json!("https://example.invalid/custom-meta-schema"),
    ] {
        assert!(matches!(
            View::new(&json!({"$schema": declaration, "type": "string"}), "text"),
            Err(Error::Schema { .. })
        ));
    }
}

#[test]
fn unresolved_local_reference_is_a_schema_error() {
    assert!(matches!(
        View::new(&json!({"$ref": "#/$defs/missing"}), "text"),
        Err(Error::Schema { .. })
    ));
}

#[test]
fn unused_unknown_formats_are_rejected_when_referenced() {
    let mut schema = json!({"$defs": {"unused": {"format": "invented-format"}}});
    let view = View::new(&schema, "accepted").expect("unused definitions are not compiled");
    assert_eq!(view.render(&json!("text")).unwrap(), "accepted");

    schema["$ref"] = json!("#/$defs/unused");
    assert!(matches!(
        View::new(&schema, "text"),
        Err(Error::Schema { .. })
    ));
}

#[test]
fn includes_cannot_load_an_existing_template_from_the_filesystem() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/static.j2");
    let contents = std::fs::read_to_string(&path).expect("read the existing template fixture");
    let direct = View::new(&json!(true), &contents).expect("compile the template directly");
    assert_eq!(direct.render(&json!({})).unwrap(), contents);
    let include = format!(
        "{{% include {:?} %}}",
        path.to_str().expect("UTF-8 fixture path")
    );

    match View::new(&json!(true), &include) {
        Err(error) => assert!(matches!(error, Error::Template { .. })),
        Ok(view) => assert!(matches!(view.render(&json!({})), Err(Error::Render { .. }))),
    }
}
