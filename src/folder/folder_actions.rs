use std::fmt::Display;

use camino::Utf8PathBuf;

pub trait FolderActions: Clone + Display {
    fn current() -> Self;
    fn home() -> Self;
    fn exists(&self) -> bool;
    fn name(&self) -> String;
    fn path_buf(&self) -> Utf8PathBuf;
}
