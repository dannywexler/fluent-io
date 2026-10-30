use crate::command::command_structs::{CommandInput, CommandOutput};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CommandReadError {
    pub input: CommandInput,
    pub output: CommandOutput,
    pub cause: CommandExit,
}

impl Display for CommandReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            r#"Attempted to execute command. {}
  Input:
    Command: {}
    Args:    {:?}
    Folder:  {:?}
  Output:
    Stdout:  {}
    Stderr:  {}
"#,
            self.cause,
            self.input.cmd.to_string_lossy(),
            self.input.args,
            self.input.folder,
            self.output.stdout,
            self.output.stderr
        )
    }
}

#[derive(Debug)]
pub struct CommandRunError {
    pub input: CommandInput,
    pub cause: CommandExit,
}

impl Display for CommandRunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            r#"Attempted to execute command. {}
  Command: {}
  Args:    {:?}
  Folder:  {:?}
"#,
            self.cause,
            self.input.cmd.to_string_lossy(),
            self.input.args,
            self.input.folder,
        )
    }
}

#[derive(Debug)]
pub enum CommandExit {
    ExitCode(i32),
    Signal,
}

impl Display for CommandExit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandExit::ExitCode(exit_code) => write!(f, "Got exit code {}", exit_code),
            CommandExit::Signal => write!(f, "Terminated by signal"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::FluentCommand;

    #[test]
    fn test_read_command() {
        let cmd = "echo";
        let arg = "hello";
        println!("Running valid command {cmd:?} {arg}");
        let fcmd = FluentCommand::find(cmd);
        assert!(fcmd.is_some());
        let command_output = fcmd.clone()
            .expect(&format!("Must be able to find valid cmd: {cmd}"))
            .add_arg(arg)
            .read()
            .unwrap_or_else(|fcmd_error| panic!("Must be able to execute {cmd:?} with args {arg}. Got fcmd_error: {fcmd_error:?}"));
        println!("CommandOutput: {command_output:#?}");
        println!("{fcmd:#?}");
    }

    #[test]
    fn test_read_invalid_command() {
        let cmd = "fdsa";
        let arg = "hello";
        println!("Attempting to running invalid command {cmd:?} {arg}");
        assert!(FluentCommand::find(cmd).is_none())
    }
}
