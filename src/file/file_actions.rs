use camino::Utf8PathBuf;

use crate::{
    file::{FileCopyToError, FileMoveToError, FileRemoveError},
    folder::Folder,
};

pub trait FileActions<T> {
    fn name(&self) -> String;
    fn name_ext(&self) -> String;
    fn parent(&self) -> Folder;
    fn utf8_path_buf(&self) -> Utf8PathBuf;
    fn exists(&self) -> bool;
    fn with_name(&self, other_name: impl AsRef<str>) -> Self;
    fn with_folder(&self, folder: impl Into<Folder>) -> Self;

    /// Finds all files (recursively) in a [`Folder`] using [`WalkDir`]. Skips any entry with an
    /// error, or any entry whose file_type is not a file
    fn find_all(folder: impl Into<Folder>) -> impl Iterator<Item = T>;

    /// Copies the file to another location.
    /// Before copying, ensures that the destination file's folder exists
    fn copy_to(&self, to: &T) -> Result<T, FileCopyToError>;

    /// Moves the file to another location
    /// Before moving, ensures that the destination file's folder exists
    fn move_to(&self, to: &T) -> Result<T, FileMoveToError>;

    fn remove(self) -> Result<(), FileRemoveError>;
}

#[cfg(test)]
mod tests {
    use crate::file::*;
    use crate::folder::*;

    #[test]
    fn test_file_name_ext() {
        let files = [
            FluentFile::new(Folder::current(), "Cargo", Some("toml")),
            FluentFile::new_with_ext(Folder::current(), "Cargo", "toml"),
            FluentFile::new(Folder::current(), "Cargo", None::<String>),
            FluentFile::new_no_ext(Folder::current(), "Cargo"),
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

    #[test]
    fn test_find_all() {
        let valid_folder = Folder::current().folder("src");
        for file in FluentFile::find_all(valid_folder) {
            println!("Found file: {file}");
        }
    }
}
