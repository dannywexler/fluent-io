use which::CanonicalPath;

use crate::command::FluentCommand;

#[derive(Clone, Debug)]
pub struct FfprobeBinary(CanonicalPath);

impl FfprobeBinary {
    pub fn find() -> Result<FfprobeBinary, which::Error> {
        CanonicalPath::new("ffprobe").map(FfprobeBinary)
    }

    pub fn path(&self) -> CanonicalPath {
        self.0.clone()
    }

    pub fn cmd(&self) -> FluentCommand {
        FluentCommand::new(self.0.clone())
    }
}
