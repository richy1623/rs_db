use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use crate::Command::{Exit, Unknown};

fn main() -> std::io::Result<ExitCode> {
    let mut stdout = io::stdout();
    let mut stdin_lock = io::stdin().lock();
    let mut buffer = String::new();
    loop {
        write!(stdout, "\nrsdb> ")?;
        stdout.flush()?;
        buffer.clear();

        if stdin_lock.read_line(&mut buffer)? == 0 {
            break;
        }

        match parse_input(&buffer) {
            Command::Exit => break,
            Command::Unknown(command) => write!(stdout, "Unknown command: {}", command)?,
        };
    }

    Ok(ExitCode::SUCCESS)
}

fn parse_input(input: &str) -> Command {
    match input.trim() {
        "exit" | "quit" | "q" => Exit,
        other => Unknown(other.to_string()),
    }
}

enum Command {
    Exit,
    Unknown(String),
}
