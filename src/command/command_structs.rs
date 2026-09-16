use std::process::Command;

use which::CanonicalPath;

use crate::command::command_errors::CommandReadError;

pub struct FluentCommand {
    cmd: CanonicalPath,
    args: Vec<String>,
}

impl FluentCommand {
    pub fn new(command: CanonicalPath) -> Self {
        FluentCommand {
            cmd: command,
            args: Vec::new(),
        }
    }

    pub fn arg(&mut self, arg: impl AsRef<str>) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    pub fn args(&mut self, args: impl IntoIterator<Item = impl AsRef<str>>) -> &mut Self {
        for new_arg in args {
            self.arg(new_arg);
        }
        self
    }

    pub fn read(&mut self) -> Result<CommandOutput, CommandReadError> {
        let input = CommandInput {
            cmd: self.cmd.clone(),
            args: self.args.clone(),
        };

        let og_output = Command::new(self.cmd.as_os_str())
            .args(self.args.clone())
            .output()
            .map_err(|io_error| CommandReadError::Io {
                input: input.clone(),
                io_error,
            })?;

        let stdout_str = String::from_utf8_lossy(og_output.stdout.as_slice())
            .trim_end()
            .to_string();

        let stderr_str = String::from_utf8_lossy(og_output.stderr.as_slice())
            .trim_end()
            .to_string();

        let cmd_output = CommandOutput {
            stdout: stdout_str,
            stderr: stderr_str,
        };

        match og_output.status.code() {
            Some(exit_code) => match exit_code {
                0 => Ok(cmd_output),
                _ => Err(CommandReadError::ExitCode {
                    input,
                    output: cmd_output,
                    exit_code,
                }),
            },
            None => Err(CommandReadError::Signal {
                input,
                output: cmd_output,
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandInput {
    pub cmd: CanonicalPath,
    pub args: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
}
