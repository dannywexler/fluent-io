use std::{
    env::{current_dir, home_dir},
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
};

use camino::Utf8PathBuf;

use crate::folder::folder_actions::FolderActions;

#[derive(Clone)]
pub struct Folder(Utf8PathBuf);

impl Folder {
    pub fn new(path: impl AsRef<Path>) -> Folder {
        path.as_ref().to_path_buf().into()
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PathBuf> for Folder {
    fn from(value: PathBuf) -> Self {
        Folder(value.to_string_lossy().to_string().into())
    }
}

impl FolderActions for Folder {
    fn current() -> Self {
        current_dir()
            .expect("Must be able to access current dir")
            .into()
    }
    fn home() -> Self {
        home_dir().expect("Must be able to access home dir").into()
    }

    fn exists(&self) -> bool {
        self.0.exists()
    }

    fn name(&self) -> String {
        self.0
            .file_name()
            .unwrap_or_else(|| panic!("Folder: {} was missing file name", self.0))
            .to_string()
    }

    fn path_buf(&self) -> Utf8PathBuf {
        self.0.clone()
    }
}
