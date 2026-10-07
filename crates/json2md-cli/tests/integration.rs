//! Exercises the binary with file-backed inputs and byte-exact Markdown output.

#![allow(
    clippy::expect_used,
    reason = "Unexpected fixture and child-process failures must fail the test immediately."
)]

#[path = "integration/support.rs"]
mod support;

#[path = "integration/render.rs"]
mod render;

#[path = "integration/failures.rs"]
mod failures;

#[path = "integration/examples.rs"]
mod examples;
