use std::{
    fmt::{self, Display, Formatter},
    io,
};

use camino::Utf8PathBuf;

#[derive(Debug)]
pub struct FileMetaDataError {
    pub path: Utf8PathBuf,
    pub io_error: io::Error,
}

impl Display for FileMetaDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tried to access metadata for file {}\nGot Io Error {}",
            self.path, self.io_error
        )
    }
}

#[derive(Debug)]
pub struct FileWriteDataError {
    pub path: Utf8PathBuf,
    pub io_error: io::Error,
}

impl Display for FileWriteDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tried to write data to file {}\nGot Io Error {}",
            self.path, self.io_error
        )
    }
}

#[derive(Debug)]
pub struct FileMoveToError {
    pub from: Utf8PathBuf,
    pub to: Utf8PathBuf,
    pub io_error: io::Error,
}

impl Display for FileMoveToError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tried to move file from {} to {}\nGot Io Error {}",
            self.from, self.to, self.io_error
        )
    }
}
