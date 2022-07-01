use std::io::{self, Write};

pub(crate) trait Printer {
    fn write(&self, writer: &mut dyn Write, key: &str, value: &str) -> io::Result<()>;
}
