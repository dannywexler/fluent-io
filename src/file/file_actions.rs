use camino::Utf8PathBuf;

use crate::{
    file::{FileActionResult, FileMetadata},
    folder::Folder,
};

pub trait FileActions {
    fn name(&self) -> String;
    fn name_ext(&self) -> String;
    fn parent(&self) -> Folder;
    fn utf8_path_buf(&self) -> Utf8PathBuf;
    fn exists(&self) -> bool;
    fn metadata(&self) -> FileActionResult<FileMetadata>;
    fn with_name(&self, other_name: impl AsRef<str>) -> Self;
    fn with_folder(&self, folder: impl Into<Folder>) -> Self;
}

#[cfg(test)]
mod tests {
    use crate::file::*;
    use crate::folder::*;

    #[test]
    fn test_file_name_ext() {
        let files = [
            FluentFile::new(Folder::current(), "Cargo", Some("toml")),
            FluentFile::named(Folder::current(), "Cargo", "toml"),
            FluentFile::new(Folder::current(), "Cargo", None::<String>),
            FluentFile::named_no_ext(Folder::current(), "Cargo"),
        ];
        for f in files {
            let name = f.name();
            let name_ext = f.name_ext();
            println!("File: '{}'", f);
            println!("  name_ext: '{}'", name_ext);
            println!("  name:     '{}'", name);
            if let Some(ext) = f.ext() {
                println!("  ext:      '{}'", ext);
                assert_eq!(format!("{}.{}", name, ext), name_ext)
            } else {
                println!("  ext:      <NONE>");
                assert_eq!(name, name_ext)
            }
            println!("");
        }
    }

    #[test]
    fn test_metadata_exists() {
        let files = [
            FluentFile::new(Folder::current(), "Cargo", Some("toml")),
            FluentFile::new(Folder::current(), "hello", Some("txt")),
            FluentFile::new(Folder::current(), "Has spaces", Some("ext")),
        ];
        for f in files {
            if f.exists() {
                let mtd = f.metadata().unwrap();
                println!("Got metadata for existing file: '{f}'");
                println!("Debug:   {mtd:?}");
            } else {
                let mtd_err = f.metadata().unwrap_err();
                println!("Got metadata error for missing file: '{f}'");
                println!("Display: {mtd_err}");
                println!("Debug:   {mtd_err:?}");
            }
            println!("");
        }
    }
}
