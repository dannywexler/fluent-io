use std::{fmt::Display, path::Path};

use camino::Utf8PathBuf;

pub trait FolderActions: Clone + Display + PartialEq + Eq {
    fn current() -> Self;
    fn home() -> Self;
    fn temp() -> Self;
    fn exists(&self) -> bool;
    fn name(&self) -> String;
    fn path_buf(&self) -> Utf8PathBuf;
    fn parent(&self) -> Self;
    fn folder(&self, path_segment: impl AsRef<Path>) -> Self;
}

#[cfg(test)]
mod tests {
    use crate::folder::*;

    #[test]
    fn test_home() {
        let home = Folder::home();
        println!("home folder: {home}");
        assert!(home.exists());
    }

    #[test]
    fn test_current() {
        let current = Folder::current();
        println!("current folder: {current}");
        assert!(current.exists());
    }

    #[test]
    fn test_temp() {
        let temp = Folder::temp();
        println!("temp folder: {temp}");
        assert!(temp.exists());
    }

    #[test]
    fn test_name() {
        let current = Folder::current();
        let name = current.name();
        println!("current folder: {current}");
        println!("current folder name: {name}");
        assert!(current.to_string().ends_with(&name));
    }

    #[test]
    fn test_relative_to_absolute() {
        let current = Folder::current();
        let empty = Folder::new("");
        let dot = Folder::new(".");
        println!("current: {current}");
        println!("empty:   {empty}");
        println!("dot:     {dot}");
        assert_eq!(current, empty);
        assert_eq!(empty, dot);
    }

    #[test]
    fn test_parent_and_child() {
        let current = Folder::current();
        let parent = current.parent();
        let current2 = parent.folder(current.name());
        println!("current:  {current}");
        println!("parent:   {parent}");
        println!("current2: {current2}");
        assert_eq!(current, current2);
    }

    #[test]
    fn test_resolve_tilde() {
        let subfolder = "subfolder";
        let h1 = Folder::home().folder(subfolder);
        let tilde = Folder::new(format!("~/{}", subfolder));
        println!("h1:    {}", h1);
        println!("tilde: {}", h1);
        assert_eq!(h1, tilde);
    }
}
