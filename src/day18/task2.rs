use crate::day18::common::find_path_length_to_exit;
use crate::utils;
use crate::utils::read_lines;

const HEIGHT: usize = 71;
const WIDTH: usize = 71;

pub fn solve(input_path: String) -> utils::Result {
  let mut grid = vec![vec![false; WIDTH]; HEIGHT];
  for line in read_lines(input_path)? {
    let parts: Vec<_> = line.split(",").collect();
    let row: usize = parts[1].parse()?;
    let col: usize = parts[0].parse()?;

    grid[row][col] = true;
    let path_len = find_path_length_to_exit(&grid, (0, 0), (HEIGHT - 1, WIDTH - 1));
    if path_len.is_none() {
      println!("{line}");
      break;
    }
  }

  Ok(())
}
