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


## Install a prebuilt binary

[GitHub Releases](https://github.com/sformisano/json2md/releases/latest) provides archives named
`json2md-v<VERSION>-<TARGET>.tar.gz`, or `.zip` for Windows.
Each archive contains `json2md` (`json2md.exe` on Windows), the CLI README, and `LICENSE`.
Rust is not required to run these binaries.

| Platform | Target | Archive |
| --- | --- | --- |
| Linux x86_64 | `x86_64-unknown-linux-musl` | `.tar.gz` |
| Linux ARM64 | `aarch64-unknown-linux-musl` | `.tar.gz` |
| macOS Intel | `x86_64-apple-darwin` | `.tar.gz` |
| macOS Apple Silicon | `aarch64-apple-darwin` | `.tar.gz` |
| Windows x86_64 | `x86_64-pc-windows-msvc` | `.zip` |

Linux binaries use musl and do not require glibc. macOS binaries require macOS 15 or later.
Windows binaries statically link the C runtime; no Visual C++ runtime installation is required.
They are built and exercised on Windows Server 2022.
The macOS and Windows binaries have no publisher signature or notarization.
Each archive has a matching `.sha256` file for verifying download integrity.

Download the archive and its checksum file for your platform.
On Linux, verify and extract the x86_64 archive with:

```sh
sha256sum --check json2md-v0.1.0-x86_64-unknown-linux-musl.tar.gz.sha256
tar -xzf json2md-v0.1.0-x86_64-unknown-linux-musl.tar.gz
```

On macOS, use `shasum -a 256 -c` with the downloaded checksum filename.
On Windows, use PowerShell's `Get-FileHash` and compare its SHA256 value with the checksum file:

```powershell
Get-FileHash .\json2md-v0.1.0-x86_64-pc-windows-msvc.zip -Algorithm SHA256
Expand-Archive .\json2md-v0.1.0-x86_64-pc-windows-msvc.zip -DestinationPath .\json2md
```

Place the extracted binary in a directory on your `PATH`.
Check `json2md --version`, then run the [CLI example](../README.md#render-files-with-the-cli).

## Publish GitHub binaries

The [release workflow](../.github/workflows/release.yml) builds binaries from an existing stable tag, such as `v0.1.0`.
It requires the tag to match the workspace version and resolves it to one commit before starting builds.
It uses Rust 1.98.1 and the committed lockfile.
A `v*` tag push starts the workflow automatically. Tag names must use `vMAJOR.MINOR.PATCH`.

Publish the crates and check their consumers before creating a new release tag.
Create the tag at the verified release commit:

```sh
git tag v0.1.0
git push origin v0.1.0
```

For an existing tag, start the workflow from `main` without changing the tag:

```sh
gh workflow run release.yml --repo sformisano/json2md --ref main -f tag=v0.1.0
gh run list --repo sformisano/json2md --workflow release.yml
```

Each platform extracts its archive and runs the packaged binary.
It checks the version, mandatory schema, failed validation, preserved output, and byte-for-byte output from the committed examples.
The examples include the release note, GitLab finding, and every GitLab summary.
Builds run on Ubuntu 24.04, macOS 15, and Windows Server 2022 with native architectures.

Only successful builds reach publication.
The workflow creates a draft release, uploads the archives and checksums, then downloads and verifies them before publication.
It records the source commit in the release notes.
It does not overwrite an existing release or asset.

If publication fails, inspect the workflow logs and any draft release before retrying.
A retry with an existing draft stops at release creation.
Delete an incomplete unpublished draft before rerunning, or finish its publication using the verified assets.
Keep published tags and assets unchanged. Corrections to consumed binaries require a new version.

After publication, download the assets from the public release page.
Verify the checksums and run the extracted native binary against the committed example files.
This checks the same download path used by consumers.
