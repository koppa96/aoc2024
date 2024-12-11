mod task1;
mod task2;

use crate::utils;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(about = "Solves task 1 for day 11.", arg_required_else_help = true)]
  Task1 { input_path: String },
  #[command(about = "Solves task 2 for day 11.", arg_required_else_help = true)]
  Task2 { input_path: String },
}

#[derive(Debug, Args)]
pub struct Arguments {
  #[command(subcommand)]
  command: Commands,
}

pub fn match_task(day: Arguments) -> utils::Result {
  match day.command {
    Commands::Task1 { input_path } => task1::solve(input_path),
    Commands::Task2 { input_path } => task2::solve(input_path),
  }
}
