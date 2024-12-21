use crate::day21::common::{
  do_horizontal_movement, do_vertical_movement, locate_instruction, numbers_to_instructions,
};
use crate::utils;
use crate::utils::read_lines;
use std::collections::HashMap;

pub fn solve(input_path: String) -> utils::Result {
  let mut sum = 0;
  let mut cache = HashMap::new();
  for line in read_lines(input_path)? {
    let number_part = &line[0..line.len() - 1];
    let number = number_part.parse::<usize>()?;

    let nums = line.chars().collect();
    let len = get_instruction_length(nums, &mut cache, 25);
    sum += number * len;
  }

  println!("{}", sum);

  Ok(())
}

fn get_instruction_length(
  nums: Vec<char>,
  cache: &mut HashMap<(usize, char, char), usize>,
  max_depth: usize,
) -> usize {
  let instructions = numbers_to_instructions(nums);
  let mut sum = 0;
  let mut prev = 'A';
  for instruction in instructions {
    sum += get_len(prev, instruction, 1, max_depth, cache);
    prev = instruction;
  }

  sum
}

fn get_len(
  start: char,
  end: char,
  depth: usize,
  max_depth: usize,
  cache: &mut HashMap<(usize, char, char), usize>,
) -> usize {
  if let Some(len) = cache.get(&(depth, start, end)) {
    return *len;
  }

  let instructions = to_instructions(start, end);
  if depth == max_depth {
    cache.insert((depth, start, end), instructions.len());
    return instructions.len();
  }

  let mut prev = 'A';
  let mut sum = 0;
  for instruction in instructions {
    sum += get_len(prev, instruction, depth + 1, max_depth, cache);
    prev = instruction;
  }

  cache.insert((depth, start, end), sum);
  sum
}

fn to_instructions(start: char, end: char) -> Vec<char> {
  let mut instructions = Vec::new();
  let start_pos = locate_instruction(start).unwrap();
  let end_pos = locate_instruction(end).unwrap();
  let movement = (
    end_pos.0 as i32 - start_pos.0 as i32,
    end_pos.1 as i32 - start_pos.1 as i32,
  );

  // If possible move LEFT first
  // Then move UP
  // Then move DOWN
  // Then move RIGHT
  if start_pos.0 == 0 && end_pos.1 == 0 {
    do_vertical_movement(movement.0, &mut instructions);
    do_horizontal_movement(movement.1, &mut instructions);
  } else if start_pos.1 == 0 && end_pos.0 == 0 {
    do_horizontal_movement(movement.1, &mut instructions);
    do_vertical_movement(movement.0, &mut instructions);
  } else if movement.1 < 0 {
    do_horizontal_movement(movement.1, &mut instructions);
    do_vertical_movement(movement.0, &mut instructions);
  } else {
    do_vertical_movement(movement.0, &mut instructions);
    do_horizontal_movement(movement.1, &mut instructions);
  }

  instructions.push('A');
  instructions
}
