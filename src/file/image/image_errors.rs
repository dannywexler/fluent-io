use camino::Utf8PathBuf;
use image::ImageError;

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ImageFileActionError {
    Dimensions {
        path: Utf8PathBuf,
        image_error: ImageError,
    },
    Open {
        path: Utf8PathBuf,
        image_error: ImageError,
    },
}

impl Display for ImageFileActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (action, path, image_error) = match self {
            ImageFileActionError::Dimensions { path, image_error } => {
                ("get dimensions of", path, image_error)
            }
            ImageFileActionError::Open { path, image_error } => ("open", path, image_error),
        };
        write!(
            f,
            "Tried to {action} image file '{path}'. Got Image Error: {image_error}"
        )
    }
}

pub type ImageFileActionResult<T = ()> = Result<T, ImageFileActionError>;
