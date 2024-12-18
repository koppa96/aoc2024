use crate::day18::common::find_path_length_to_exit;
use crate::utils;
use crate::utils::read_lines;

const HEIGHT: usize = 71;
const WIDTH: usize = 71;
const NUM_BYTES: usize = 1024;

pub fn solve(input_path: String) -> utils::Result {
  let mut grid = vec![vec![false; WIDTH]; HEIGHT];
  let first_lines = read_lines(input_path)?.take(NUM_BYTES);
  for line in first_lines {
    let parts: Vec<_> = line.split(",").collect();
    let row: usize = parts[1].parse()?;
    let col: usize = parts[0].parse()?;

    grid[row][col] = true;
  }

  let path_len = find_path_length_to_exit(&grid, (0, 0), (HEIGHT - 1, WIDTH - 1));
  match path_len {
    Some(len) => println!("{len}"),
    None => println!("No path found."),
  }

  Ok(())
}
