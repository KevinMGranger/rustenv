use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

use change_case::constant_case;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Evaluates a dhall file, and writes it in dotenv format to stdout.
struct Args {
    #[clap()]
    /// What dhall file to read.
    file: PathBuf,

    #[clap(short = 'c', long)]
    /// Don't do the default transformation to constant_case.
    no_case: bool,
}
fn main() -> Result<()> {
    let Args { file, no_case } = Args::parse();

    let vars: HashMap<String, String> = serde_dhall::from_file(file).parse()?;

    // TODO: escape values
    let vars: Box<dyn Iterator<Item = (String, String)>> = if no_case {
        Box::new(vars.into_iter())
    } else {
        Box::new(vars.into_iter().map(|(k, v)| (constant_case(&k), v)))
    };

    for (key, val) in vars {
        println!(r#"{key}="{val}""#);
    }

    Ok(())
}
