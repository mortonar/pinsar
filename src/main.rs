mod clioptions;
mod data;

use crate::clioptions::CliOptions;
use chrono::Local;
use clap::Parser;

fn main() -> std::io::Result<()> {
    let opts = CliOptions::parse();
    let stream = opts.input_stream()?;

    let parse_start = Local::now();
    let sar_data: data::SarData = serde_json::from_reader(stream)?;
    let duration = Local::now() - parse_start;
    println!("Finished parsing JSON: {}ms", duration.num_milliseconds());
    println!("{:#?}", sar_data);

    Ok(())
}
