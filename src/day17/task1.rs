use crate::day17::common::Program;
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let mut program = Program::parse(input_path)?;
  program.run()?;

  let string_vec: Vec<_> = program.output.iter().map(|item| item.to_string()).collect();
  println!("{}", string_vec.join(","));

  Ok(())
}
