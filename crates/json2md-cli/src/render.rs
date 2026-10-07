use std::{fs, io, io::Write, path::Path};

use json2md::View;
use serde_json::Value;

use crate::{args::RenderArgs, error::Error, output};

pub(crate) fn run(args: &RenderArgs) -> Result<(), Error> {
    let input = read_json(&args.input)?;
    let schema = read_json(&args.schema)?;
    let template = fs::read_to_string(&args.template).map_err(|source| Error::Read {
        path: args.template.clone(),
        source,
    })?;

    let library_error = |source| Error::Library {
        input: args.input.clone(),
        schema: args.schema.clone(),
        template: args.template.clone(),
        source,
    };
    let view = View::new(&schema, &template).map_err(library_error)?;
    let markdown = view.render(&input).map_err(library_error)?;

    if let Some(path) = &args.output {
        output::write_file(
            path,
            markdown.as_bytes(),
            &[&args.input, &args.schema, &args.template],
        )
    } else {
        let mut stdout = io::stdout().lock();
        stdout
            .write_all(markdown.as_bytes())
            .and_then(|()| stdout.flush())
            .map_err(|source| Error::Stdout { source })
    }
}

fn read_json(path: &Path) -> Result<Value, Error> {
    let bytes = fs::read(path).map_err(|source| Error::Read {
        path: path.to_owned(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| Error::Json {
        path: path.to_owned(),
        source,
    })
}
