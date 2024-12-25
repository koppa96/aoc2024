mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day17;
mod day18;
mod day19;
mod day2;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
  #[command(about = "Commands for solving tasks for day 4.")]
  Day4(day4::Arguments),
  #[command(about = "Commands for solving tasks for day 5.")]
  Day5(day5::Arguments),
  #[command(about = "Commands for solving tasks for day 6.")]
  Day6(day6::Arguments),
  #[command(about = "Commands for solving tasks for day 7.")]
  Day7(day7::Arguments),
  #[command(about = "Commands for solving tasks for day 8.")]
  Day8(day8::Arguments),
  #[command(about = "Commands for solving tasks for day 9.")]
  Day9(day9::Arguments),
  #[command(about = "Commands for solving tasks for day 10.")]
  Day10(day10::Arguments),
  #[command(about = "Commands for solving tasks for day 11.")]
  Day11(day11::Arguments),
  #[command(about = "Commands for solving tasks for day 12.")]
  Day12(day12::Arguments),
  #[command(about = "Commands for solving tasks for day 13.")]
  Day13(day13::Arguments),
  #[command(about = "Commands for solving tasks for day 14.")]
  Day14(day14::Arguments),
  #[command(about = "Commands for solving tasks for day 17.")]
  Day17(day17::Arguments),
  #[command(about = "Commands for solving tasks for day 18.")]
  Day18(day18::Arguments),
  #[command(about = "Commands for solving tasks for day 19.")]
  Day19(day19::Arguments),
  #[command(about = "Commands for solving tasks for day 21.")]
  Day21(day21::Arguments),
  #[command(about = "Commands for solving tasks for day 22.")]
  Day22(day22::Arguments),
  #[command(about = "Commands for solving tasks for day 23.")]
  Day23(day23::Arguments),
  #[command(about = "Commands for solving tasks for day 24.")]
  Day24(day24::Arguments),
  #[command(about = "Commands for solving tasks for day 25.")]
  Day25(day25::Arguments),
}

fn main() {
  let args = Cli::parse();

  let result = match args.command {
    Commands::Day1(day) => day1::match_task(day),
    Commands::Day2(day) => day2::match_task(day),
    Commands::Day3(day) => day3::match_task(day),
    Commands::Day4(day) => day4::match_task(day),
    Commands::Day5(day) => day5::match_task(day),
    Commands::Day6(day) => day6::match_task(day),
    Commands::Day7(day) => day7::match_task(day),
    Commands::Day8(day) => day8::match_task(day),
    Commands::Day9(day) => day9::match_task(day),
    Commands::Day10(day) => day10::match_task(day),
    Commands::Day11(day) => day11::match_task(day),
    Commands::Day12(day) => day12::match_task(day),
    Commands::Day13(day) => day13::match_task(day),
    Commands::Day14(day) => day14::match_task(day),
    Commands::Day17(day) => day17::match_task(day),
    Commands::Day18(day) => day18::match_task(day),
    Commands::Day19(day) => day19::match_task(day),
    Commands::Day21(day) => day21::match_task(day),
    Commands::Day22(day) => day22::match_task(day),
    Commands::Day23(day) => day23::match_task(day),
    Commands::Day24(day) => day24::match_task(day),
    Commands::Day25(day) => day25::match_task(day),
  };

  utils::handle_error(result);
}
