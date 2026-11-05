use anyhow::Result;
use strum::{IntoStaticStr, VariantNames};
use tracing::instrument;

use crate::{
    User,
    error::ParseError,
    statement::{
        Statement::{Insert, Select},
        insert::InsertStatementExecutable,
        select::SelectStatementExecutable,
    },
};

mod insert;
mod select;

#[derive(Debug)]
pub enum QueryResult {
    Success,
    Rows(Vec<User>),
    Count(usize),
}

pub trait ExecutableStatement {
    fn execute(&self) -> Result<QueryResult>;
}

#[derive(IntoStaticStr, VariantNames, Debug)]
pub enum Statement {
    Insert(InsertStatementExecutable),
    Select(SelectStatementExecutable),
}

impl Statement {
    #[instrument(ret, err)]
    pub fn parse(input: &str) -> Result<Statement> {
        let (statement_type, args) =
            input
                .split_once(':')
                .ok_or(ParseError::InvalidCommandSyntax {
                    query: input.to_string(),
                })?;
        let args = args.split_whitespace().collect::<Vec<&str>>();

        match statement_type {
            "insert" => Ok(Insert(insert::parse(&args)?)),
            "select" => Ok(Select(select::parse(&args)?)),
            other => Err(ParseError::UnknownCommand {
                allowed_commands: Statement::VARIANTS,
                command: other.to_string(),
            }
            .into()),
        }
    }

    #[instrument(ret, err)]
    pub fn execute(self) -> Result<QueryResult> {
        match self {
            Insert(insert_statement_executable) => insert_statement_executable.execute(),
            Select(select_statement_executable) => select_statement_executable.execute(),
        }
    }
}
