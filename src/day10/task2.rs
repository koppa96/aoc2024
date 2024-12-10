use crate::day10::common::DIRECTIONS;
use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let board: Vec<Vec<_>> = read_lines(input_path)?
    .map(|line| {
      line
        .chars()
        .map(|char| char.to_digit(10))
        .flatten()
        .collect()
    })
    .collect();

  let mut sum = 0;
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == 0 {
        sum += rating(row, col, &board);
      }
    }
  }

  println!("{sum}");

  Ok(())
}

fn rating(row: usize, col: usize, board: &Vec<Vec<u32>>) -> u32 {
  if board[row][col] == 9 {
    return 1;
  }

  let mut s = 0;
  for direction in DIRECTIONS {
    if let Some((next_row, next_col)) = direction(row, col, board) {
      if board[next_row][next_col] > board[row][col]
        && board[next_row][next_col] - board[row][col] == 1
      {
        s += rating(next_row, next_col, board);
      }
    }
  }

  s
}
