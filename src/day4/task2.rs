use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut board: Vec<Vec<char>> = Vec::new();
  for line in read_lines(input_path)? {
    board.push(line.chars().collect());
  }

  let mut count = 0;
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if is_xmas_center(row, col, &board) {
        count += 1;
      }
    }
  }

  println!("{count}");

  Ok(())
}

fn is_xmas_center(row: usize, col: usize, board: &Vec<Vec<char>>) -> bool {
  if board[row][col] != 'A' {
    return false;
  }

  if row == 0 || col == 0 || row >= board.len() - 1 || col >= board[row].len() - 1 {
    return false;
  }

  let has_diagonal_mas = (board[row + 1][col + 1] == 'M' && board[row - 1][col - 1] == 'S')
    || (board[row - 1][col - 1] == 'M' && board[row + 1][col + 1] == 'S');
  if !has_diagonal_mas {
    return false;
  }

  (board[row + 1][col - 1] == 'M' && board[row - 1][col + 1] == 'S')
    || (board[row - 1][col + 1] == 'M' && board[row + 1][col - 1] == 'S')
}
