# json2md CLI

`json2md` validates JSON with a JSON Schema and renders it as Markdown using a
MiniJinja template. The CLI calls the [json2md Rust library](https://docs.rs/json2md).

Download a prebuilt binary from [GitHub Releases](https://github.com/sformisano/json2md/releases/latest).
See [binary installation](https://github.com/sformisano/json2md/blob/main/docs/publishing.md#install-a-prebuilt-binary) for platforms and checksum verification.

Or install this package from crates.io with Rust 1.98 or later:

```sh
cargo install --locked json2md-cli
```

The installed binary is named `json2md`:

```sh
json2md render input.json --schema schema.json --template view.j2
json2md render input.json --schema schema.json --template view.j2 --output document.md
```

The schema and template are required. Every render validates the input before
executing the template. Markdown goes to standard output unless `--output` is supplied.
Diagnostics go to standard error. Failures return a nonzero exit status.
Validation and rendering failures produce no Markdown. A failed operation preserves
an existing output file. Input files are not modified.

See the [project README](https://github.com/sformisano/json2md#readme) for a complete
data, schema, and template example, and the
[CLI contract](https://github.com/sformisano/json2md/blob/main/docs/architecture.md#cli-contract)
for output details.

Licensed under MIT OR Apache-2.0, at your choice. Both texts are included in `LICENSE`.
