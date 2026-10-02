use which::CanonicalPath;

use crate::command::FluentCommand;

#[derive(Clone, Debug)]
pub struct FfprobeBinary(CanonicalPath);

impl FfprobeBinary {
    pub fn find() -> Result<FfprobeBinary, which::Error> {
        CanonicalPath::new("ffprobe").map(FfprobeBinary)
    }

    pub fn must_find() -> FfprobeBinary {
        FfprobeBinary::find().unwrap_or_else(|which_error| {
            panic!("Could not find ffprobe binary!\n{which_error}");
        })
    }

    pub fn path(&self) -> CanonicalPath {
        self.0.clone()
    }

    pub fn cmd(&self) -> FluentCommand {
        FluentCommand::new(self.0.clone())
    }
}
