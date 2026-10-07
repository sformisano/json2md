# Publishing

The workspace publishes two crates to crates.io: the `json2md` library and the
`json2md-cli` package, which installs the `json2md` binary.
Both inherit their version, Rust requirement, repository URL, and license from
the root `Cargo.toml`. The CLI's library dependency also declares a registry version.
Update that dependency when changing the workspace version.

## Verify the release

Run the [development checks](../README.md#development) from a clean checkout.
Use Rust 1.98 or later, including Cargo 1.98 for workspace publishing.

```sh
cargo package --locked --workspace
cargo publish --locked --workspace --dry-run
```

Cargo packages and builds both crates, including the CLI's dependency on the
packaged library. The archives are written to `target/package`.
Each package contains its source, README, license texts, and lockfile.
Integration tests and shared example assets remain in the repository.

Before uploading, verify the crate names and account permissions on crates.io.
Check that the repository and documentation links in both READMEs are publicly
accessible. A dry run does not upload packages or confirm registry authorization.

## Publish and check consumers

Authenticate with crates.io using Cargo's credential support, then publish from
the verified, committed checkout:

```sh
cargo publish --locked --workspace
```

Cargo publishes dependencies before their dependents. If publication stops,
check which versions reached crates.io before retrying.
A published version cannot be replaced; corrections require a new version.
Source changes after verification require the development and packaging checks again.

From outside the checkout, install the published CLI and compare its output with
the committed release-note Markdown:

```sh
cargo install --locked json2md-cli --version 0.1.0
json2md render note.json --schema note.schema.json --template note.j2
```

Use the files linked in the [README example](../README.md#from-json-to-markdown).
Also compile and run the README's Rust example in a fresh project using
`json2md = "0.1"`, and check that the [API documentation](https://docs.rs/json2md)
builds successfully. These registry checks confirm delivery after publication.
