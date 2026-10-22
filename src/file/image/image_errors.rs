use camino::Utf8PathBuf;
use image::ImageError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Tried to open image file {path}\nGot Image Error {image_error}")]
pub struct ImageOpenError {
    pub path: Utf8PathBuf,
    pub image_error: ImageError,
}
