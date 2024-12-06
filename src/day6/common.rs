use crate::utils::read_lines;
use std::io;

#[derive(Clone, Eq, PartialEq)]
pub(crate) enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  pub(crate) fn right(&self) -> Direction {
    match self {
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
    }
  }

  pub(crate) fn next(&self, (row, col): (i32, i32)) -> (i32, i32) {
    match self {
      Direction::Up => (row - 1, col),
      Direction::Right => (row, col + 1),
      Direction::Down => (row + 1, col),
      Direction::Left => (row, col - 1),
    }
  }
}

pub(crate) enum Tile {
  Empty,
  Visited(Direction),
  Obstacle,
}

pub(crate) fn parse_input(
  input_path: String,
) -> io::Result<(Vec<Vec<Tile>>, (i32, i32), Direction)> {
  let mut board: Vec<Vec<Tile>> = Vec::new();
  let mut guard_pos = (0, 0);
  let mut guard_dir = Direction::Up;

  for (i, line) in read_lines(input_path)?.enumerate() {
    let mut row: Vec<Tile> = Vec::new();
    for (j, char) in line.chars().enumerate() {
      row.push(match char {
        '#' => Tile::Obstacle,
        '^' => {
          guard_pos = (i as i32, j as i32);
          guard_dir = Direction::Up;
          Tile::Visited(Direction::Up)
        }
        '>' => {
          guard_pos = (i as i32, j as i32);
          guard_dir = Direction::Right;
          Tile::Visited(Direction::Right)
        }
        'v' => {
          guard_pos = (i as i32, j as i32);
          guard_dir = Direction::Down;
          Tile::Visited(Direction::Down)
        }
        '<' => {
          guard_pos = (i as i32, j as i32);
          guard_dir = Direction::Left;
          Tile::Visited(Direction::Left)
        }
        _ => Tile::Empty,
      })
    }

    board.push(row);
  }

  Ok((board, guard_pos, guard_dir))
}

fn is_inside((row, col): (i32, i32), board: &Vec<Vec<Tile>>) -> bool {
  row >= 0 && col >= 0 && row < board.len() as i32 && col < board[row as usize].len() as i32
}

pub(crate) fn traverse(
  board: &mut Vec<Vec<Tile>>,
  guard_pos: &(i32, i32),
  guard_dir: &Direction,
) -> bool {
  let mut current_pos = guard_pos.to_owned();
  let mut current_dir = guard_dir.to_owned();

  loop {
    let next_pos = current_dir.next(current_pos);
    if !is_inside(next_pos, &board) {
      return false;
    }

    match &board[next_pos.0 as usize][next_pos.1 as usize] {
      Tile::Obstacle => {
        current_dir = current_dir.right();
      }
      Tile::Visited(dir) => {
        if *dir == current_dir {
          return true;
        } else {
          current_pos = next_pos;
          board[current_pos.0 as usize][current_pos.1 as usize] =
            Tile::Visited(current_dir.clone());
        }
      }
      _ => {
        current_pos = next_pos;
        board[current_pos.0 as usize][current_pos.1 as usize] = Tile::Visited(current_dir.clone());
      }
    }
  }
}
