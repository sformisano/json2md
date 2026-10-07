# json2md

`json2md` is a Rust library that validates JSON and renders it as Markdown.
Keep a document's content separate from its layout, and check its fields and
values before producing the document.

## From JSON to Markdown

This example creates a release note with a title, summary, and list of changes.

### 1. Start with the data

The [JSON input](https://github.com/sformisano/json2md/blob/main/examples/assets/note.json) holds the note's content:

```json
{
  "title": "Release 0.1",
  "summary": "JSON data becomes a checked Markdown document.",
  "changes": [
    "Validate input before rendering.",
    "Reuse compiled schemas and templates."
  ]
}
```

These values describe the content. The headings and list formatting belong in
the template introduced below.

### 2. Define the accepted data

JSON Schema describes which fields and values are accepted.
The [note's schema](https://github.com/sformisano/json2md/blob/main/examples/assets/note.schema.json) requires a title and summary
as strings, and changes as an array of strings:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "title": { "type": "string" },
    "summary": { "type": "string" },
    "changes": { "type": "array", "items": { "type": "string" } }
  },
  "required": ["title", "summary", "changes"],
  "additionalProperties": false
}
```

Declaring a property in `properties` describes its value. Listing it in
`required` makes its presence mandatory. `additionalProperties: false` rejects
fields that the schema does not declare.

### 3. Describe the presentation

A [template](https://github.com/sformisano/json2md/blob/main/examples/assets/note.j2) combines fixed Markdown with values from the JSON input.
Templates use MiniJinja syntax: `{{ ... }}` inserts a value, and `{% ... %}`
controls a loop or condition. JSON object properties become template variables.

```jinja
# {{ title }}

{{ summary }}

## Changes
{% for change in changes %}
- {{ change }}{% endfor %}
```

## Use the Rust library

The library requires Rust 1.98 or later. Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
json2md = "0.1"
serde_json = "1.0"
```

See the [API reference](https://docs.rs/json2md) for public types and methods.

### Compile the view and render

A `View` compiles one schema and one template. Its `render` method validates
the input before executing the template, then returns the complete Markdown string.

The complete [runnable example](https://github.com/sformisano/json2md/blob/main/examples/render.rs) embeds the release-note files:

```rust
use json2md::View;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema: Value = serde_json::from_str(include_str!("assets/note.schema.json"))?;
    let input: Value = serde_json::from_str(include_str!("assets/note.json"))?;
    let view = View::new(&schema, include_str!("assets/note.j2"))?;

    print!("{}", view.render(&input)?);
    Ok(())
}
```

Run it from the repository root:

```sh
cargo run --locked -p json2md --example render
```

Standard output matches the [expected Markdown](https://github.com/sformisano/json2md/blob/main/examples/assets/note.md), including its final newline:

```markdown
# Release 0.1

JSON data becomes a checked Markdown document.

## Changes

- Validate input before rendering.
- Reuse compiled schemas and templates.
```

### Reuse the view

Keep the `View` to render more notes without compiling the schema and template again.
Each input is still validated. The view owns its compiled assets, so the original
schema and template can be dropped.

### Handle invalid input

Input that fails the schema returns `Error::Validation` with field paths and diagnostics.
Using the same `view`, replace the title with a number:

```rust
use json2md::Error;
use serde_json::json;

let invalid = json!({
    "title": 42,
    "summary": "JSON data becomes a checked Markdown document.",
    "changes": []
});

match view.render(&invalid) {
    Ok(markdown) => println!("{markdown}"),
    Err(Error::Validation(errors)) => {
        for issue in errors.issues() {
            eprintln!("{}: {}", issue.instance_path(), issue);
        }
    }
    Err(error) => eprintln!("{error}"),
}
```

The diagnostic for this input is:

```text
/title: 42 is not of type "string"
```

The `/title` path identifies the rejected property. No template execution occurs
for this input. Match typed error variants to handle failures; diagnostic wording
and issue ordering can change with dependencies.

Use `view.validate(&input)` when you only need validation. Rendering always
validates; there is no unchecked rendering method.

### Handle optional fields

The note currently requires `summary`. Remove it from the schema's `required`
array to make it optional.
The original template's `{{ summary }}` then fails with `Error::Render` when that
property is absent. Guard the optional section explicitly:

```jinja
{% if summary is defined %}
{{ summary }}
{% endif %}
```

Some MiniJinja filters and tests handle missing values without an error.
Keep a property required in the schema when every document must supply it.
A present JSON `null` differs from an absent property; this note's string schema
rejects `null` for `summary`.

`View::new` checks the schema and template separately. It does not prove that
the template can render every input accepted by the schema.

## Rendering behavior

The renderer preserves Markdown and HTML characters without automatic escaping.
For example, a JSON string containing `**bold**` produces `**bold**` in the output.
Choose how to escape or sanitize values for your Markdown renderer.
The final template newline is preserved.

Library callers load files and write output. Schema references within the supplied
document are supported; external schema retrieval and template file loading are disabled.
Known schema formats are validated. Compile-time schema-template compatibility
checking is outside v0.1. See the [architecture contract](https://github.com/sformisano/json2md/blob/main/docs/architecture.md)
for schema drafts, numeric limits, errors, and trust boundaries.

## Render files with the CLI

The command-line tool uses the same library to render files.
Install the `json2md-cli` package from crates.io. Its binary is named `json2md`:

```sh
cargo install --locked json2md-cli
```

Supply your JSON, schema, and template files. The schema and template are required:

```sh
json2md render input.json --schema schema.json --template view.j2
```

Markdown goes to standard output. Use `--output PATH` to write a file instead:

```sh
json2md render input.json --schema schema.json --template view.j2 --output document.md
```

Run the same release-note example from the repository root without installing:

```sh
cargo run --locked -p json2md-cli -- render examples/assets/note.json \
  --schema examples/assets/note.schema.json --template examples/assets/note.j2
```

Every render validates its input. Failures produce diagnostics on standard error
and a nonzero exit status. Validation or rendering failures produce no Markdown.
An unsuccessful operation preserves an existing output file.
Output cannot replace an input, schema, or template file.
See the [CLI contract](https://github.com/sformisano/json2md/blob/main/docs/architecture.md#cli-contract) for output and diagnostic details.

## Development

Repository commands run from the workspace root. To install the CLI from source,
use `cargo install --locked --path crates/json2md-cli`.

The library integration tests compare real template files against independently written
Markdown files. They also exercise invalid schemas, invalid data, strict missing
values, schema references, formats, and view reuse.
The CLI integration tests invoke the binary with real files. They check exact
output, diagnostics, failure status, and preservation of inputs and existing output.

```sh
cargo fmt --all -- --check
cargo test --locked --workspace --all-targets
cargo test --locked --workspace --doc
cargo clippy --locked --workspace --all-targets -- -D warnings
```

Generate API documentation with `cargo doc --locked --workspace --no-deps`.
The [CI workflow](https://github.com/sformisano/json2md/blob/main/.github/workflows/ci.yml) also compares library examples and CLI
output with committed Markdown files.
The [GitLab fixtures](https://github.com/sformisano/json2md/blob/main/examples/gitlab-review/README.md) also exercise finding and
summary bodies as offline regression tests. They are excluded from release packages.
See [agent skills](https://github.com/sformisano/json2md/blob/main/docs/agent-skills.md) for repository authoring guidance and
[README guidance](https://github.com/sformisano/json2md/blob/main/docs/readme-guidance.md) for documentation examples.
See [publishing](https://github.com/sformisano/json2md/blob/main/docs/publishing.md) for package verification and release commands.

## License

Licensed under MIT OR Apache-2.0, at your choice. See [LICENSE](https://github.com/sformisano/json2md/blob/main/LICENSE) for both texts.
