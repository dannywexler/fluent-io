//! Helpers for interacting with Folders.
//!
//! Existing folder io methods are spread out between methods on [`std::path::Path`], and
//! free-functions in [`std::fs`]. The goal of [`Folder`] is to provide helpful methods
//! specific to interacting with Folders

mod folder_actions;
mod folder_struct;
pub use folder_actions::*;
pub use folder_struct::*;
