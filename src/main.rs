mod day1;
mod day2;
mod day3;
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
  #[command(about = "Commands for solving tasks for day 2.")]
  Day2(day2::Arguments),
  #[command(about = "Commands for solving tasks for day 3.")]
  Day3(day3::Arguments),
}

fn main() {
  let args = Cli::parse();

  let result = match args.command {
    Commands::Day1(day) => day1::match_task(day),
    Commands::Day2(day) => day2::match_task(day),
    Commands::Day3(day) => day3::match_task(day),
  };

  utils::handle_error(result);
}
