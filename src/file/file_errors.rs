use std::{
    fmt::{self, Display, Formatter},
    io,
};

use camino::Utf8PathBuf;

#[derive(Debug)]
pub enum FileActionError {
    MetaData {
        path: Utf8PathBuf,
        io_error: io::Error,
    },
    WriteData {
        path: Utf8PathBuf,
        io_error: io::Error,
    },
}

impl Display for FileActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (action, path, io_error) = match self {
            FileActionError::MetaData { path, io_error } => ("access metadata for", path, io_error),
            FileActionError::WriteData { path, io_error } => ("write to", path, io_error),
        };
        write!(
            f,
            "Tried to {action} file '{path}'. Got IO Error: {io_error}."
        )
    }
}

pub type FileActionResult<T = ()> = Result<T, FileActionError>;
