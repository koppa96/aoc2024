static NUMBERS: [[char; 3]; 4] = [
  ['7', '8', '9'],
  ['4', '5', '6'],
  ['1', '2', '3'],
  [' ', '0', 'A'],
];

pub static INSTRUCTIONS: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

pub fn numbers_to_instructions(nums: Vec<char>) -> Vec<char> {
  let mut current_pos = (3, 2);
  let mut instructions = Vec::new();
  for num in nums {
    let pos = locate_number(num).unwrap();
    instructions.extend_from_slice(&to_instructions(&current_pos, &pos));
    current_pos = pos;
  }

  instructions
}

pub fn to_instructions(start: &(usize, usize), end: &(usize, usize)) -> Vec<char> {
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

pub fn do_horizontal_movement(diff: i32, instructions: &mut Vec<char>) {
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

pub fn do_vertical_movement(diff: i32, instructions: &mut Vec<char>) {
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

pub fn locate_instruction(instruction: char) -> Option<(usize, usize)> {
  for i in 0..INSTRUCTIONS.len() {
    for j in 0..INSTRUCTIONS[i].len() {
      if INSTRUCTIONS[i][j] == instruction {
        return Some((i, j));
      }
    }
  }

  None
}

pub fn locate_number(num: char) -> Option<(usize, usize)> {
  for i in 0..NUMBERS.len() {
    for j in 0..NUMBERS[i].len() {
      if NUMBERS[i][j] == num {
        return Some((i, j));
      }
    }
  }

  None
}
