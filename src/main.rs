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
    #[arg(long, short)]
    check_for_override: bool,
}

fn main() -> AnyResult<()> {
    let cli = Cli::parse();
    let reader = BufReader::new(
        std::fs::File::open(&cli.alias)
            .with_context(|| format!("open alias file {} failed", cli.alias.display()))?,
    );
    for (idx, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("Read string failed at line {idx}"))?;
        if line.trim().is_empty() {
            continue;
        }
        let alias = Alias::from_str(line.as_str())
            .with_context(|| format!("Parse alias from \"{line}\" failed"))?;
        if cli.check_for_override {
            if alias.supports(cli.shell) && check_cmd_exist(alias.name(), cli.shell)? {
                eprintln!("Command {} already defined", alias.name())
            }
        } else {
            alias.to_script(cli.shell).inspect(|cmd| println!("{cmd}"));
        }
    }
    Ok(())
}

fn check_cmd_exist(name: &str, shell: SupportedShells) -> AnyResult<bool> {
    Ok(match shell {
        SupportedShells::Pwsh => std::process::Command::new("pwsh")
            .arg("--")
            .arg("get-command")
            .arg(name)
            .arg("-ErrorAction")
            .arg("SilentlyContinue")
            .status()?
            .success(),
        SupportedShells::Bash => std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("command -v {name}"))
            .status()?
            .success(),
        SupportedShells::Fish => std::process::Command::new("fish")
            .arg("-c")
            .arg(format!("type -q {name}"))
            .status()?
            .success(),
    })
}
