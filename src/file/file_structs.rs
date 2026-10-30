use core::fmt;
use std::{
    fmt::{Display, Formatter},
    fs,
    time::SystemTime,
};

use camino::Utf8PathBuf;
use jiff::Timestamp;
use walkdir::WalkDir;

use crate::{
    file::{
        FileActions, FileCopyToError, FileMetaDataError, FileMoveToError,
        FileRemoveError, FileWriteDataError,
    },
    folder::{Folder, FolderActions},
};

#[derive(Debug, Clone)]
pub struct FluentFile {
    folder: Folder,
    name: String,
    ext: Option<String>,
}

impl FluentFile {
    /// Creates a new [`FluentFile`] in a [`Folder`], with a name and optional extension
    pub fn new(
        folder: impl Into<Folder>,
        name: impl AsRef<str>,
        ext: Option<impl AsRef<str>>,
    ) -> Self {
        FluentFile {
            folder: folder.into(),
            name: name.as_ref().to_string(),
            ext: ext.map(|item| item.as_ref().to_string()),
        }
    }

    /// Creates a new [`FluentFile`] in a [`Folder`], with a name and extension
    pub fn new_with_ext(
        folder: impl Into<Folder>,
        name: impl AsRef<str>,
        ext: impl AsRef<str>,
    ) -> Self {
        FluentFile::new(folder.into(), name.as_ref(), Some(ext.as_ref().to_string()))
    }

    /// Creates a new [`FluentFile`] in a [`Folder`], with a name and without an extension
    pub fn new_no_ext(folder: impl Into<Folder>, name: impl AsRef<str>) -> Self {
        FluentFile::new(folder.into(), name.as_ref(), None::<String>)
    }

    /// Finds all files (recursively) in a [`Folder`] using [`WalkDir`]. Skips any entry with an
    /// error, or any entry whose file_type is not a file
    pub fn find_all(folder: impl Into<Folder>) -> impl Iterator<Item = FluentFile> {
        WalkDir::new(folder.into().utf8_path_buf())
            .sort_by_file_name()
            .into_iter()
            .filter_map(|dir_entry| {
                let dir_ent = dir_entry.ok()?;
                if !dir_ent.file_type().is_file() {
                    return None;
                }
                let path = dir_ent.path();

                let folder = path.parent()?.to_str()?;
                let name = path.file_stem()?.to_str()?;
                let mut ext = None;
                if let Some(os_str) = path.extension() {
                    ext = Some(os_str.to_str()?);
                }
                Some(FluentFile::new(folder, name, ext))
            })
    }

    /// Gets the file extension
    pub fn ext(&self) -> Option<String> {
        self.ext.clone()
    }

    /// Copies the file to another location.
    /// Before copying, ensures that the destination file's folder exists
    pub fn copy_to(&self, to: &FluentFile) -> Result<FluentFile, FileCopyToError> {
        to.parent().ensure_exists();
        fs::copy(self.utf8_path_buf(), to.utf8_path_buf())
            .map(|_| to.clone())
            .map_err(|io_error| FileCopyToError {
                from: self.utf8_path_buf(),
                to: to.utf8_path_buf(),
                io_error,
            })
    }

    /// Moves the file to another location
    /// Before moving, ensures that the destination file's folder exists
    pub fn move_to(&self, to: &FluentFile) -> Result<FluentFile, FileMoveToError> {
        to.parent().ensure_exists();
        fs::rename(self.utf8_path_buf(), to.utf8_path_buf())
            .map(|_| to.clone())
            .map_err(|io_error| FileMoveToError {
                from: self.utf8_path_buf(),
                to: to.utf8_path_buf(),
                io_error,
            })
    }

    /// Gets the metadata for the file
    /// This is a similar to [`std::fs::metadata`] returning [`std::fs::Metadata`], except
    /// this method returns [`FileMetadata`] instead, which uses [`jiff::Timestamp`] instead
    /// of [`SystemTime`]
    pub fn metadata(&self) -> Result<FileMetadata, FileMetaDataError> {
        let metadata =
            self.utf8_path_buf()
                .metadata()
                .map_err(|io_error| FileMetaDataError {
                    path: self.utf8_path_buf(),
                    io_error,
                })?;

        let modified_systime = metadata.modified().unwrap_or_else(|cause| {
            panic!("File {self} has MetaData but could not access modified time due to IO Error: {cause:#?}.");
        });

        let modified =
            file_system_time_to_timestamp(self.to_string(), "modified", modified_systime)
                .unwrap_or_else(|| panic!("File at '{self}' must have modified time!"));

        let accessed_systime = metadata.accessed().unwrap_or_else(|cause| {
            panic!("File {self} has MetaData but could not access accessed time due to IO Error: {cause:#?}.");
        });

        let accessed =
            file_system_time_to_timestamp(self.to_string(), "accessed", accessed_systime);

        let created_systime = metadata.created().unwrap_or_else(|cause| {
            panic!("File {self} has MetaData but could not access created time due to IO Error: {cause:#?}.");
        });

        let created =
            file_system_time_to_timestamp(self.to_string(), "created", created_systime);

        Ok(FileMetadata {
            accessed,
            bytes: metadata.len(),
            created,
            modified,
        })
    }

    /// Writes data (String or bytes or anything that can be converted to bytes) to this file
    /// Before writing, ensures the file's folder exists
    pub fn write_data(&self, data: impl AsRef<[u8]>) -> Result<(), FileWriteDataError> {
        self.parent().ensure_exists();
        fs::write(self.utf8_path_buf(), data).map_err(|io_error| FileWriteDataError {
            path: self.utf8_path_buf(),
            io_error,
        })
    }
}

impl Display for FluentFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

    fn utf8_path_buf(&self) -> Utf8PathBuf {
        self.folder.utf8_path_buf().join(self.name_ext())
    }

    fn exists(&self) -> bool {
        self.utf8_path_buf().exists()
    }

    fn with_name(&self, other_name: impl AsRef<str>) -> Self {
        FluentFile::new(self.folder.clone(), other_name.as_ref(), self.ext.clone())
    }

    fn with_folder(&self, folder: impl Into<Folder>) -> Self {
        FluentFile::new(folder.into(), self.name.clone(), self.ext.clone())
    }

    fn remove(self) -> Result<(), FileRemoveError> {
        fs::remove_file(self.utf8_path_buf()).map_err(|io_error| FileRemoveError {
            path: self.utf8_path_buf(),
            io_error,
        })
    }
}

#[derive(Debug)]
pub struct FileMetadata {
    pub accessed: Option<Timestamp>,
    pub bytes: u64,
    pub created: Option<Timestamp>,
    pub modified: Timestamp,
}

fn file_system_time_to_timestamp(
    path: impl AsRef<str>,
    label: &str,
    sys_time: SystemTime,
) -> Option<Timestamp> {
    let nanos_offset_from_epoch = sys_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|dur| dur.as_nanos() as i128)
        // If have an error, then the SystemTime was before UNIX_EPOCH
        // Flip the duration nanos to be negative to indicate this
        .unwrap_or_else(|sys_time_err| -(sys_time_err.duration().as_nanos() as i128));
    if nanos_offset_from_epoch == 0 {
        return None;
    }

    Some(Timestamp::from_nanosecond(nanos_offset_from_epoch).unwrap_or_else(|jiff_err| {
        let path = path.as_ref();
        let before_after = if nanos_offset_from_epoch.is_negative() {
            "before"
        } else {
            "after"
        };
        eprintln!(
            "File {path} had {label} SystemTime of {nanos_offset_from_epoch} nanos {before_after} unix epoch",
        );
        let jiff_err_type = if jiff_err.is_range() {
            "Range"
        } else if jiff_err.is_invalid_parameter() {
            "Invalid Parameter"
        } else {
            "Crate Feature"
        };
        eprintln!(" Caused jiff {} Error {:?} ", jiff_err_type, jiff_err);
        let ts_min = Timestamp::MIN.as_nanosecond();
        let ts_max = Timestamp::MAX.as_nanosecond();
        if nanos_offset_from_epoch < ts_min {
            eprintln!("  This value was less than jiff::Timestamp::MIN");
            eprintln!("    As Date:  -009999-01-02");
            eprintln!("    As Nanos: {ts_min}");
        }
        if nanos_offset_from_epoch > ts_max {
            eprintln!("  This value was greater than jiff::Timestamp::MAX");
            eprintln!("    As Date:  9999-12-30");
            eprintln!("    As Nanos: {ts_max}");
        }
        panic!("Must be able to convert a SystemTime into a Timestamp");
    }))
}
