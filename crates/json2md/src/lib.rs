//! Validate JSON with JSON Schema and render it as Markdown.
//!
//! A [`View`] compiles one schema and one template for repeated use. Every
//! [`View::render`] validates its input before running the template.
//!
//! ```
//! use json2md::View;
//! use serde_json::json;
//!
//! let schema = json!({
//!     "type": "object",
//!     "properties": {"title": {"type": "string"}},
//!     "required": ["title"]
//! });
//! let view = View::new(&schema, "# {{ title }}\n")?;
//! assert_eq!(view.render(&json!({"title": "Release notes"}))?, "# Release notes\n");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Templates use MiniJinja syntax. Missing values are strict and output has no
//! automatic escaping. Callers own file I/O, Markdown escaping, and the trust
//! policy for their schemas, templates, input, and output.

mod context;
mod error;
mod schema;
mod validation;
mod view;

pub use error::Error;
pub use validation::{ValidationErrors, ValidationIssue};
pub use view::View;
