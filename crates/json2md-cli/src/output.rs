use std::{fs, io, io::Write, path::Path};

use tempfile::NamedTempFile;

use crate::error::Error;

pub(crate) fn write_file(path: &Path, markdown: &[u8], inputs: &[&Path]) -> Result<(), Error> {
    reject_input_alias(path, inputs)?;

    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let output_error = |operation, source| Error::FileOutput {
        operation,
        path: path.to_owned(),
        source,
    };
    let mut staged = NamedTempFile::new_in(parent)
        .map_err(|source| output_error("stage Markdown output at", source))?;
    staged
        .write_all(markdown)
        .map_err(|source| output_error("write Markdown output to", source))?;
    staged
        .flush()
        .map_err(|source| output_error("flush Markdown output for", source))?;
    staged
        .persist(path)
        .map_err(|error| output_error("replace Markdown output at", error.error))?;
    Ok(())
}

fn reject_input_alias(output: &Path, inputs: &[&Path]) -> Result<(), Error> {
    let destination = match fs::canonicalize(output) {
        Ok(path) => path,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(source) => {
            return Err(Error::FileOutput {
                operation: "resolve output path",
                path: output.to_owned(),
                source,
            });
        }
    };

    for input in inputs {
        let input_path = fs::canonicalize(input).map_err(|source| Error::FileOutput {
            operation: "resolve supplied file",
            path: input.to_path_buf(),
            source,
        })?;
        if destination == input_path {
            return Err(Error::OutputAlias {
                output: output.to_owned(),
                input: input.to_path_buf(),
            });
        }
    }
    Ok(())
}
