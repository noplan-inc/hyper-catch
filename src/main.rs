use option::CliOptions;

mod contract;
mod formatter;
mod option;

use clap::Parser;

fn main() {
    let options = CliOptions::parse();
    println!("{:?}", options);
}
