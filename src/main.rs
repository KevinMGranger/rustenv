mod common;
mod dotenv;
mod fish;

// all target-specific types will already be case-transformed, but _not_ escaped.

use change_case::constant_case;
use clap::{
    builder::{EnumValueParser, ValueParser},
    App, Arg, ArgMatches, Command, ValueEnum, ValueHint,
};
use common::Printer;
use is_terminal::IsTerminal;
use std::collections::HashMap;
use std::env;
use std::io;
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Copy)]
enum Format {
    Fish,
    #[clap(alias("dotenv"))]
    DotEnv,
}

impl Format {
    fn make_printer(self, matches: &ArgMatches) -> anyhow::Result<Box<dyn Printer>> {
        Ok(match self {
            Format::Fish => Box::new(fish::Printer::from_args(matches)?),
            Format::DotEnv => Box::new(dotenv::Printer),
        })
    }
}

// region programargs
struct ProgramArgs {
    file: PathBuf,
    case: bool,
    printer: Box<dyn Printer>,
}

impl TryFrom<&ArgMatches> for ProgramArgs {
    type Error = anyhow::Error;
    fn try_from(matches: &ArgMatches) -> anyhow::Result<Self, Self::Error> {
        let file = matches.get_one::<PathBuf>("file").unwrap().clone();

        // if matches.is_present("dotenv") {
        //     return Ok(ProgramArgs::EvalDotenv(file));
        // }

        let format = if let Some(format) = matches.get_one::<Format>("format") {
            *format
        } else if !std::io::stdout().is_terminal() {
            Format::DotEnv
        } else if env::var_os("SHELL")
            .map(PathBuf::from)
            .filter(|p| fish::shell_is_fish(&p))
            .is_some()
        {
            Format::Fish
        } else {
            anyhow::bail!("Can't determine what output format, please use --format")
        };

        let case = !matches.is_present("no-case");

        Ok(Self {
            file,
            case,
            printer: format.make_printer(&matches)?,
        })
    }
}

impl ProgramArgs {
    fn run(self) -> anyhow::Result<()> {
        // ProgramArgs::EvalDotenv(file) => {
        //     for item in dotenvy::from_path_iter(file)? {
        //         let (key, val) = item?;
        //         dbg!(key);
        //         dbg!(val);
        //     }
        //     return Ok(())
        // }
        let vars: HashMap<String, String> = serde_dhall::from_file(self.file).parse()?;
        let mut stdout = io::stdout().lock();

        let vars: Box<dyn Iterator<Item = (String, String)>> = if self.case {
            Box::new(vars.into_iter().map(|(k, v)| (constant_case(&k), v)))
        } else {
            Box::new(vars.into_iter())
        };

        for (key, val) in vars {
            self.printer
                .write(&mut stdout, &constant_case(&key), &val)?;
        }

        Ok(())
    }
}

// endregion
fn make_app() -> App<'static> {
    let file = Arg::new("file")
        .help("What dhall file to read")
        .takes_value(true)
        .required(true)
        .value_parser(ValueParser::path_buf())
        .value_hint(ValueHint::FilePath);
    let format = Arg::new("format")
        .long("format")
        .short('f')
        .help("The output format. Defaults to dotenv if output is not a tty, fish if running in fish, and otherwise errors if unspecified.")
        .takes_value(true)
        .value_parser(EnumValueParser::<Format>::new());

    let no_case = Arg::new("no-case")
        .help("Don't do the default transformation to constant_case")
        .long("no-case")
        .short('c')
        .takes_value(false);

    let set_flags = Arg::new("fish set flags")
        .long("fish-flags")
        .help("Which flags to append to fish's `set`. Defaults to `-x`. Pass an empty string to not use `-x`.")
        .takes_value(true)
        .default_value("-x");

    // // TODO mutual exclusion. But also it's just for debugging so idk?
    // let run_dotenv = Arg::new("dotenv")
    //     .help("Don't evaluate dhall, just dump values from a dotenv file.")
    //     .long("dotenv");

    let cmd = Command::new("dhallenv")
        .about("Translates dhall into a dotenv file or something evaluable by the fish shell.")
        .arg_required_else_help(true)
        .args(&[file, format, no_case, set_flags]);

    cmd
}
fn main() -> anyhow::Result<()> {
    let matches = make_app().get_matches();
    let program = ProgramArgs::try_from(&matches)?;
    program.run()
}
