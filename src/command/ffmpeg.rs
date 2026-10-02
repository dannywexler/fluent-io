use which::CanonicalPath;

use crate::command::FluentCommand;

#[derive(Clone, Debug)]
pub struct FfmpegBinary(CanonicalPath);

impl FfmpegBinary {
    pub fn find() -> Result<FfmpegBinary, which::Error> {
        CanonicalPath::new("ffmpeg").map(FfmpegBinary)
    }

    pub fn must_find() -> FfmpegBinary {
        FfmpegBinary::find().unwrap_or_else(|which_error| {
            panic!("Could not find ffmpeg binary!\n{which_error}");
        })
    }

    pub fn path(&self) -> CanonicalPath {
        self.0.clone()
    }

    pub fn cmd(&self) -> FluentCommand {
        FluentCommand::new(self.0.clone())
    }
}
