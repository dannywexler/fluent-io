//! Helpers for interacting with Files.
//!
//! Existing file io methods are spread out between methods on [`std::fs::File`], methods on
//! [`std::path::Path`], and free-functions in [`std::fs`]. The goal of [`FluentFile`] is to
//! provide helpful methods specific to interacting with a File.

mod file_actions;
mod file_errors;
mod file_structs;
mod image;
mod video;
pub use file_actions::*;
pub use file_errors::*;
pub use file_structs::*;
pub use image::*;
pub use video::*;
