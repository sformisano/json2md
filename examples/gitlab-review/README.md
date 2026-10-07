# GitLab review comments

This example renders finding comments and cumulative review summaries from JSON.
Each comment type has its own JSON Schema and MiniJinja template.
Merge request URLs, reviewer IDs, commit IDs, and discussion links are fictional fixture data.

| Pair | Input | Expected Markdown |
| --- | --- | --- |
| [Finding schema](finding.schema.json) and [template](finding.j2) | [Finding](finding.json) | [Finding comment](finding.md) |
| [Summary schema](summary.schema.json) and [template](summary.j2) | [Initial review](initial.json) | [Initial summary](initial.md) |
| Same summary pair | [Focused follow-up](follow-up.json) | [Summary with history](follow-up.md) |
| Same summary pair | [Pending review](pending.json) | [Pending summary](pending.md) |
| Same summary pair | [Review without findings](no-findings.json) | [Empty-round summary](no-findings.md) |

## Render a summary

The [Rust example](../gitlab_review.rs) reads the committed schema, template, and
follow-up data into the ordinary `View` API. It prints the rendered comment body.
Run it from the repository root:

```sh
cargo run --locked --quiet -p json2md --example gitlab_review
```

The CLI accepts the same assets. Render a finding or summary from the repository root:

```sh
cargo run --locked --quiet -p json2md-cli -- render examples/gitlab-review/finding.json \
  --schema examples/gitlab-review/finding.schema.json --template examples/gitlab-review/finding.j2
cargo run --locked --quiet -p json2md-cli -- render examples/gitlab-review/follow-up.json \
  --schema examples/gitlab-review/summary.schema.json --template examples/gitlab-review/summary.j2
```

The latest round appears first. Earlier rounds occupy separate collapsed blocks.
The final metadata block separates review scope from depth and retains cumulative policy counts.
Finding IDs and links retain the supplied identities across rounds.
An empty completed round has no findings table or extra outcome paragraph.

Both templates use `tojson` for JSON discovery markers.
The finding template preserves its supplied Markdown body and includes disclosure only when supplied.
The summary template escapes HTML attributes in historical commit links.
Other Markdown escaping remains the caller's responsibility under the [library contract](../../docs/architecture.md#template-behavior).

## Caller responsibilities

The schemas validate these render-ready records, including required fields, types, and supported labels.
They are example presentation schemas; they do not implement the complete review assessment model.
The caller supplies the latest round first, older rounds newest first, and findings sorted by numeric ID.
It derives scope, depth, verdicts, policy selection, counts, and verified links from review evidence.
The renderer preserves supplied history; it does not reconcile provider state or count discussions.

The caller serializes rendered text into the GitLab request's `body` field.
It verifies permission, ownership, current revisions, duplicate prevention, and publication results.
Repeated rendering is deterministic. The caller compares existing comments before deciding whether to publish.
Offline validation does not establish publication authority or prove a live unchanged retry.

## Tests

```sh
cargo test --locked -p json2md --test integration gitlab
cargo test --locked -p json2md-cli --test integration gitlab
```

The tests exercise the real files through `View::render` and compare every output byte with the committed Markdown files.
They parse discovery-marker JSON, reject incomplete input, preserve Markdown and explicit disclosure,
and round-trip rendered bodies through JSON request serialization.
History checks require balanced sibling blocks. Reuse checks cover unchanged input after other review states.
The CLI tests compare the finding and every summary with their expected Markdown files,
through both standard output and file output. These are offline integration tests;
they do not create or update GitLab comments.
