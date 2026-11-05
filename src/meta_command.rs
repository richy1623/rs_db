use anyhow::Result;
use strum::{IntoStaticStr, VariantNames};
use tracing::instrument;

use crate::error::ParseError;

#[derive(IntoStaticStr, VariantNames, Debug)]
pub enum MetaCommand {
    Exit,
}

#[derive(Debug)]
pub enum Flow {
    Continue,
    Exit,
}

impl MetaCommand {
    #[instrument(ret, err)]
    pub fn parse(input: &str) -> Result<MetaCommand> {
        match input {
            ".exit" | ".e" | ".quit" | ".q" => Ok(MetaCommand::Exit),
            other => Err(ParseError::UnknownCommand {
                allowed_commands: MetaCommand::VARIANTS,
                command: other.to_string(),
            }
            .into()),
        }
    }
    #[instrument(ret, err)]
    pub fn execute(self) -> Result<Flow> {
        match self {
            MetaCommand::Exit => Ok(Flow::Exit),
        }
    }
}
