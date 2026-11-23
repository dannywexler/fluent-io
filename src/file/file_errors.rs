use std::{fmt::Display, io, path::PathBuf};

use camino::Utf8PathBuf;
use mime::Mime;
use thiserror::Error;

#[derive(Debug)]
pub enum FilePathParseError {
    MissingParent { provided_path: PathBuf },
    MissingName { provided_path: PathBuf },
    NotUTF8,
}

#[derive(Debug, Error)]
#[error("Tried to access metadata of file {path}\nGot Io Error {io_error}")]
pub struct FileMetaDataError {
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

#[derive(Debug, Error)]
#[error("Tried to write data for file {path}\nGot Io Error {io_error}")]
pub struct FileWriteDataError {
    pub path: Utf8PathBuf,
    pub io_error: io::Error,
}

#[derive(Debug, Error)]
#[error("Tried to read file to string {path}\nGot Io Error {io_error}")]
pub struct FileReadToStringError {
    pub path: Utf8PathBuf,
    pub io_error: io::Error,
}

#[derive(Debug)]
pub enum FileReadToStructError {
    ReadToVec {
        path: Utf8PathBuf,
        io_error: io::Error,
    },
    ParseJson {
        path: Utf8PathBuf,
        serde_json_error: serde_json::Error,
    },
    UnsupportedFormat {
        path: Utf8PathBuf,
        mime: Mime,
    },
}

impl Display for FileReadToStructError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (path, msg) = match self {
            FileReadToStructError::ReadToVec { path, io_error } => {
                (path, format!("Got Io Error {io_error}"))
            }
            FileReadToStructError::ParseJson {
                path,
                serde_json_error,
            } => (path, format!("Got serde_json error {serde_json_error}")),
            FileReadToStructError::UnsupportedFormat { path, mime } => {
                (path, format!("Format {mime} is currently unsupported."))
            }
        };
        write!(f, "Tried to read file to struct {path}\n{msg}")
    }
}
