use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    command::{CommandExit, CommandRunError, command_errors::CommandReadError},
    folder::{Folder, FolderActions},
};

#[derive(Clone, Debug)]
pub struct FluentCommand {
    cmd: PathBuf,
    args: Vec<String>,
    custom_folder: Option<PathBuf>,
}

impl FluentCommand {
    /// Tries to find the executable path for the binary name using [which](https://docs.rs/which/latest/which/fn.which.html)
    pub fn find(binary_name: impl AsRef<Path>) -> Option<Self> {
        which::which(binary_name.as_ref())
            .ok()
            .map(|pathbuf| FluentCommand {
                cmd: pathbuf,
                args: Vec::new(),
                custom_folder: None,
            })
    }

    /// Get the path to the binary
    pub fn cmd(&self) -> PathBuf {
        self.cmd.clone()
    }

    /// Get iterator over the args
    pub fn args(&self) -> impl Iterator<Item = String> {
        self.args.clone().into_iter()
    }

    /// Add an additional arg
    pub fn add_arg(&mut self, arg: impl AsRef<str>) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    /// Add multiple additional args
    pub fn add_args(
        &mut self,
        args: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        for new_arg in args {
            self.add_arg(new_arg);
        }
        self
    }

    /// Get the custom folder to run the command in, if specified by calling `set_folder`
    pub fn folder(&self) -> Option<PathBuf> {
        self.custom_folder.clone()
    }

    /// Set the custom folder to run the command in
    pub fn set_folder(&mut self, new_folder: impl Into<Folder>) -> &mut Self {
        self.custom_folder = Some(new_folder.into().into());
        self
    }

    /// Execute the command, then collect and return its standard out and standard error
    ///
    /// This will not print anything to the console
    pub fn read(&mut self) -> Result<CommandOutput, CommandReadError> {
        let input = CommandInput {
            cmd: self.cmd.clone(),
            args: self.args.clone(),
            folder: self
                .custom_folder
                .clone()
                .unwrap_or(Folder::current().into()),
        };

        let og_output = Command::new(self.cmd.clone())
            .args(self.args.clone())
            .current_dir(input.folder.clone())
            .output()
            .unwrap_or_else(|io_error| {
                panic!("Already checked that the cmd: {:?} is executable so it should not be possible to get an IO error here. Got IO Error: {io_error:#?}", self.cmd);
            });

        let cmd_output = CommandOutput {
            stdout: String::from_utf8_lossy(og_output.stdout.as_slice())
                .trim_end()
                .to_string(),
            stderr: String::from_utf8_lossy(og_output.stderr.as_slice())
                .trim_end()
                .to_string(),
        };

        match og_output.status.code() {
            Some(exit_code) => match exit_code {
                0 => Ok(cmd_output),
                _ => Err(CommandReadError {
                    input,
                    output: cmd_output,
                    cause: CommandExit::ExitCode(exit_code),
                }),
            },
            None => Err(CommandReadError {
                input,
                output: cmd_output,
                cause: CommandExit::Signal,
            }),
        }
    }

    /// Execute the command, streaming its stdout and stderr to the console as the command
    /// runs
    pub fn run(&mut self) -> Result<(), CommandRunError> {
        let input = CommandInput {
            cmd: self.cmd.clone(),
            args: self.args.clone(),
            folder: self
                .custom_folder
                .clone()
                .unwrap_or(Folder::current().into()),
        };

        let exit_status = Command::new(self.cmd.clone())
            .args(self.args.clone())
            .current_dir(input.folder.clone())
            .status()
            .unwrap_or_else(|io_error| panic!("Already checked that the cmd: {:?} is executable so it should not be possible to get an IO error here. Got IO Error: {io_error:#?}", self.cmd));
        match exit_status.code() {
            Some(exit_code) => match exit_code {
                0 => Ok(()),
                _ => Err(CommandRunError {
                    input,
                    cause: CommandExit::ExitCode(exit_code),
                }),
            },
            None => Err(CommandRunError {
                input,
                cause: CommandExit::Signal,
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandInput {
    pub cmd: PathBuf,
    pub args: Vec<String>,
    pub folder: PathBuf,
}

#[derive(Clone, Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
}
