use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Incorrect number of args for statement (expected `{expected:?}`, got `{found:?}`)")]
    IncorrectNumberOfArgs { expected: usize, found: usize },
    #[error("Invalid command syntax provided (expected `<command>:<query>`, got `{query}`)")]
    InvalidCommandSyntax { query: String },
    #[error("An unknown command type was used (expected `[allowed_commands]`, got `{command}`)")]
    UnknownCommand {
        allowed_commands: &'static [&'static str],
        command: String,
    },
}
