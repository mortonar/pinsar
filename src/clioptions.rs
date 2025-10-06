use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufReader, Error, Read};
use std::path::PathBuf;

/// Command line options
#[derive(Parser, Debug, Clone, Default)]
#[command(author, version, about = "A graphing tool for Linux SAR data")]
pub struct CliOptions {
    /// File to read - read from stdin if not provided
    #[arg(short, long)]
    pub in_file: Option<PathBuf>,

    /// Directory to write output files
    #[arg(short, long, default_value = "./images/")]
    pub output_dir: PathBuf,
}

impl CliOptions {
    pub fn input_stream(&self) -> Result<Box<dyn Read>, Error> {
        match &self.in_file {
            Some(file) => {
                dbg!(format!("Reading file: {}", file.display()));
                let file = File::open(file)?;
                Ok(Box::new(BufReader::new(file)))
            }
            None => {
                dbg!("No file provided. Using stdin.");
                Ok(Box::new(BufReader::new(io::stdin())))
            }
        }
    }
}
