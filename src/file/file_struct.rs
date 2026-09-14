use std::fmt::Display;

use crate::{file::FileActions, folder::Folder};

pub struct FluentFile {
    folder: Folder,
    ext: Option<String>,
    name: String,
}

impl FluentFile {
    pub fn new(folder: &Folder, name: impl AsRef<str>, ext: Option<impl AsRef<str>>) -> Self {
        FluentFile {
            folder: folder.clone(),
            name: name.as_ref().to_string(),
            ext: ext.map(|item| item.as_ref().to_string()),
        }
    }

    pub fn ext(&self) -> Option<String> {
        self.ext.clone()
    }
}

impl Display for FluentFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.folder, self.name_ext())
    }
}

impl FileActions for FluentFile {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn name_ext(&self) -> String {
        match &self.ext {
            Some(ext) => format!("{}.{}", self.name, ext),
            None => self.name.clone(),
        }
    }

    fn parent(&self) -> Folder {
        self.folder.clone()
    }
}
