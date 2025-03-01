use clap::{Parser, Subcommand};
use codecrafters_interpreter::Lexer;
use miette::Context;
use miette::IntoDiagnostic;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize { filename: PathBuf },
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Tokenize { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;

            let lexer = Lexer::new(&file_contents);
            for token in lexer {
                let token = token?;
                println!("{token}");
            }
            println!("EOF  null");
        }
    }
    Ok(())
}
