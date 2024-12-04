use crate::utils;
use crate::utils::read_lines;

struct Pos {
  row: i32,
  col: i32,
}

type Direction = fn(pos: Pos) -> Pos;

static DIRECTIONS: [Direction; 8] = [
  |pos| Pos {
    row: pos.row,
    col: pos.col + 1,
  },
  |pos| Pos {
    row: pos.row + 1,
    col: pos.col + 1,
  },
  |pos| Pos {
    row: pos.row + 1,
    col: pos.col,
  },
  |pos| Pos {
    row: pos.row + 1,
    col: pos.col - 1,
  },
  |pos| Pos {
    row: pos.row,
    col: pos.col - 1,
  },
  |pos| Pos {
    row: pos.row - 1,
    col: pos.col - 1,
  },
  |pos| Pos {
    row: pos.row - 1,
    col: pos.col,
  },
  |pos| Pos {
    row: pos.row - 1,
    col: pos.col + 1,
  },
];

pub fn solve(input_path: String) -> utils::Result {
  let mut board: Vec<Vec<char>> = Vec::new();
  for line in read_lines(input_path)? {
    board.push(line.chars().collect());
  }

  let mut count = 0;
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      count += count_xmases_starting_from(row, col, &board);
    }
  }

  println!("{count}");

  Ok(())
}

fn is_outside(pos: &Pos, board: &Vec<Vec<char>>) -> bool {
  pos.row < 0
    || pos.col < 0
    || pos.row >= board.len() as i32
    || pos.col >= board[pos.row as usize].len() as i32
}

fn count_xmases_starting_from(row: usize, col: usize, board: &Vec<Vec<char>>) -> i32 {
  if board[row][col] != 'X' {
    return 0;
  }

  let mut count = 0;
  for direction in DIRECTIONS {
    let mut pos = direction(Pos {
      row: row as i32,
      col: col as i32,
    });
    if is_outside(&pos, board) || board[pos.row as usize][pos.col as usize] != 'M' {
      continue;
    }

    pos = direction(pos);
    if is_outside(&pos, board) || board[pos.row as usize][pos.col as usize] != 'A' {
      continue;
    }

    pos = direction(pos);
    if is_outside(&pos, board) || board[pos.row as usize][pos.col as usize] != 'S' {
      continue;
    }

    count += 1;
  }

  count
}
