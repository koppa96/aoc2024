mod day1;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "aoc")]
#[command(about = "Solves tasks for advent of code 2024.")]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  #[command(about = "Commands for solving tasks for day 1.")]
  Day1(day1::Arguments),
}

fn main() {
  let args = Cli::parse();

  let result = match args.command {
    Commands::Day1(day) => day1::match_task(day),
  };

  utils::handle_error(result);
}
