use crate::folder::FolderActions;

pub trait FileActions {
    fn name(&self) -> String;
    fn name_ext(&self) -> String;
    fn parent(&self) -> impl FolderActions;
}
