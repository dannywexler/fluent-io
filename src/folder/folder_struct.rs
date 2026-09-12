use std::{
    env::{current_dir, home_dir},
    fmt::{self, Display, Formatter},
    path::Path,
};

use camino::Utf8PathBuf;

use crate::folder::folder_actions::FolderActions;

pub struct Folder(Utf8PathBuf);

impl Folder {
    pub fn new(path: impl AsRef<str>) -> Folder {
        Folder(
            Path::new(path.as_ref())
                .to_string_lossy()
                .to_string()
                .into(),
        )
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Utf8PathBuf> for Folder {
    fn from(value: Utf8PathBuf) -> Self {
        Folder(value)
    }
}

impl FolderActions for Folder {
    fn current() -> Self {
        Folder::new(
            current_dir()
                .expect("Must be able to access current dir")
                .to_string_lossy(),
        )
    }
    fn home() -> Self {
        Folder::new(
            home_dir()
                .expect("Must be able to access home dir")
                .to_string_lossy()
        )
    }

    fn name(&self) -> String {
        self.0
            .file_name()
            .unwrap_or_else(|| panic!("Folder: {} was missing file name", self.0))
            .to_string()
    }

    fn exists(&self) -> bool {
        self.0.exists()
    }
}
