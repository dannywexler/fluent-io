use std::time::Duration;

use image::ImageFormat;
use jiff::Timestamp;

use crate::{
    command::FfmpegBinary,
    file::{
        DEFAULT_PHASH_SIMILARITY_THRESHOLD, FileActions, ImageFileActionError, ImagePhash,
        VideoFile, VideoFileActionError, VideoFileActionResult, VideoPhashError,
    },
    folder::{Folder, FolderActions},
};

#[derive(Debug)]
pub struct VideoPhash(Vec<ImagePhash>);

impl VideoPhash {
    pub fn image_phashes(&self) -> Vec<ImagePhash> {
        self.0.clone()
    }

    pub fn distance_to(&self, other: &VideoPhash) -> Vec<usize> {
        self.0
            .iter()
            .zip(&other.0)
            .map(|(a, b)| a.distance_to(b))
            .collect::<Vec<_>>()
    }

    pub fn is_similar_to(&self, other: &VideoPhash) -> bool {
        self.is_similar_to_custom_threshold(other, DEFAULT_PHASH_SIMILARITY_THRESHOLD)
    }

    pub fn is_similar_to_custom_threshold(&self, other: &VideoPhash, threshold: usize) -> bool {
        self.distance_to(other)
            .iter()
            .all(|dist| dist <= &threshold)
    }
}

impl From<Vec<ImagePhash>> for VideoPhash {
    fn from(value: Vec<ImagePhash>) -> Self {
        VideoPhash(value)
    }
}

impl VideoFile {
    pub fn phash(
        &self,
        video_duration: &Duration,
        ffmpeg: &FfmpegBinary,
    ) -> VideoFileActionResult<VideoPhash> {
        let video_duration_seconds = video_duration.as_secs_f32();
        let mut phashes = vec![];

        let dest_folder = Folder::temp()
            .folder("phashes")
            .folder(format!("{:x}", Timestamp::now().as_millisecond()));
        dest_folder.ensure_exists();

        let mut efb = self.extract_frame(ffmpeg);
        efb.set_image_folder(dest_folder.to_string());
        efb.set_image_format(ImageFormat::Jpeg);

        let mut maybe_error = None::<VideoFileActionError>;

        for pct in 1..=9 {
            let seconds = video_duration_seconds * pct as f32 / 10.0;
            efb.set_image_name(pct.to_string());
            match efb.extract(seconds) {
                Ok(img_file) => {
                    let phash =
                        img_file
                            .phash()
                            .map_err(|imgact_err| VideoFileActionError::Phash {
                                path: self.utf8_path_buf(),
                                cause: Box::new(VideoPhashError::ImagePhash(match imgact_err {
                                    ImageFileActionError::Phash { image_error, .. } => image_error,
                                    _ => panic!("Should only be possible to get image phash error"),
                                })),
                            });
                    if let Err(phash_err) = phash {
                        maybe_error = Some(phash_err);
                        break;
                    }
                    phashes.push(phash.unwrap());
                }
                Err(extract_error) => {
                    maybe_error = Some(extract_error);
                    break;
                }
            }
        }

        dest_folder.remove();
        if let Some(vid_err) = maybe_error {
            return Err(vid_err);
        }
        Ok(VideoPhash(phashes))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        command::{FfmpegBinary, FfprobeBinary},
        file::{FileActions, VideoFile, VideoFormat},
        folder::{Folder, FolderActions},
    };

    #[test]
    fn test_phashes() -> Result<(), Box<dyn std::error::Error>> {
        let vfile = VideoFile::new(
            Folder::current().folder("test_data/videos"),
            "corgi",
            VideoFormat::Mp4,
        );
        if !vfile.exists() {
            eprintln!("Skipping file that does not exist: {vfile}");
            return Ok(());
        }
        let vfile_meta =
            vfile
                .metadata(&FfprobeBinary::find().unwrap())
                .map_err(|vfile_mtaerr| {
                    eprintln!("Could not get video metadata for {vfile}");
                    eprintln!("{vfile_mtaerr:#?}");
                    assert!(false);
                    panic!();
                })?;
        println!("Got video metadata for {vfile}:");
        println!("{vfile_meta:#?}");
        match vfile.phash(&vfile_meta.duration, &FfmpegBinary::find().unwrap()) {
            Ok(vid_phash) => println!("Got valid vid_phash: {vid_phash:#?}"),
            Err(vph_error) => println!("Got vid_phash_error: {vph_error:#?}"),
        };
        Ok(())
    }
}
