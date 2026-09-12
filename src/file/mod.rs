mod file_actions;
mod file_struct;
pub use file_actions::*;
pub use file_struct::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::folder::*;

    #[test]
    fn test_file_name_ext() {
        let f = FluentFile::new(Folder::current(), "Cargo", Some("toml"));
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
