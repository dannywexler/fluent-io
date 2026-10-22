use camino::Utf8PathBuf;
use image::ImageError;
use std::{io, time::Duration};

use thiserror::Error;

use crate::command::CommandReadError;

#[derive(Debug, Error)]
#[error("Tried to access metadata of video file {path}\nGot Error {cause:#?}")]
pub struct VideoMetaDataError {
    pub path: Utf8PathBuf,
    pub cause: Box<VideoMetaDataErrorCause>,
}

#[derive(Debug)]
pub enum VideoMetaDataErrorCause {
    Io(io::Error),
    Ffprobe(CommandReadError),
    Serde(serde_json::Error),
    InvalidStructure(String),
}

#[derive(Debug, Error)]
#[error("Tried to extract frame of video file {path} at {:.2}seconds\nGot Error {cause:#?}", duration.as_secs_f32())]
pub struct VideoExtractFrameError {
    pub path: Utf8PathBuf,
    pub duration: Duration,
    pub cause: Box<CommandReadError>,
}

#[derive(Debug, Error)]
#[error("Tried to extract frame of video file {path}\nGot Error {cause:#?}")]
pub struct VideoPhashError {
    pub path: Utf8PathBuf,
    pub cause: Box<VideoPhashErrorCause>,
}

#[derive(Debug)]
pub enum VideoPhashErrorCause {
    Ffmpeg(CommandReadError),
    ImagePhash(ImageError),
}
