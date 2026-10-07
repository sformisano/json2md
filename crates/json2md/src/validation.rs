use std::{error::Error as StdError, fmt};

use jsonschema::{ValidationError, Validator};
use serde_json::Value;

/// A validation failure with paths into the input and its schema resource.
///
/// Paths use JSON Pointer syntax. An empty path refers to the document root.
/// Its display diagnostic can include the rejected value.
#[derive(Debug)]
pub struct ValidationIssue {
    error: ValidationError<'static>,
}

impl ValidationIssue {
    /// The input location that fails validation, as a JSON Pointer.
    ///
    /// A missing required property points to its containing object.
    #[must_use]
    pub fn instance_path(&self) -> &str {
        self.error.instance_path().as_str()
    }

    /// The schema keyword location within its schema resource, as a JSON Pointer.
    ///
    /// An embedded `$id` starts a new resource, so this pointer need not resolve
    /// against the outer schema document supplied to the view.
    #[must_use]
    pub fn schema_path(&self) -> &str {
        self.error.schema_path().as_str()
    }
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

/// A nonempty collection of owned validation failures.
///
/// Failures remain available after the input or view is dropped. [`Self::issues`]
/// exposes every reported failure; the error source contains the first one.
/// Issue ordering and diagnostic wording are not stable contracts.
#[derive(Debug)]
pub struct ValidationErrors {
    issues: Vec<ValidationIssue>,
}

impl ValidationErrors {
    /// The failures reported by the validator, including their input and schema paths.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    pub(crate) fn check(validator: &Validator, input: &Value) -> Result<(), Self> {
        let issues: Vec<_> = validator
            .iter_errors(input)
            .map(|error| ValidationIssue {
                error: error.to_owned(),
            })
            .collect();

        if issues.is_empty() {
            Ok(())
        } else {
            Err(Self { issues })
        }
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} JSON Schema validation failure(s)", self.issues.len())
    }
}

impl StdError for ValidationErrors {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.issues
            .first()
            .map(|issue| &issue.error as &(dyn StdError + 'static))
    }
}
