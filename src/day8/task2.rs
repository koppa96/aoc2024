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
      if row == i && col == j || board[i][j] == '.' {
        continue;
      }

      if board[row][col] == board[i][j] {
        return true;
      }

      let dir_x = i as i32 - row as i32;
      let dir_y = j as i32 - col as i32;

      let (normalized_dir_x, normalized_dir_y) = normalize_dir(dir_x, dir_y);
      let mut curr_x = i as i32 + normalized_dir_x;
      let mut curr_y = j as i32 + normalized_dir_y;
      loop {
        if !is_inside(curr_x, curr_y, &board) {
          break;
        }

        if board[curr_x as usize][curr_y as usize] == board[i][j] {
          return true;
        }

        curr_x = curr_x + normalized_dir_x;
        curr_y = curr_y + normalized_dir_y;
      }
    }
  }

  false
}

fn normalize_dir(x: i32, y: i32) -> (i32, i32) {
  if x == 0 {
    return (0, 1);
  } else if y == 0 {
    return (1, 0);
  }

  let gcd = gcd(x.abs() as u32, y.abs() as u32) as i32;
  (x / gcd, y / gcd)
}

fn gcd(mut n: u32, mut m: u32) -> u32 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n {
      std::mem::swap(&mut m, &mut n);
    }
    m %= n;
  }
  n
}
