use crate::folder::Folder;

pub trait FileActions {
    fn name(&self) -> String;
    fn name_ext(&self) -> String;
    fn parent(&self) -> Folder;
}

#[cfg(test)]
mod tests {
    use crate::file::*;
    use crate::folder::*;

    #[test]
    fn test_file_name_ext() {
        let f = FluentFile::new(&Folder::current(), "Cargo", Some("toml"));
        let name = f.name();
        let ext = f.ext().unwrap_or("NO_EXT".into());
        let name_ext = f.name_ext();
        println!("File: {}", f);
        println!("  name:     {}", name);
        println!("  ext:      {}", ext);
        println!("  name_ext: {}", name_ext);

        assert_eq!(format!("{}.{}", name, ext), name_ext)
    }
}
