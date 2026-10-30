use std::ops::{Deref, DerefMut};

use crate::command::FluentCommand;

#[derive(Clone, Debug)]
pub struct FfmpegBinary(FluentCommand);

impl FfmpegBinary {
    pub fn find() -> Option<FfmpegBinary> {
        FluentCommand::find("ffmpeg").map(FfmpegBinary)
    }
}

impl Deref for FfmpegBinary {
    type Target = FluentCommand;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FfmpegBinary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
