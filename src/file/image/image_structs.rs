use core::fmt;
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

use camino::Utf8PathBuf;
use image::{DynamicImage, ImageFormat, image_dimensions};

use crate::{
    file::{
        FileActionResult, FileActions, FileMetadata, FluentFile, ImageFileActionError,
        ImageFileActionResult,
    },
    folder::Folder,
};

pub struct ImageFile {
    inner: FluentFile,
    format: ImageFormat,
}

impl ImageFile {
    pub fn new(folder: impl Into<Folder>, name: impl AsRef<str>, format: ImageFormat) -> ImageFile {
        ImageFile {
            inner: FluentFile::new(folder.into(), name, Some(must_get_image_format_ext(format))),
            format,
        }
    }

    pub fn ext(&self) -> &'static str {
        must_get_image_format_ext(self.format)
    }

    pub fn mime_type(&self) -> &'static str {
        self.format.to_mime_type()
    }

    pub fn dimensions(&self) -> ImageFileActionResult<(u32, u32)> {
        image_dimensions(self.to_string()).map_err(|image_error| ImageFileActionError::Dimensions {
            path: self.utf8_path_buf(),
            image_error,
        })
    }

    pub fn open(&self) -> ImageFileActionResult<DynamicImage> {
        image::open(self.to_string()).map_err(|image_error| ImageFileActionError::Open {
            path: self.utf8_path_buf(),
            image_error,
        })
    }
}

impl Deref for ImageFile {
    type Target = FluentFile;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Display for ImageFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FileActions for ImageFile {
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
        ImageFile::new(self.inner.utf8_path_buf(), other_name.as_ref(), self.format)
    }

    fn with_folder(&self, folder: impl Into<Folder>) -> ImageFile {
        ImageFile::new(folder.into(), self.name(), self.format)
    }
}

fn must_get_image_format_ext(image_format: ImageFormat) -> &'static str {
    image_format.extensions_str().first().expect(
        "Cannot handle unknown image format without at least one extension returned by .extensions_str()",
    )
}
