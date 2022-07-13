mod janky_ordered;
use anyhow::{bail, Context, Error, Result};
use clap::Parser;
use is_terminal::IsTerminal;
use std::{env, io};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Print a dotenv file into a format that the fish shell can `eval`.
///
/// If stdin is not a terminal, it will read from the stdin stream.
/// If stdin is a terminal, it will read from .env.
///
/// To read from a different file, use shell redirection.
/// To read from .env when stdin is inherited from a parent,
/// redirect stdin to /dev/null.
struct Args {
    #[clap(short = 's', long, default_value = "-x")]
    /// Which flags to append to fish's `set`.
    /// Pass an empty string to not use the default of `-x`.
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

fn main() -> Result<()> {
    let Args {
        set_flags,
        list,
        query,
        check,
        // eval,
    } = Args::parse();

    let vars = if io::stdin().is_terminal() {
        match dotenvy::dotenv_iter() {
            Ok(i) => i,
            Err(dotenvy::Error::Io(ioe)) if ioe.kind() == io::ErrorKind::NotFound => {
                bail!("dotenv file not found")
            }
            Err(e) => return Err(anyhow::Error::from(e)),
        }
    } else {
        dotenvy::from_path_iter("/dev/stdin")?
    };

    if list {
        let keys: Vec<String> = vars
            .map(|r| r.map(|(k, _)| k).map_err(Error::from))
            .collect::<Result<Vec<String>>>()?;

        let keys = keys.into_iter().rev().collect::<janky_ordered::Set>();

        for key in keys.into_iter().rev() {
            println!("{key}");
        }
    } else if !query.is_empty() {
        let keys = vars
            .filter_map(|r| match r {
                Err(e) => Some(Err(Error::from(e))),
                Ok((key, value)) => {
                    if key == query {
                        Some(Ok(value))
                    } else {
                        None
                    }
                }
            })
            .collect::<Result<Vec<String>>>()?;
        if let Some(val) = keys.last() {
            println!("{val}");
        } else {
            bail!("{query} not found");
        }
    } else if check {
        let keys = vars
            .map(|r| r.map_err(Error::from))
            .collect::<Result<Vec<(String, String)>>>()?;

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
            std::process::exit(1);
        }
    } else {
        for item in vars {
            let (key, val) = item?;

            let escaped = val.replace('\\', r"\\").replace('\'', r"\'");
            // TODO: document why the escape-unescape dance is necessary.
            // Also test that this actually works.
            println!("set {set_flags} {key} (string escape '{escaped}' | string unescape | string collect)");
        }
    }

    Ok(())
}
