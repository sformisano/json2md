//! Exercises public rendering and validation contracts with real template fixtures.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "Unexpected fixture and operation failures must fail the test immediately."
)]

#[path = "integration/render.rs"]
mod render;

#[path = "integration/validation.rs"]
mod validation;

#[path = "integration/construction.rs"]
mod construction;

#[path = "integration/readme.rs"]
mod readme;

#[path = "integration/numbers.rs"]
mod numbers;

#[path = "integration/gitlab.rs"]
mod gitlab;
