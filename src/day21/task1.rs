use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut sum = 0;
  for line in read_lines(input_path)? {
    let number_part = &line[0..line.len() - 1];
    let number = number_part.parse::<usize>()?;

    let nums = line.chars().collect();
    let instructions =
      instructions_to_instructions(instructions_to_instructions(numbers_to_instructions(nums)));
    sum += number * instructions.len();
  }

  println!("{}", sum);

  Ok(())
}

static NUMBERS: [[char; 3]; 4] = [
  ['7', '8', '9'],
  ['4', '5', '6'],
  ['1', '2', '3'],
  [' ', '0', 'A'],
];

static INSTRUCTIONS: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

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
    } else if current_pos.1 == 0 && pos.0 == 1 {
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

fn numbers_to_instructions(nums: Vec<char>) -> Vec<char> {
  let mut current_pos = (3, 2);
  let mut instructions = Vec::new();
  for num in nums {
    let pos = locate_number(num).unwrap();
    instructions.extend_from_slice(&to_instructions(&current_pos, &pos));
    current_pos = pos;
  }

  instructions
}

fn to_instructions(start: &(usize, usize), end: &(usize, usize)) -> Vec<char> {
  let mut instructions = Vec::new();
  let movement = (end.0 as i32 - start.0 as i32, end.1 as i32 - start.1 as i32);

  // If possible move LEFT first
  // Then move UP
  // Then move DOWN
  // Then move RIGHT
  if start.0 == 3 && end.1 == 0 {
    do_vertical_movement(movement.0, &mut instructions);
    do_horizontal_movement(movement.1, &mut instructions);
  } else if start.1 == 0 && end.0 == 3 {
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

fn do_horizontal_movement(diff: i32, instructions: &mut Vec<char>) {
  if diff > 0 {
    for _ in 0..diff {
      instructions.push('>');
    }
  } else {
    for _ in 0..-diff {
      instructions.push('<');
    }
  }
}

fn do_vertical_movement(diff: i32, instructions: &mut Vec<char>) {
  if diff > 0 {
    for _ in 0..diff {
      instructions.push('v');
    }
  } else {
    for _ in 0..-diff {
      instructions.push('^');
    }
  }
}

fn locate_instruction(instruction: char) -> Option<(usize, usize)> {
  for i in 0..INSTRUCTIONS.len() {
    for j in 0..INSTRUCTIONS[i].len() {
      if INSTRUCTIONS[i][j] == instruction {
        return Some((i, j));
      }
    }
  }

  None
}

fn locate_number(num: char) -> Option<(usize, usize)> {
  for i in 0..NUMBERS.len() {
    for j in 0..NUMBERS[i].len() {
      if NUMBERS[i][j] == num {
        return Some((i, j));
      }
    }
  }

  None
}
