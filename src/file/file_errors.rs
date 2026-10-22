use std::io;

use camino::Utf8PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Tried to access metadata of file {path}\nGot Io Error {io_error}")]
pub struct FileMetaDataError {
    pub path: Utf8PathBuf,
    pub io_error: io::Error,
}

#[derive(Debug, Error)]
#[error("Tried to write data for file {path}\nGot Io Error {io_error}")]
pub struct FileWriteDataError {
    pub path: Utf8PathBuf,
    pub io_error: io::Error,
}

#[derive(Debug, Error)]
#[error("Tried to copy file from {from} to {to}\nGot Io Error {io_error}")]
pub struct FileCopyToError {
    pub from: Utf8PathBuf,
    pub to: Utf8PathBuf,
    pub io_error: io::Error,
}

#[derive(Debug, Error)]
#[error("Tried to move file from {from} to {to}\nGot Io Error {io_error}")]
pub struct FileMoveToError {
    pub from: Utf8PathBuf,
    pub to: Utf8PathBuf,
    pub io_error: io::Error,
}

#[derive(Debug, Error)]
#[error("Tried to remove file {path}\nGot Io Error {io_error}")]
pub struct FileRemoveError {
    pub path: Utf8PathBuf,
    pub io_error: io::Error,
}
