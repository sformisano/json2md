use json2md::{Error, View};
use serde_json::{Value, json};

#[test]
fn release_note_files_and_invalid_title_match_the_readme() {
    let schema: Value =
        serde_json::from_str(include_str!("../../../../examples/assets/note.schema.json")).unwrap();
    let mut input: Value =
        serde_json::from_str(include_str!("../../../../examples/assets/note.json")).unwrap();
    let view = View::new(&schema, include_str!("../../../../examples/assets/note.j2")).unwrap();

    assert_eq!(
        view.render(&input).unwrap(),
        include_str!("../../../../examples/assets/note.md")
    );

    input["title"] = json!(42);
    let Error::Validation(errors) = view.render(&input).unwrap_err() else {
        panic!("the invalid title must fail schema validation");
    };
    assert_eq!(errors.issues().len(), 1);
    let issue = &errors.issues()[0];
    let diagnostic = format!("{}: {}", issue.instance_path(), issue);
    assert_eq!(diagnostic, "/title: 42 is not of type \"string\"");
    assert!(include_str!("../../../../README.md").contains(&format!("```text\n{diagnostic}\n```")));
}
