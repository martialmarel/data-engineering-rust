use clap::Parser;

const CSV_FILE: &str = "data/global-life-expt-2022.csv";

#[derive(Parser)]
//add extended help
#[command(
    version = "1.0",
    author = "Noah Gift",
    about = "A command-line tool that reads a CSV file and prints the contents of the file as a DataFrame",
    after_help = "Example: cargo run -- print --rows 3"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser)]
pub enum Commands {
    Print {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
        #[arg(long, default_value = "10")]
        rows: usize,
    },
    Describe {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
    },
    Schema {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
    },
    Shape {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
    },
    Sort {
        #[arg(long, default_value = CSV_FILE)]
        path: String,
        #[arg(long, default_value = "2020")]
        year: String,
        #[arg(long, default_value = "10")]
        rows: usize,
        #[arg(long, default_value = "true")]
        order: bool,
    },
}
