//command-line tool that reads a CSV file and prints the contents of the file as a DataFrame
mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use polars::prelude::*;
use using_polars_dataframe_cli as lib;

fn main() {
    std::env::set_var("POLARS_FMT_MAX_ROWS", "-1");

    let args = Cli::parse();
    match args.command {
        Some(Commands::Print { path, rows }) => {
            let df = lib::read_csv(&path);
            println!("{:?}", df.head(Some(rows)));
        }
        Some(Commands::Describe { path }) => {
            let df = lib::read_csv(&path);
            println!("{:?}", df);
        }
        Some(Commands::Schema { path }) => {
            let df = lib::read_csv(&path);
            println!("{:?}", df.schema());
        }
        Some(Commands::Shape { path }) => {
            let df = lib::read_csv(&path);
            println!("{:?}", df.shape());
        }
        Some(Commands::Sort {
            path,
            year,
            rows,
            order,
        }) => {
            let df = lib::read_csv(&path);
            let country_column_name = "Country Name";
            //select the country column and the year string passed in and return a new dataframe
            let vs = df.select_series([country_column_name, &year]).unwrap();
            //convert the Vec<Series> to a DataFrame
            let df2 = DataFrame::new(vs).unwrap();
            //drop any rows with null values and return a new dataframe
            let df2: DataFrame = df2.drop_nulls::<String>(None).unwrap();
            //sort the dataframe by the year column and by order passed in
            let df2 = df2.sort([&year], order, true).unwrap();

            //print the first "rows" of the dataframe
            println!("{:?}", df2.head(Some(rows)));
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
