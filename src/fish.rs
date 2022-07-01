use std::io::{self, Write};
use std::path::Path;

pub(crate) fn shell_is_fish(shell_path: &Path) -> bool {
    shell_path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| *name == "fish")
        .is_some()
}

pub(crate) struct Printer {
    pub(crate) set_flags: String,
}

impl Printer {
    pub(crate) fn from_args(matches: &clap::ArgMatches) -> anyhow::Result<Self> {
        let set_flags = matches.get_one::<String>("fish set flags").unwrap().clone();
        Ok(Self { set_flags })
    }
}

impl crate::common::Printer for Printer {
    fn write(&self, writer: &mut dyn Write, key: &str, val: &str) -> io::Result<()> {
        let escaped = val.replace('\\', r"\\").replace('\'', r"\'");
        let set_flags = &self.set_flags;
        writeln!(
            writer,
            "set {set_flags} {key} (string escape '{escaped}' | string unescape | string collect)"
        )?;

        Ok(())
    }
}
