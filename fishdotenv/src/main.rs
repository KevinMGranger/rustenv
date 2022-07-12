use anyhow::Result;
use clap::Parser;
use is_terminal::IsTerminal;
use std::io;

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
    #[clap(short('s'), long, default_value="-x")]
    /// Which flags to append to fish's `set`.
    /// Pass an empty string to not use the default of `-x`.
    set_flags: String,
}

fn main() -> Result<()> {
    let Args { set_flags } = Args::parse();

    let vars = if io::stdin().is_terminal() {
        dotenvy::dotenv_iter()?
    } else {
        dotenvy::from_path_iter("/dev/stdin")?
    };

    for item in vars {
        let (key, val) = item?;

        let escaped = val.replace('\\', r"\\").replace('\'', r"\'");
        // TODO: document why the escape-unescape dance is necessary.
        // Also test that this actually works.
        println!(
            "set {set_flags} {key} (string escape '{escaped}' | string unescape | string collect)"
        );
    }

    Ok(())
}
