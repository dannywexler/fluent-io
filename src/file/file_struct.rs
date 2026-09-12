use std::fmt::Display;

use crate::{file::FileActions, folder::FolderActions};

pub struct FluentFile<FolderLike: FolderActions> {
    folder: FolderLike,
    ext: Option<String>,
    name: String,
}

impl<FolderLike: FolderActions> FluentFile<FolderLike> {
    pub fn new(folder: FolderLike, name: impl AsRef<str>, ext: Option<impl AsRef<str>>) -> Self {
        FluentFile {
            folder,
            name: name.as_ref().to_string(),
            ext: ext.map(|item| item.as_ref().to_string()),
        }
    }

    pub fn ext(&self) -> Option<String> {
        self.ext.clone()
    }
}

impl<FolderLike: FolderActions> Display for FluentFile<FolderLike> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.folder, self.name_ext())
    }
}

impl<FolderLike: FolderActions> FileActions for FluentFile<FolderLike> {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn name_ext(&self) -> String {
        match &self.ext {
            Some(ext) => format!("{}.{}", self.name, ext),
            None => self.name.clone(),
        }
    }

    fn parent(&self) -> impl FolderActions {
        self.folder.clone()
    }
}
