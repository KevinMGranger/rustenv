mod janky_ordered;
use anyhow::{bail, Result};
use clap::Parser;
use is_terminal::IsTerminal;
use std::{env, io};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Print a dotenv file into a format that the fish shell can `eval`.
///
///
/// If stdin is not a terminal, it will read from the stdin stream.
/// If stdin is a terminal, it will read from .env.
///
/// To read from a different file, use shell redirection.
/// To read from .env when stdin is inherited from a parent,
/// redirect stdin to /dev/null.
///
/// Error codes: 3 if the .env file was not found, 4 if check failed.
struct Args {
    #[clap(short = 's', long, default_value = "--export --global")]
    /// Which flags to append to fish's `set`.
    /// Pass an empty string to not use the default.
    set_flags: String,

    // TODO: make certain flags mutually exclusive?
    // TODO: want to get rid of duplicates, but maybe a "raw" option that just lists them as lines? like -l versus -L ?
    #[clap(short = 'l', long)]
    /// List the environment variables that should be set.
    list: bool,

    #[clap(short = 'q', long, default_value = "")]
    /// Get the value of the given environment variable, if the dotenv file were applied.
    query: String,

    #[clap(short = 'c', long)]
    /// Check if the current env lines up with what the dotenv file prescribes.
    check: bool,
    // #[clap(short = 'e', long)]
    // /// Evaluate the dotenv file, printing what the substituted, deduplicated results would be.
    // eval: bool,
}

fn list(vars: impl Iterator<Item = dotenvy::Result<(String, String)>>) -> Result<()> {
    let keys: Vec<String> = vars
        .map(|r| r.map(|(k, _)| k))
        .collect::<dotenvy::Result<Vec<String>>>()?;

    let keys = keys.into_iter().rev().collect::<janky_ordered::Set>();

    for key in keys.into_iter().rev() {
        println!("{key}");
    }

    Ok(())
}

fn query(
    vars: impl Iterator<Item = dotenvy::Result<(String, String)>>,
    query: String,
) -> Result<()> {
    let keys = vars
        .filter_map(|r| match r {
            Err(e) => Some(Err(e)),
            Ok((key, value)) => {
                if key == query {
                    Some(Ok(value))
                } else {
                    None
                }
            }
        })
        .collect::<dotenvy::Result<Vec<String>>>()?;
    if let Some(val) = keys.last() {
        println!("{val}");
    } else {
        bail!("{query} not found");
    }
    Ok(())
}
fn check(vars: impl Iterator<Item = dotenvy::Result<(String, String)>>) -> Result<()> {
    let keys = vars.collect::<dotenvy::Result<Vec<(String, String)>>>()?;

    let map = keys.into_iter().rev().collect::<janky_ordered::Map>();

    let mismatches = map
        .into_iter()
        .rev()
        .filter_map(|(k, v)| match env::var(&k) {
            Ok(actual) => {
                if *v != actual {
                    Some((k, v, Some(actual)))
                } else {
                    None
                }
            }
            Err(e) => match e {
                env::VarError::NotPresent => Some((k, v, None)),
                env::VarError::NotUnicode(_) => panic!("{e}"),
            },
        });

    let mut any = false;
    for (k, v, actual) in mismatches {
        println!(
            "{k} is {}, not {v}",
            actual.as_ref().map(String::as_str).unwrap_or("unset")
        );
        any = true;
    }
    if any {
        std::process::exit(4);
    }
    Ok(())
}

fn fish(
    vars: impl Iterator<Item = dotenvy::Result<(String, String)>>,
    set_flags: String,
) -> Result<()> {
    let items = vars.collect::<dotenvy::Result<Vec<(String, String)>>>()?;
    let map = items.into_iter().rev().collect::<janky_ordered::Map>();
    for (key, val) in map.into_iter().rev() {
        let escaped = val.replace('\\', r"\\").replace('\'', r"\'");
        // TODO: document why the escape-unescape dance is necessary.
        // Also test that this actually works.
        println!(
            "set {set_flags} {key} (string escape '{escaped}' | string unescape | string collect)"
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let vars = if io::stdin().is_terminal() {
        match dotenvy::dotenv_iter() {
            Ok(i) => i,
            Err(dotenvy::Error::Io(ioe)) if ioe.kind() == io::ErrorKind::NotFound => {
                eprintln!(".env not found");
                std::process::exit(3)
            }
            Err(e) => return Err(anyhow::Error::from(e)),
        }
    } else {
        dotenvy::from_path_iter("/dev/stdin")?
    };

    if args.list {
        list(vars)
    } else if !args.query.is_empty() {
        query(vars, args.query)
    } else if args.check {
        check(vars)
    } else {
        fish(vars, args.set_flags)
    }
}
