use crate::day21::common::{
  do_horizontal_movement, do_vertical_movement, locate_instruction, numbers_to_instructions,
};
use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut sum = 0;
  for line in read_lines(input_path)? {
    let number_part = &line[0..line.len() - 1];
    let number = number_part.parse::<usize>()?;

    let nums = line.chars().collect();
    let mut instructions = numbers_to_instructions(nums);
    instructions = instructions_to_instructions(instructions);
    instructions = instructions_to_instructions(instructions);
    sum += number * instructions.len();
  }

  println!("{}", sum);

  Ok(())
}

fn instructions_to_instructions(instructions: Vec<char>) -> Vec<char> {
  let mut current_pos = (0, 2);
  let mut new_instructions = Vec::new();
  for instruction in instructions {
    let pos = locate_instruction(instruction).unwrap();
    let movement = (
      pos.0 as i32 - current_pos.0 as i32,
      pos.1 as i32 - current_pos.1 as i32,
    );

    // If possible move LEFT first
    // Then move UP
    // Then move DOWN
    // Then move RIGHT
    if current_pos.0 == 0 && pos.1 == 0 {
      do_vertical_movement(movement.0, &mut new_instructions);
      do_horizontal_movement(movement.1, &mut new_instructions);
    } else if current_pos.1 == 0 && pos.0 == 0 {
      do_horizontal_movement(movement.1, &mut new_instructions);
      do_vertical_movement(movement.0, &mut new_instructions);
    } else if movement.1 < 0 {
      do_horizontal_movement(movement.1, &mut new_instructions);
      do_vertical_movement(movement.0, &mut new_instructions);
    } else {
      do_vertical_movement(movement.0, &mut new_instructions);
      do_horizontal_movement(movement.1, &mut new_instructions);
    }

    new_instructions.push('A');
    current_pos = pos;
  }

  new_instructions
}
