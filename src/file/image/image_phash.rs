use std::{
    fmt::{self, Display, Formatter},
    io,
};

use image::ImageError;
use imghash::{ImageHash, perceptual_hash};

use crate::file::{FileActions, ImageFile, ImageFileActionError, ImageFileActionResult};

pub const DEFAULT_PHASH_SIMILARITY_THRESHOLD: usize = 6;

#[derive(Debug)]
pub struct ImagePhash(String);

impl ImagePhash {
    pub fn distance_to(&self, other: &ImagePhash) -> usize {
        if self.0 == other.0 {
            return 0;
        }
        let self_enc = must_decode_phash(self);
        let other_enc = must_decode_phash(other);
        self_enc.distance(&other_enc).unwrap_or_else(|phe| panic!("Should be impossible to ever have an error calculating the distance between two ImageHashes. Instead got error {phe:?}"))
    }

    pub fn is_similar_to(&self, other: &ImagePhash) -> bool {
        self.is_similar_to_custom_threshold(other, DEFAULT_PHASH_SIMILARITY_THRESHOLD)
    }

    pub fn is_similar_to_custom_threshold(&self, other: &ImagePhash, threshold: usize) -> bool {
        if self.0 == other.0 {
            return true;
        }
        self.distance_to(other) <= threshold
    }
}

impl Display for ImagePhash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ImageFile {
    pub fn phash(&self) -> ImageFileActionResult<ImagePhash> {
        let path = self.utf8_path_buf();
        if !self.exists() {
            let image_error =
                ImageError::IoError(io::Error::new(io::ErrorKind::NotFound, self.to_string()));
            return Err(ImageFileActionError::Phash { path, image_error });
        }

        let image_hash = perceptual_hash(path.as_std_path()).map_err(|phe| match phe {
                    imghash::ImageHashError::ImageError(image_error) => ImageFileActionError::Phash { path, image_error },
                    _ => panic!("Should only ever be possible to get io::ErrorKind::NotFound if the file does not exist, or image::ImageError when the file exists but could not be phashed. Already checked that the file exists before attempting to get phash of it."),
                })?;

        let phash_str = image_hash
                .encode().unwrap_or_else(|phe| panic!("Should be impossible to have an error encoding a successfully created ImageHash instance. Instead got error {phe:?}"));
        Ok(ImagePhash(phash_str))
    }
}

fn must_decode_phash(image_phash: &ImagePhash) -> ImageHash {
    ImageHash::decode(&image_phash.0, 8, 8).unwrap_or_else(|phe| panic!("Should be impossible to ever have an error decoding an ImagePhash's internal string because those internal strings are only ever encoded with the default 8x8 size. Instead got error {phe:?}"))
}

#[cfg(test)]
mod tests {
    use ::image::ImageFormat;

    use crate::file::*;
    use crate::folder::*;

    #[test]
    fn test_phash() {
        let files = [
            ImageFile::new(
                Folder::current().folder("test_data/images"),
                "beach",
                ImageFormat::Jpeg,
            ),
            ImageFile::new(Folder::current(), "Cargo", ImageFormat::Png),
        ];
        for fimg in files {
            if fimg.exists() {
                match fimg.phash() {
                    Ok(valid_phash) => println!("Got valid phash for existing file: {valid_phash}"),
                    Err(phe) => {
                        println!(
                            "Should never be able to get an error for an existing image. Got error: {phe:?}"
                        );
                        assert!(false);
                    }
                };
            } else {
                match fimg.phash() {
                    Ok(valid_phash) => {
                        println!(
                            "Should never be able to get a valid phash for an non-existant image. Got valid_phash: {valid_phash}"
                        );
                        assert!(false);
                    }
                    Err(phe) => {
                        println!("Got phash error for file that does not exist: {phe:?}")
                    }
                }
            }
        }
    }
}
