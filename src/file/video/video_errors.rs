use camino::Utf8PathBuf;
use serde_json::{Map, Value};
use std::io;

use crate::{command::CommandReadError, file::VideoStream};

#[derive(Debug)]
pub enum VideoFileActionError {
    Phash {
        path: Utf8PathBuf,
    },
    Metadata {
        path: Utf8PathBuf,
        cause: VideoMetaDataError,
    },
}

#[derive(Debug)]
pub enum VideoMetaDataError {
    Io(io::Error),
    FfprobeMissing(which::Error),
    FfprobeExec(CommandReadError),
    Serde(serde_json::Error),
    InvalidStructure(String),
}

pub type VideoFileActionResult<T = ()> = Result<T, VideoFileActionError>;

pub fn parse_video_stream(
    raw_object: Map<String, Value>,
) -> Result<VideoStream, VideoMetaDataError> {
    let format_obj = raw_object
        .get("format")
        .ok_or_else(|| VideoMetaDataError::InvalidStructure("Missing format field".into()))?;

    let duration: f32 = format_obj
        .get("duration")
        .ok_or_else(|| {
            VideoMetaDataError::InvalidStructure("Missing format.duration field".into())
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

    let streams = raw_object["streams"].as_array().ok_or_else(|| {
        VideoMetaDataError::InvalidStructure("streams field was not an array".into())
    })?;
    for stream in streams {
        let mut stream2 = stream.clone();
        let stream = stream2.as_object_mut().ok_or_else(|| {
            VideoMetaDataError::InvalidStructure("streams.N was not an object".into())
        })?;

        let codec_type = stream
            .get("codec_type")
            .ok_or_else(|| {
                VideoMetaDataError::InvalidStructure("streams.N was missing codec_type".into())
            })?
            .as_str()
            .ok_or_else(|| {
                VideoMetaDataError::InvalidStructure(
                    "streams.N.codec_type field was not an string".into(),
                )
            })?;

        if codec_type != "video" {
            continue;
        }

        stream.insert("duration".into(), duration.into());
        stream.insert("bit_rate".into(), bit_rate.into());
        let vid_stream: VideoStream = serde_json::from_value(Value::Object(stream.clone()))
            .map_err(VideoMetaDataError::Serde)?;

        return Ok(vid_stream);
    }
    Err(VideoMetaDataError::InvalidStructure(
        "No stream with stream.codec_type == 'video' was found.".into(),
    ))
}
