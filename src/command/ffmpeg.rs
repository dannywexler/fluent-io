use which::CanonicalPath;

use crate::command::FluentCommand;

#[derive(Clone, Debug)]
pub struct FfmpegBinary(CanonicalPath);

impl FfmpegBinary {
    pub fn find() -> Result<FfmpegBinary, which::Error> {
        CanonicalPath::new("ffmpeg").map(FfmpegBinary)
    }

    pub fn path(&self) -> CanonicalPath {
        self.0.clone()
    }

    pub fn cmd(&self) -> FluentCommand {
        FluentCommand::new(self.0.clone())
    }
}
