use camino::Utf8PathBuf;
use image::ImageError;
use std::io;

use crate::command::CommandReadError;

#[derive(Debug)]
pub enum VideoFileActionError {
    Metadata {
        path: Utf8PathBuf,
        cause: Box<VideoMetaDataError>,
    },
    ExtractFrame {
        path: Utf8PathBuf,
        cause: Box<VideoFrameError>,
    },
    Phash {
        path: Utf8PathBuf,
        cause: Box<VideoPhashError>,
    },
}

#[derive(Debug)]
pub enum VideoMetaDataError {
    Io(io::Error),
    Ffprobe(CommandReadError),
    Serde(serde_json::Error),
    InvalidStructure(String),
}

#[derive(Debug)]
pub enum VideoFrameError {
    Io(io::Error),
    Ffmpeg(CommandReadError),
}

#[derive(Debug)]
pub enum VideoPhashError {
    Io(io::Error),
    Ffmpeg(CommandReadError),
    ImagePhash(ImageError),
}

pub type VideoFileActionResult<T = ()> = Result<T, VideoFileActionError>;
