use jsonschema::Validator;
use minijinja::{AutoEscape, Environment, UndefinedBehavior};
use serde_json::Value;

use crate::{Error, ValidationErrors, context, schema};

const TEMPLATE_NAME: &str = "view.md";

/// One compiled JSON Schema and Markdown template, reusable across inputs.
///
/// Construction owns its compiled assets and does not borrow the supplied schema
/// or template. A compiled pair establishes each asset's validity separately;
/// it does not establish that the template can render every schema-valid input.
pub struct View {
    validator: Validator,
    environment: Environment<'static>,
}

impl View {
    /// Compiles a schema and a standalone MiniJinja template.
    ///
    /// JSON Schema drafts are detected from `$schema`; an omitted declaration
    /// uses draft 2020-12. Known formats are asserted; unknown formats in compiled
    /// validation constraints fail construction. References inside the document
    /// are supported; external retrieval is disabled. The template has built-in
    /// filters and tests, strict undefined values, no automatic escaping, and
    /// preserves its trailing newline. No template loader is registered.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Schema`] when schema compilation fails, or
    /// [`Error::Template`] when template compilation fails. The schema is
    /// compiled first.
    pub fn new(schema: &Value, template: &str) -> Result<Self, Error> {
        let validator = schema::compile(schema)?;
        let mut environment = Environment::new();
        environment.set_undefined_behavior(UndefinedBehavior::Strict);
        environment.set_auto_escape_callback(|_| AutoEscape::None);
        environment.set_keep_trailing_newline(true);
        environment
            .add_template_owned(TEMPLATE_NAME, template.to_owned())
            .map_err(|source| Error::Template {
                source: Box::new(source),
            })?;

        Ok(Self {
            validator,
            environment,
        })
    }

    /// Validates input without executing the template.
    ///
    /// # Errors
    ///
    /// Returns the reported schema failures with owned diagnostics and paths.
    pub fn validate(&self, input: &Value) -> Result<(), ValidationErrors> {
        ValidationErrors::check(&self.validator, input)
    }

    /// Validates input and renders it as Markdown.
    ///
    /// Object properties become top-level template variables. Output is returned
    /// only after the entire render succeeds. Input is neither mutated nor
    /// normalized; schema defaults do not fill missing properties. Integer
    /// representations use the renderer's 128-bit range; decimal and exponent
    /// representations use finite `f64` and can lose precision.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] before template execution if input is
    /// invalid. Schema-valid input can produce [`Error::Render`], for example
    /// when a template accesses a missing property, uses an invalid operation,
    /// or input contains a number outside the renderer's supported range.
    pub fn render(&self, input: &Value) -> Result<String, Error> {
        self.validate(input)?;
        self.environment
            .get_template(TEMPLATE_NAME)
            .and_then(|template| template.render(context::from_json(input)?))
            .map_err(|source| Error::Render {
                source: Box::new(source),
            })
    }
}
