use crate::statement::{ExecutableStatement, QueryResult};
use anyhow::Result;

#[derive(Debug)]
pub struct SelectStatementExecutable {}

impl ExecutableStatement for SelectStatementExecutable {
    fn execute(&self) -> Result<QueryResult> {
        Ok(QueryResult::Success)
    }
}

pub(crate) fn parse(_args: &[&str]) -> Result<SelectStatementExecutable> {
    Ok(SelectStatementExecutable {})
}
