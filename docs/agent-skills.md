# Agent skills

The repository declares skills for its synchronous Rust library and CLI in [.skillcatalog/skillcatalog.yml](../.skillcatalog/skillcatalog.yml).
The project profile ID is `json2md`.

## Installation

Run these commands from the checkout where the agent works:

```sh
skc install
skc validate --manifest
skc validate --drift
```

Installation requires SkillCatalog and access to the configured `ai-skills` catalog.
Enable the agent's delivery target before installation.
For Codex, use `skc settings enable codex`.
For Claude Code, use `skc settings enable claude-code`.

Codex receives skills under `.agents/skills/`.
Claude Code receives skills under `.claude/skills/`.
Generated skill files are ignored by Git. Do not edit them by hand.

SkillCatalog binds each project profile ID to one checkout on a machine.
For a concurrent checkout, create a distinct personal profile with `skc profile create`, using that checkout's directory and the same manifest entries.

Before changing source, read the installed skills that apply to the change.
Resolve installation or delivery failures before coding.
Skill installation does not change the library's [scope](architecture.md).

## Rust guidance

The selected skills cover:

- Small, focused files with clear responsibility boundaries.
- Typed errors, preserved causes, and diagnostics with one reporting owner.
- Enums and semantic types for controlled alternatives and distinct identities.
- Validated construction, ownership, borrowing, and deliberate abstractions.
- Repository documentation and claims checked against current behavior.
- Tests of behavior and contracts, reusable harnesses, and verification of completion claims.

The manifest also includes the selected skills' supporting policies.
Skills guide agent decisions; implementation checks must verify the approved contracts.
