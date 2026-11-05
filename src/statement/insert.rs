use crate::{
    User,
    error::ParseError,
    statement::{ExecutableStatement, QueryResult},
};
use anyhow::Result;

#[derive(Debug)]
pub struct InsertStatementExecutable {
    user: User,
}

impl ExecutableStatement for InsertStatementExecutable {
    fn execute(&self) -> Result<QueryResult> {
        println!("written {:?}", self.user);

        Ok(QueryResult::Success)
    }
}

pub(crate) fn parse(args: &[&str]) -> Result<InsertStatementExecutable> {
    if args.len() != 3 {
        return Err(ParseError::IncorrectNumberOfArgs {
            expected: 3,
            found: args.len(),
        }
        .into());
    }

    Ok(InsertStatementExecutable {
        user: User::new(args[0], args[1], args[2])?,
    })
}
