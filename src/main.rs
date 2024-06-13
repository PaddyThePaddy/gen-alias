use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use alias::Alias;
use anyhow::{Context, Result as AnyResult};
use clap::Parser;

mod alias;

#[derive(Clone, Debug, clap::ValueEnum, Copy, PartialEq, Eq)]
enum SupportedShells {
    Pwsh,
    Bash,
    Fish,
}

#[derive(clap::Parser, Debug, Clone)]
struct Cli {
    alias: PathBuf,
    #[arg(value_enum)]
    shell: SupportedShells,
}

fn main() -> AnyResult<()> {
    let cli = Cli::parse();
    let reader = BufReader::new(
        std::fs::File::open(&cli.alias)
            .with_context(|| format!("open alias file {} failed", cli.alias.display()))?,
    );
    for (idx, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("Read string failed at line {idx}"))?;
        let alias = Alias::from_str(line.as_str())
            .with_context(|| format!("Parse alias from \"{}\" failed", line))?;
        alias.to_script(cli.shell).inspect(|cmd| println!("{cmd}"));
    }
    Ok(())
}
