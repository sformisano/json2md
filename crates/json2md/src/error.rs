use std::error::Error as StdError;

use crate::ValidationErrors;

/// The stage that prevents a view from being constructed or rendered.
///
/// Dependency diagnostics are available through [`StdError::source`]. Their
/// types and wording are outside this crate's compatibility contract. Diagnostic
/// details can contain input values or template text; callers control disclosure.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The schema is invalid or requires an unavailable external resource.
    #[error("could not compile JSON Schema")]
    Schema {
        /// The validator's diagnostic, including reference-resolution failures.
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
    /// The template cannot be compiled.
    #[error("could not compile Markdown template")]
    Template {
        /// The template engine's syntax diagnostic.
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
    /// The input fails validation; the template does not run.
    #[error("JSON does not match the view's schema")]
    Validation(#[from] ValidationErrors),
    /// Schema-valid input cannot be converted or fails during template execution.
    #[error("could not render Markdown template")]
    Render {
        /// The numeric conversion or template execution diagnostic.
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
}
