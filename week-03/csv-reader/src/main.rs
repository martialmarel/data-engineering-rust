use clap::Parser;
use comfy_table::Table;
use std::{error::Error, path::Path, process};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the CSV file to read
    #[arg(value_name = "CSV filepath")]
    csv_filepath: String,

    /// The delimiter used in the CSV file
    #[arg(short, long, default_value = "';'")]
    delimiter: char,

    /// Whether the CSV file has headers
    #[arg(short, long, action = clap::ArgAction::Set, default_value = "true")]
    with_headers: String,
}

fn read_csvfile(
    filepath: String,
    delimiter: char,
    with_headers: bool,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .from_path(filepath)?;
    let mut table = Table::new();

    if with_headers && rdr.headers().is_ok() {
        table.set_header(rdr.headers()?.iter());
    }

    for result in rdr.records() {
        let record = result?;
        table.add_row(&record);
    }

    println!("{table}");
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let csv_filepath = cli.csv_filepath;
    println!("Processing CSV: {}", csv_filepath);

    if !Path::new(&csv_filepath).exists() {
        panic!("File does not exist: {}", csv_filepath);
    }

    let csv_with_headers = matches!(cli.with_headers.as_str(), "yes" | "true" | "1");

    if let Err(err) = read_csvfile(csv_filepath, cli.delimiter, csv_with_headers) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
