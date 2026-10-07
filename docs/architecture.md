# Architecture

`json2md` v0.1 is a synchronous Rust library for validating JSON and rendering
Markdown. JSON is the source of truth. Consuming projects supply each schema,
template, and their pairing.

The Cargo workspace contains two crates:

| Crate | Responsibility |
| --- | --- |
| `crates/json2md` | The `json2md` library: compile assets, validate input, and render Markdown |
| `crates/json2md-cli` | The `json2md` binary: parse arguments, read files, report diagnostics, and write output |

The CLI calls the library directly. Schemas, templates, and data remain caller-owned
assets. Neither crate performs reviews, contacts providers, or publishes comments.

## Runtime contract

`View::new(&schema, template)` compiles one JSON Schema and one MiniJinja template.
The view owns its compiled assets and can be reused across inputs.
The schema is compiled first. A construction failure returns a typed error.

`View::render(&input)` validates the input before executing the template.
Invalid input returns `Error::Validation`; the template does not run.
Successful rendering returns a complete Markdown `String`.
A rendering failure returns `Error::Render` without exposing partial output.
`View::validate(&input)` checks data without executing the template.

Input is not mutated or normalized. Schema defaults do not fill missing values.
Object properties become top-level template variables. To iterate a root array or
display a root scalar, supply an object wrapper, such as `{"items": [1, 2]}`, with
a matching schema and reference `items` in the template. There is no named
variable for the entire input document.

Rendering converts JSON values directly into MiniJinja values. Integer
representations are exact within the renderer's signed or unsigned 128-bit range.
Decimal and exponent representations use finite `f64` and can lose precision. Numbers outside these
ranges produce `Error::Render`, including when another dependency enables
Serde JSON's arbitrary-precision representation. Validation alone does not
apply the renderer's numeric limits.

The caller owns file loading, output writes, domain rules, and provider operations.
The library has no filesystem loader, application configuration, or plugin registry.
The CLI owns file access and command-line behavior.

## CLI contract

```sh
json2md render input.json --schema schema.json --template view.j2 [--output PATH]
```

The CLI reads JSON, a JSON Schema, and a UTF-8 MiniJinja template from files.
It constructs a `View` and calls `View::render`; validation runs on every render.
The CLI has no unchecked mode or separate validation step.

The complete rendered string is written to standard output by default.
`--output PATH` writes that string to a file and leaves standard output empty.
Output bytes are unchanged, including Unicode, quotes, and trailing newlines.
Input files are read without modification. An output path resolving to an input,
schema, or template file is rejected.

All read, parse, construction, validation, rendering, and write failures return
a nonzero exit status. Diagnostics go to standard error and identify the relevant
files and underlying causes. Validation diagnostics include every reported input
JSON Pointer and schema resource pointer. Empty pointers identify the document root.

Validation and rendering finish before output writes begin, so either failure
produces no partial Markdown. File output is staged in the destination directory
and atomically replaces the destination only after the complete write succeeds.
Failures preserve an existing output file. The destination directory must exist.
Replacement uses the temporary file's permissions and replaces a destination symlink itself.
This does not promise crash durability. A standard-output I/O failure can leave
bytes already accepted by the receiving stream.

## Schema behavior

The `jsonschema` dependency detects draft 4, 6, 7, 2019-09, or 2020-12 from `$schema`.
Schemas without `$schema` use draft 2020-12.
Known `format` keywords are asserted. Unknown formats in compiled validation
constraints fail view construction. An unused definition is not compiled;
an unknown format inside it is rejected when a reference makes it reachable.

References inside the supplied schema document are supported, including `$defs`.
An explicit retriever rejects external resources. This remains true if a caller
enables additional `jsonschema` dependency features.
The library does not fetch remote schemas or read schema files.

## Template behavior

MiniJinja compiles and renders the template. Built-in filters and tests are available.
The `tojson` filter serializes values as JSON, including structured metadata inside Markdown comments.
Undefined values use strict behavior: accessing an absent variable can fail rendering.
Optional values need a guard, such as `{% if summary is defined %}`.
A JSON `null` value is present and differs from a missing variable.

Output has no automatic HTML or Markdown escaping. The final template newline is
preserved. Callers choose escaping and sanitization for their output destination.

Each view contains one template. No template loader is registered, and no additional
templates are supplied for includes, imports, or inheritance.
Cargo feature selection is not a security boundary: consuming crates can unify
dependency features. The public contract does not promise a restricted template language.

## Errors and diagnostics

The non-exhaustive `Error` enum distinguishes failures by stage:

| Error | When it occurs |
| --- | --- |
| `Schema` | Schema compilation or reference resolution fails |
| `Template` | Template compilation fails |
| `Validation` | Input fails the compiled schema |
| `Render` | Valid input cannot be converted or fails during template execution |

`ValidationErrors` contains a nonempty collection of owned `ValidationIssue` values.
Issues expose JSON Pointer paths into the input and the schema resource.
An embedded `$id` starts a new resource; a schema pointer can therefore be relative
to that resource instead of the outer schema document supplied to the view.
An empty pointer identifies the document root. A missing required property points
to its containing object.

Errors retain their underlying diagnostic through `std::error::Error::source`.
Dependency error types, wording, and issue order are outside the compatibility contract.
Diagnostic details can contain input values or template text.
Callers control how much diagnostic content they expose.

## Trust and execution

Schemas and templates are caller-controlled application assets.
The library does not provide execution isolation or configurable resource budgets
for hostile schemas, templates, or unbounded input.
Callers that accept such assets must enforce their own trust and size limits.

Schema validity is distinct from schema-template compatibility.
Construction can succeed for a schema that permits `{}` and a template that accesses `title`.
Rendering `{}` then fails because strict template handling detects the missing value.
Tests with representative input prove those cases, not every value the schema accepts.

## Implementation layout

Library modules live in `crates/json2md/src`. Its integration tests live in
`crates/json2md/tests`. Shared example assets remain under `examples` at the
repository root. CLI source and binary integration tests live in `crates/json2md-cli`.

| Module | Responsibility |
| --- | --- |
| `lib.rs` | Public exports and a minimal API example |
| `view.rs` | Compile the asset pair, validate, and render |
| `context.rs` | Convert JSON into template values with checked numeric ranges |
| `schema.rs` | Validator configuration and external retrieval rejection |
| `validation.rs` | Owned validation issues and their paths |
| `error.rs` | Typed stage errors and source chains |

The library calls `jsonschema` and MiniJinja through its own interface.
It has no custom template parser, schema interpreter, or rendering subprocess.
The lockfile records the dependency versions used for repository verification.
Release packages contain source, package documentation, and license texts.
Shared examples and integration tests remain repository assets; see
[publishing](publishing.md) for package verification and release commands.

## Deferred compatibility checking

Compile-time schema-template compatibility checking is outside v0.1.
There is no build hook, procedural macro, or static template analyzer.
Runtime validation remains necessary regardless of any future build integration.

The json2md maintainers revisit this feature after the runtime core is proven.
A separate proposal must define the supported schema and template constructs,
the guarantees for optional values and loops, and how unsupported analysis fails.
It must also define how checked assets remain identical to the assets used at runtime.
