use crate::command::command_structs::{CommandInput, CommandOutput};
use std::{
    fmt::{self, Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum CommandReadError {
    Io {
        input: CommandInput,
        io_error: io::Error,
    },
    ExitCode {
        input: CommandInput,
        output: CommandOutput,
        exit_code: i32,
    },
    Signal {
        input: CommandInput,
        output: CommandOutput,
    },
}

impl Display for CommandReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            CommandReadError::Io { input, io_error } => {
                writeln!(
                    f,
                    "Attempted to run cmd: {} with args: {:?}",
                    input.cmd.to_string_lossy(),
                    input.args
                )?;
                writeln!(f, "Got Io Error: {}", io_error)
            }
            CommandReadError::ExitCode {
                input,
                output,
                exit_code,
            } => {
                writeln!(
                    f,
                    "Attempted to run cmd: {} with args: {:?}",
                    input.cmd.to_string_lossy(),
                    input.args
                )?;
                writeln!(f, "Got ExitCode: {}", exit_code)?;
                writeln!(f, "Got stdout: {}", output.stdout)?;
                writeln!(f, "Got stderr: {}", output.stderr)
            }
            CommandReadError::Signal { input, output } => {
                writeln!(
                    f,
                    "Attempted to run cmd: {} with args: {:?}",
                    input.cmd.to_string_lossy(),
                    input.args
                )?;
                writeln!(f, "Was terminated by signal.")?;
                writeln!(f, "Got stdout: {}", output.stdout)?;
                writeln!(f, "Got stderr: {}", output.stderr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use which::CanonicalPath;

    use crate::command::FluentCommand;

    #[test]
    fn test_read_command() {
        let cmd = CanonicalPath::new("echo").unwrap();
        let arg = "hello";
        println!("Running valid command {cmd:?} {arg}");
        let fcmd = FluentCommand::new(cmd.clone()).arg(arg).read().unwrap_or_else(|fcmd_error| panic!("Must be able to execute {cmd:?} with args {arg}. Got fcmd_error: {fcmd_error:?}"));
        println!("Got stdout: {}", fcmd.stdout);
        println!("Got stderr: {}", fcmd.stderr);
    }
}
