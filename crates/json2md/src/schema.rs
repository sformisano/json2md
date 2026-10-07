use jsonschema::{Retrieve, Uri, Validator};
use serde_json::Value;

use crate::Error;

pub(crate) fn compile(schema: &Value) -> Result<Validator, Error> {
    jsonschema::options()
        .should_validate_formats(true)
        .should_ignore_unknown_formats(false)
        .with_retriever(NoExternalResources)
        .build(schema)
        .map_err(|source| Error::Schema {
            source: Box::new(source),
        })
}

struct NoExternalResources;

impl Retrieve for NoExternalResources {
    fn retrieve(
        &self,
        _uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(Box::new(ExternalResourceUnavailable))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("external schema retrieval is disabled; embed referenced schemas in the supplied document")]
struct ExternalResourceUnavailable;
