use core::fmt;
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

use camino::Utf8PathBuf;

use crate::{
    file::{FileActionResult, FileActions, FileMetadata, FluentFile},
    folder::Folder,
};

#[derive(Debug)]
pub struct VideoFile {
    inner: FluentFile,
    format: VideoFormat,
}

impl VideoFile {
    pub fn new(folder: impl Into<Folder>, name: impl AsRef<str>, format: VideoFormat) -> VideoFile {
        VideoFile {
            inner: FluentFile::new(folder.into(), name, Some(format.to_string())),
            format,
        }
    }

    pub fn find_all(folder: impl Into<Folder>) -> impl Iterator<Item = VideoFile> {
        FluentFile::find_all(folder).filter_map(|ff| {
            let ffext = ff.ext()?;
            let format = VideoFormat::from_extension(ffext)?;
            Some(VideoFile::new(ff.parent(), ff.name(), format))
        })
    }

    pub fn format(&self) -> VideoFormat {
        self.format
    }

    pub fn ext(&self) -> String {
        self.format.to_string()
    }
}

impl Deref for VideoFile {
    type Target = FluentFile;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Display for VideoFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FileActions for VideoFile {
    fn name(&self) -> String {
        self.inner.name()
    }

    fn name_ext(&self) -> String {
        self.inner.name_ext()
    }

    fn parent(&self) -> Folder {
        self.inner.parent()
    }

    fn utf8_path_buf(&self) -> Utf8PathBuf {
        self.inner.utf8_path_buf()
    }

    fn exists(&self) -> bool {
        self.inner.exists()
    }

    fn metadata(&self) -> FileActionResult<FileMetadata> {
        self.inner.metadata()
    }

    fn with_name(&self, other_name: impl AsRef<str>) -> Self {
        VideoFile::new(self.inner.utf8_path_buf(), other_name.as_ref(), self.format)
    }

    fn with_folder(&self, folder: impl Into<Folder>) -> VideoFile {
        VideoFile::new(folder.into(), self.name(), self.format)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VideoFormat {
    Avi,
    Flv,
    Mkv,
    Mov,
    Mp4,
    Mpg,
    Rm,
    Wmv,
}

impl VideoFormat {
    pub fn from_extension(any_str: impl AsRef<str>) -> Option<VideoFormat> {
        match any_str.as_ref() {
            "avi" => Some(VideoFormat::Avi),
            "flv" => Some(VideoFormat::Flv),
            "mkv" => Some(VideoFormat::Mkv),
            "mov" => Some(VideoFormat::Mov),
            "mp4" => Some(VideoFormat::Mp4),
            "mpg" => Some(VideoFormat::Mpg),
            "rm" => Some(VideoFormat::Rm),
            "wmv" => Some(VideoFormat::Wmv),
            _ => None,
        }
    }
}

impl Display for VideoFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ext = match &self {
            VideoFormat::Avi => "avi",
            VideoFormat::Flv => "flv",
            VideoFormat::Mkv => "mkv",
            VideoFormat::Mov => "mov",
            VideoFormat::Mp4 => "mp4",
            VideoFormat::Mpg => "mpg",
            VideoFormat::Rm => "rm",
            VideoFormat::Wmv => "wmv",
        };
        write!(f, "{}", ext)
    }
}
