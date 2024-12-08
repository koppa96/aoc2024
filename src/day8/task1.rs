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
      if is_antinode(row, col, &board) {
        count += 1;
      }
    }
  }

  println!("{count}");

  Ok(())
}

fn is_inside(row: i32, col: i32, board: &Vec<Vec<char>>) -> bool {
  row >= 0 && col >= 0 && row < board.len() as i32 && col < board[row as usize].len() as i32
}

fn is_antinode(row: usize, col: usize, board: &Vec<Vec<char>>) -> bool {
  for i in 0..board.len() {
    for j in 0..board[i].len() {
      if i == row && j == col || board[i][j] == '.' {
        continue;
      }

      let dir_x = i as i32 - row as i32;
      let dir_y = j as i32 - col as i32;

      let other_pos_x = i as i32 + dir_x;
      let other_pos_y = j as i32 + dir_y;
      if is_inside(other_pos_x, other_pos_y, &board)
        && board[other_pos_x as usize][other_pos_y as usize] == board[i][j]
      {
        return true;
      }
    }
  }

  false
}
