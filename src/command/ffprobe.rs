use std::ops::{Deref, DerefMut};

use crate::command::FluentCommand;

#[derive(Clone, Debug)]
pub struct FfprobeBinary(FluentCommand);

impl FfprobeBinary {
    pub fn find() -> Option<FfprobeBinary> {
        FluentCommand::find("ffprobe").map(FfprobeBinary)
    }
}

impl Deref for FfprobeBinary {
    type Target = FluentCommand;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FfprobeBinary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
