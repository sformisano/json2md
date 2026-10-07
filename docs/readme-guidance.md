# README authoring guidance

Use these Rust library READMEs as examples of clear explanations:

- [Redactable README](https://github.com/sformisano/redactable/blob/main/README.md): explains the available approaches, shows their outputs, and helps readers choose between them.
- [Type History README](https://github.com/sformisano/type-history/blob/main/README.md): introduces a concrete problem and follows one example through increasingly detailed steps.

Read the current versions before substantial README edits.
Adapt their explanations and progression to `json2md`. Do not copy their prose, APIs, release details, or document length.

## What the README should teach

1. State what the library does and the problem it solves in plain language.
2. Show the shortest complete path from JSON data to Markdown. Include the schema, template, Rust call, and expected Markdown once the API exists.
3. Explain each step beside its example. Define an unfamiliar term before relying on it.
4. Reuse one concrete example as details are introduced, such as missing fields, invalid input, and template reuse.
5. Show an invalid input and its actual diagnostic, as well as the successful output.
6. Explain supported behavior, limits, and what the caller must do. Help readers choose between API options when there is a real choice.
7. Link to detailed guides and API reference from the relevant explanation. Add navigation when the README's size requires it.

Use examples that apply to ordinary library callers. Review-specific use belongs in an additional example when supported.
Keep installation commands, dependencies, platform support, and Rust version requirements accurate for the documented revision.
Check runnable snippets and expected outputs against the implementation.
Label conceptual or deliberately failing examples explicitly.

## Planning-stage accuracy

Until implementation exists, keep the README explicit about the project's planning status.
Link to the [architecture proposal](architecture.md) for proposed behavior and open decisions.
Do not present proposed APIs, crate installation commands, release badges, or planned guarantees as available behavior.
