use std::{
    env::{current_dir, home_dir, temp_dir},
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
};

use camino::{Utf8Path, Utf8PathBuf};

use crate::folder::folder_actions::FolderActions;

static TILDE: &str = "~";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Folder(Utf8PathBuf);

impl Folder {
    pub fn new(path: impl AsRef<Path>) -> Folder {
        let path = path.as_ref().to_string_lossy();
        let abs_path = if path.starts_with(TILDE) {
            // remove "~" and join with full home folder
            must_get_home_dir().join(path.trim_start_matches(TILDE).trim_start_matches("/"))
        } else {
            // join with current folder
            must_get_current_dir().join(path.to_string())
        };
        Folder(
            abs_path
                .as_str()
                .trim_end_matches("/")
                .trim_end_matches("..")
                .into(),
        )
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Folder {
    fn from(value: &str) -> Self {
        Folder::new(value)
    }
}

impl From<String> for Folder {
    fn from(value: String) -> Self {
        Folder::new(value)
    }
}

impl From<&Path> for Folder {
    fn from(value: &Path) -> Self {
        Folder::new(value.to_string_lossy().to_string())
    }
}

impl From<PathBuf> for Folder {
    fn from(value: PathBuf) -> Self {
        Folder::new(value.to_string_lossy().to_string())
    }
}

impl From<&Utf8Path> for Folder {
    fn from(value: &Utf8Path) -> Self {
        Folder::new(value)
    }
}

impl From<Utf8PathBuf> for Folder {
    fn from(value: Utf8PathBuf) -> Self {
        Folder::new(value)
    }
}

impl FolderActions for Folder {
    fn current() -> Self {
        must_get_current_dir().into()
    }

    fn home() -> Self {
        must_get_home_dir().into()
    }

    fn temp() -> Self {
        temp_dir().into()
    }

    fn exists(&self) -> bool {
        self.0.exists()
    }

    fn name(&self) -> String {
        self.0
            .file_name()
            .unwrap_or_else(|| {
                eprintln!("Folder '{}' was missing a name (last piece of path).", self.0);
                eprintln!("This should be impossible because Folder::new constructor trims any trailing '/' or '..'");
                panic!("Could not get Utf8PathBuf.file_name()");
            })
            .to_string()
    }

    fn utf8_path_buf(&self) -> Utf8PathBuf {
        self.0.clone()
    }

    fn parent(&self) -> Self {
        self.0.parent().unwrap_or(&self.0).to_path_buf().into()
    }

    fn folder(&self, path_segment: impl AsRef<Path>) -> Self {
        self.0
            .join(path_segment.as_ref().to_string_lossy().to_string())
            .into()
    }
}

fn must_get_current_dir() -> Utf8PathBuf {
    let current_dir = current_dir().expect("Must be able to access current dir");
    let current_dir_str = current_dir.to_string_lossy().to_string();

    Utf8PathBuf::try_from(current_dir).unwrap_or_else(|pb_err| {
        eprintln!(
            "Expected current dir '{}' to be valid Utf8",
            current_dir_str
        );
        eprintln!("Got camino::FromPathBufError: {:?}", &pb_err);
        eprintln!(
            "  Contains an inner std::io::Error {:?}",
            pb_err.into_io_error()
        );
        panic!("Must be able to get current dir as Utf8");
    })
}

fn must_get_home_dir() -> Utf8PathBuf {
    let home_dir = home_dir().expect("Must be able to access home dir");
    let home_dir_str = home_dir.to_string_lossy().to_string();

    Utf8PathBuf::try_from(home_dir).unwrap_or_else(|pb_err| {
        eprintln!("Expected home dir '{}' to be valid Utf8", home_dir_str);
        eprintln!("Got camino::FromPathBufError: {:?}", &pb_err);
        eprintln!(
            "  Contains an inner std::io::Error {:?}",
            pb_err.into_io_error()
        );
        panic!("Must be able to get home dir as Utf8");
    })
}
