//! Helpers for finding and executing commands
//!
//! [`FluentCommand`] is similar in purpose to [`std::process::Command`] but with a few key
//! differences.
//!
//! [`std::process::Command`] combines the finding of the binary and running of the command into
//! one overall step step. This means that every command execution method like [`std::process::Command::status`]
//! counts the command failing to launch as an Error, and anything that happened after successfully
//! launching the command as Ok. This includes the command exiting with a non-zero exit code.
//!
//! By contrast, [`FluentCommand`] separates the finding of the binary and running of the command
//! into two separate steps. [`FluentCommand::find`] trys to find the binary using [`which::which`].
//! Only if the binary is found and is executable is a FluentCommand instance returned. If the
//! binary is not found, `None` is returned. This way when trying to execute a command using `read`
//! or `run`, the success or failure of those methods are determined directly by the success or
//! failure of the command itself.
//!
//! Regarding success vs. failure of commands, `FluentCommand` treats a 0 exit code as  successful
//! and a non-zero exit code or a signal termination as failure. See
//! [`CommandExit`].

mod command_errors;
mod command_structs;
mod ffmpeg;
mod ffprobe;
pub use command_errors::*;
pub use command_structs::*;
pub use ffmpeg::*;
pub use ffprobe::*;
