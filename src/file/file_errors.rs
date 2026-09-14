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
}

impl Display for FileActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FileActionError::MetaData { path, io_error } => write!(
                f,
                "Tried to access metadata for file '{path}'. Got IO Error: {io_error}."
            ),
        }
    }
}

pub type FileActionResult<T = ()> = Result<T, FileActionError>;
