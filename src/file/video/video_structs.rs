use core::fmt;
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    time::Duration,
};

use camino::Utf8PathBuf;
use image::ImageFormat;
use jiff::Timestamp;
use monostate::MustBe;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    command::{FfmpegBinary, FfprobeBinary},
    file::{
        FileActionError, FileActionResult, FileActions, FileMetadata, FluentFile, ImageFile,
        VideoFileActionError, VideoFileActionResult, VideoFrameError,
        VideoMetaDataError::{self, Ffprobe},
    },
    folder::{Folder, FolderActions},
};

#[derive(Debug, Clone)]
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

    pub fn metadata(&self, ffprobe: &FfprobeBinary) -> VideoFileActionResult<VideoMetaData> {
        let file_metadata = self.inner.metadata().map_err(|fae| {
            let io_err = match fae {
                FileActionError::MetaData { io_error, .. } => io_error,
                _ => panic!("Should only be possible to get metadata error"),
            };
            VideoFileActionError::Metadata {
                path: self.utf8_path_buf(),
                cause: Box::new(VideoMetaDataError::Io(io_err)),
            }
        })?;

        let ffprobe_cmd = ffprobe
            .cmd()
            .args([
                "-v",
                "error",
                "-show_format",
                "-show_streams",
                "-print_format",
                "json",
                self.utf8_path_buf().as_str(),
            ])
            .read()
            .map_err(|fcmd_err| VideoFileActionError::Metadata {
                path: self.utf8_path_buf(),
                cause: Box::new(Ffprobe(fcmd_err)),
            })?;

        let ffprobe_output: Value = serde_json::from_str(&ffprobe_cmd.stdout).map_err(|srde| {
            VideoFileActionError::Metadata {
                path: self.utf8_path_buf(),
                cause: Box::new(VideoMetaDataError::Serde(srde)),
            }
        })?;

        let format_stream: FormatStream =
            ffprobe_output
                .clone()
                .try_into()
                .map_err(|cause| VideoFileActionError::Metadata {
                    path: self.utf8_path_buf(),
                    cause: Box::new(cause),
                })?;

        let video_stream: VideoStream =
            ffprobe_output
                .try_into()
                .map_err(|cause| VideoFileActionError::Metadata {
                    path: self.utf8_path_buf(),
                    cause: Box::new(cause),
                })?;

        Ok(VideoMetaData {
            bit_rate: format_stream.bit_rate,
            bytes: file_metadata.bytes,
            codec: video_stream.codec_name,
            created: file_metadata.created,
            duration: Duration::from_secs_f32(format_stream.duration),
            height: video_stream.height,
            modified: file_metadata.modified,
            width: video_stream.width,
        })
    }

    pub fn extract_frame(&self, ffmpeg: &FfmpegBinary) -> ExtractFrameBuilder {
        ExtractFrameBuilder::new(self, ffmpeg)
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
    WebM,
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
            "mpg" | "MPG" => Some(VideoFormat::Mpg),
            "rm" => Some(VideoFormat::Rm),
            "webm" => Some(VideoFormat::WebM),
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
            VideoFormat::WebM => "webm",
            VideoFormat::Wmv => "wmv",
        };
        write!(f, "{}", ext)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoMetaData {
    pub bit_rate: u32,
    pub bytes: u64,
    pub codec: VideoCodec,
    pub created: Option<Timestamp>,
    pub duration: Duration,
    pub height: u16,
    pub modified: Timestamp,
    pub width: u16,
}

struct FormatStream {
    pub bit_rate: u32,
    pub duration: f32,
}

impl TryFrom<Value> for FormatStream {
    type Error = VideoMetaDataError;

    fn try_from(raw_ffprobe_output: Value) -> Result<Self, Self::Error> {
        let format_obj = raw_ffprobe_output.get("format").ok_or_else(|| {
            VideoMetaDataError::InvalidStructure("ffprobe output was missing format field".into())
        })?;

        let duration: f32 = format_obj
            .get("duration")
            .ok_or_else(|| {
                VideoMetaDataError::InvalidStructure("format.duration was missing".into())
            })?
            .as_str()
            .ok_or_else(|| {
                VideoMetaDataError::InvalidStructure("format.duration was not a string".into())
            })?
            .parse()
            .map_err(|parse_err| {
                VideoMetaDataError::InvalidStructure(format!(
                    "format.duration could not be parsed to a f32: {parse_err:#?}"
                ))
            })?;

        let bit_rate: u32 = format_obj
            .get("bit_rate")
            .ok_or_else(|| {
                VideoMetaDataError::InvalidStructure("Missing format.bit_rate field".into())
            })?
            .as_str()
            .ok_or_else(|| {
                VideoMetaDataError::InvalidStructure("format.bit_rate was not a string".into())
            })?
            .parse()
            .map_err(|parse_err| {
                VideoMetaDataError::InvalidStructure(format!(
                    "format.bit_rate could not be parsed to a u32: {parse_err:#?}"
                ))
            })?;
        Ok(FormatStream { duration, bit_rate })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct VideoStream {
    codec_type: MustBe!("video"),
    pub codec_name: VideoCodec,
    pub height: u16,
    pub width: u16,
}

impl TryFrom<Value> for VideoStream {
    type Error = VideoMetaDataError;

    fn try_from(raw_ffprobe_output: Value) -> Result<Self, Self::Error> {
        let streams = raw_ffprobe_output
            .get("streams")
            .ok_or_else(|| {
                VideoMetaDataError::InvalidStructure(
                    "ffprobe output was missing streams field".into(),
                )
            })?
            .as_array()
            .ok_or_else(|| {
                VideoMetaDataError::InvalidStructure("streams value was not an array".into())
            })?;

        for stream in streams {
            if let Ok(vid_stream) = serde_json::from_value(stream.clone()) {
                return Ok(vid_stream);
            }
            continue;
        }
        Err(VideoMetaDataError::InvalidStructure(
            "No stream with codec_type of 'video' was found.".into(),
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoCodec {
    Av1,
    Cinepak,
    Flv1,
    H264,
    Hevc,
    Mjpeg,
    Mpeg1Video,
    Mpeg2Video,
    Mpeg4,
    Msmpeg4V1,
    Msmpeg4V2,
    Msmpeg4V3,
    Prores,
    Rv10,
    Rv20,
    Rv30,
    Rv40,
    Svq3,
    Vc1,
    Vp6F,
    Vp9,
    Wmv1,
    Wmv2,
    Wmv3,
}

pub struct ExtractFrameBuilder {
    source_video: VideoFile,
    ffmpeg: FfmpegBinary,
    image_format: ImageFormat,
    image_folder: Folder,
    image_name: String,
    custom_width: Option<u16>,
    custom_height: Option<u16>,
}

impl ExtractFrameBuilder {
    pub fn new(source_video: &VideoFile, ffmpeg: &FfmpegBinary) -> Self {
        ExtractFrameBuilder {
            source_video: source_video.to_owned(),
            ffmpeg: ffmpeg.to_owned(),
            image_format: ImageFormat::Avif,
            image_folder: source_video.parent().clone(),
            image_name: source_video.name(),
            custom_width: None,
            custom_height: None,
        }
    }

    pub fn set_image_format(&mut self, new_image_format: ImageFormat) -> &mut Self {
        self.image_format = new_image_format;
        self
    }

    pub fn set_image_folder(&mut self, new_destination_folder: impl Into<Folder>) -> &mut Self {
        self.image_folder = new_destination_folder.into();
        self
    }

    pub fn set_image_name(&mut self, new_name: impl AsRef<str>) -> &mut Self {
        self.image_name = new_name.as_ref().to_string();
        self
    }

    pub fn set_custom_width(&mut self, custom_width: u16) -> &mut Self {
        self.custom_width = Some(custom_width);
        self
    }

    pub fn set_custom_height(&mut self, custom_height: u16) -> &mut Self {
        self.custom_height = Some(custom_height);
        self
    }

    pub fn extract(&self, seconds: f32) -> VideoFileActionResult<ImageFile> {
        self.image_folder.ensure_exists();
        let dest_path = ImageFile::new(
            self.image_folder.clone(),
            self.image_name.clone(),
            self.image_format,
        );

        let mut fcmd = self.ffmpeg.cmd();
        fcmd.args([
            "-v",
            "error",
            "-ss",
            seconds.to_string().as_str(),
            "-i",
            self.source_video.to_string().as_str(),
            "-frames:v",
            "1",
            "-y",
            dest_path.to_string().as_str(),
        ]);

        let scale = match (self.custom_width, self.custom_height) {
            (None, None) => None,
            (Some(c_width), None) => Some(format!("{c_width}:-1")),
            (None, Some(c_height)) => Some(format!("-1:{c_height}")),
            (Some(c_width), Some(c_height)) => Some(format!("{c_width}:{c_height}")),
        };

        if let Some(scale_inner) = scale {
            fcmd.args(["-vf", format!("scale={scale_inner}").as_str()]);
        }

        fcmd.read()
            .map_err(|fcmd_err| VideoFileActionError::ExtractFrame {
                path: self.source_video.utf8_path_buf(),
                cause: Box::new(VideoFrameError::Ffmpeg(fcmd_err)),
            })?;

        Ok(dest_path)
    }
}
