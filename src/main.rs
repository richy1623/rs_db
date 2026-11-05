use anyhow::Result;
use rs_db::meta_command::{Flow, MetaCommand};
use std::io::{self, BufRead, Write};
use std::process::ExitCode;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use rs_db::statement::Statement;

fn main() -> Result<ExitCode> {
    tracing_subscriber::registry()
        .with(
            fmt::layer().with_span_events(fmt::format::FmtSpan::NEW | fmt::format::FmtSpan::CLOSE),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

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

        let buffer = buffer.trim();

        match handle_line(&mut stdout, buffer) {
            Ok(Flow::Exit) => break,
            Ok(Flow::Continue) => {}
            Err(error) => writeln!(stdout, "Error: {error}")?,
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn handle_line(out: &mut impl Write, line: &str) -> Result<Flow> {
    if line.is_empty() {
        return Ok(Flow::Continue);
    }
    if line.starts_with(".") {
        return MetaCommand::parse(line)?.execute();
    }

    let query_result = Statement::parse(&line)?.execute()?;
    writeln!(out, "{:?}", query_result)?;
    Ok(Flow::Continue)
}
