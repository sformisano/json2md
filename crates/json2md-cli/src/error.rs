use std::{error::Error as StdError, io, io::Write, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("could not read file '{}'", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not parse JSON from '{}'", path.display())]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("could not process JSON '{}' with schema '{}' and template '{}'", input.display(), schema.display(), template.display())]
    Library {
        input: PathBuf,
        schema: PathBuf,
        template: PathBuf,
        #[source]
        source: json2md::Error,
    },
    #[error("could not {operation} '{}'", path.display())]
    FileOutput {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not write Markdown to stdout")]
    Stdout {
        #[source]
        source: io::Error,
    },
    #[error("output '{}' resolves to supplied file '{}'; choose a separate output path", output.display(), input.display())]
    OutputAlias { output: PathBuf, input: PathBuf },
}

pub(crate) fn report(error: &Error, stderr: &mut impl Write) -> io::Result<()> {
    writeln!(stderr, "error: {error}")?;

    if let Error::Library {
        input,
        schema,
        source: json2md::Error::Validation(errors),
        ..
    } = error
    {
        writeln!(stderr, "  caused by: {errors}")?;
        for issue in errors.issues() {
            let root = if issue.instance_path().is_empty() {
                " (document root)"
            } else {
                ""
            };
            writeln!(
                stderr,
                "  {}#{}{root}; schema '{}' resource #{}: {issue}",
                input.display(),
                issue.instance_path(),
                schema.display(),
                issue.schema_path(),
            )?;
        }
        return Ok(());
    }

    let mut source = error.source();
    while let Some(cause) = source {
        writeln!(stderr, "  caused by: {cause}")?;
        source = cause.source();
    }
    Ok(())
}
