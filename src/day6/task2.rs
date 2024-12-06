use crate::day6::common::{parse_input, traverse, Direction, Tile};
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let (mut board, guard_pos, guard_dir) = parse_input(input_path)?;

  traverse(&mut board, &guard_pos, &guard_dir);

  let mut visited: Vec<(i32, i32)> = Vec::new();
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      match board[row][col] {
        Tile::Visited(_) => visited.push((row as i32, col as i32)),
        _ => {}
      }
    }
  }

  let mut circles = 0;
  for i in 1..visited.len() {
    reset(&mut board, &guard_pos, &guard_dir);
    board[visited[i].0 as usize][visited[i].1 as usize] = Tile::Obstacle;

    let has_circle = traverse(&mut board, &guard_pos, &guard_dir);
    if has_circle {
      circles += 1;
    }

    board[visited[i].0 as usize][visited[i].1 as usize] = Tile::Empty;
  }

  println!("{}", circles);

  Ok(())
}

fn reset(board: &mut Vec<Vec<Tile>>, (guard_row, guard_col): &(i32, i32), guard_dir: &Direction) {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if let Tile::Visited(_) = board[row][col] {
        board[row][col] = Tile::Empty;
      }
    }
  }

  board[*guard_row as usize][*guard_col as usize] = Tile::Visited(guard_dir.to_owned());
}
