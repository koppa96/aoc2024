use crate::day17::common::Program;
use crate::utils;
use std::ops::Range;

pub fn solve(input_path: String) -> utils::Result {
  let program = Program::parse(input_path)?;
  let result = traverse(program, 0..8, 1)?;

  println!("{result}");

  Ok(())
}

fn traverse(mut program: Program, r: Range<i64>, len: usize) -> Result<i64, String> {
  for i in r {
    program.reset(i, 0, 0);
    program.run()?;
    if slice_equals(program.output.as_slice(), program.instructions.as_slice()) {
      return Ok(i);
    }

    let slice_start = program.instructions.len() - len;
    let instruction_slice = &program.instructions[slice_start..program.instructions.len()];
    if slice_equals(program.output.as_slice(), instruction_slice) {
      if let Ok(result) = traverse(program.clone(), i * 8..(i + 1) * 8, len + 1) {
        return Ok(result);
      }
    }
  }

  Err("Not found".to_string())
}

fn slice_equals(s1: &[u8], s2: &[u8]) -> bool {
  if s1.len() != s2.len() {
    return false;
  }

  for i in 0..s1.len() {
    if s1[i] != s2[i] {
      return false;
    }
  }

  true
}
