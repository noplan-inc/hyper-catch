use anyhow::Result;
use std::fs::{self};
use std::io;
use std::io::{BufWriter, Write};

pub struct Outputter {
    pub buff_writer: io::BufWriter<fs::File>,
}

impl Outputter {
    pub fn new(file_path: &str) -> Result<Outputter> {
        Ok(Outputter {
            buff_writer: BufWriter::new(fs::File::create(file_path)?),
        })
    }

    pub fn write(&mut self, line: String) -> Result<()> {
        let line_with_break = line + "\n";
        self.buff_writer.write_all(line_with_break.as_bytes())?;

        self.buff_writer.flush()?;

        Ok(())
    }
}
