use crate::day6::common::Tile;
use crate::day6::common::{parse_input, traverse};
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let (mut board, guard_pos, guard_dir) = parse_input(input_path)?;

  traverse(&mut board, &guard_pos, &guard_dir);

  let mut count = 0;
  for row in board {
    for tile in row {
      if matches!(tile, Tile::Visited(_)) {
        count += 1;
      }
    }
  }

  println!("{count}");

  Ok(())
}
