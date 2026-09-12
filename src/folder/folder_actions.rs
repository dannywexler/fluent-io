pub trait FolderActions {
    fn current() -> Self;
    fn home() -> Self;
    fn exists(&self) -> bool;
    fn name(&self) -> String;
}
