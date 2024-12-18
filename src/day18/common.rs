use std::collections::{HashMap, VecDeque};

pub static DIRECTIONS: [fn(
  row: usize,
  col: usize,
  board: &Vec<Vec<bool>>,
) -> Option<(usize, usize)>; 4] = [
  |row, col, board| {
    if col == board[row].len() - 1 {
      None
    } else {
      Some((row, col + 1))
    }
  },
  |row, col, board| {
    if row == board.len() - 1 {
      None
    } else {
      Some((row + 1, col))
    }
  },
  |row, col, _| if col == 0 { None } else { Some((row, col - 1)) },
  |row, col, _| if row == 0 { None } else { Some((row - 1, col)) },
];

struct Step {
  pos: (usize, usize),
  len: usize,
}

pub fn find_path_length_to_exit(
  grid: &Vec<Vec<bool>>,
  start: (usize, usize),
  end: (usize, usize),
) -> Option<usize> {
  let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
  let mut queue = VecDeque::new();
  queue.push_back(Step { pos: start, len: 0 });

  while let Some(step) = queue.pop_front() {
    if grid[step.pos.0][step.pos.1] == true {
      continue;
    }

    if step.pos == end {
      return Some(step.len);
    }

    match visited.get(&step.pos) {
      Some(len) => {
        if step.len < *len {
          visited.insert(step.pos, step.len);
          for dir in DIRECTIONS {
            if let Some(next_pos) = dir(step.pos.0, step.pos.1, grid) {
              queue.push_back(Step {
                pos: next_pos,
                len: step.len + 1,
              });
            }
          }
        }
      }
      None => {
        visited.insert(step.pos, step.len);
        for dir in DIRECTIONS {
          if let Some(next_pos) = dir(step.pos.0, step.pos.1, grid) {
            queue.push_back(Step {
              pos: next_pos,
              len: step.len + 1,
            });
          }
        }
      }
    }
  }

  None
}
