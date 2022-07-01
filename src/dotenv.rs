use std::io::{self, Write};

pub(crate) struct Printer;

impl crate::common::Printer for Printer {
    // todo: figuring out the escaping rules is really hard.
    // especially since there's no real standard, and multiple competing implementations.
    // we're just going to avoid escaping and hope the data is simple enough.
    fn write(&self, writer: &mut dyn Write, key: &str, val: &str) -> io::Result<()> {
        writeln!(writer, "{key}={val}")?;

        Ok(())
    }
}
