mod folder_actions;
mod folder_struct;
pub use folder_actions::*;
pub use folder_struct::*;

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
    fn test_name() {
        let current = Folder::current();
        let name = current.name();
        println!("current folder: {current}");
        println!("current folder name: {name}");
        assert!(current.to_string().ends_with(&name));
    }
}
