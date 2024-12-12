pub static DIRECTIONS: [fn(row: usize, col: usize) -> Option<(usize, usize)>; 4] = [
  |row, col| Some((row, col + 1)),
  |row, col| Some((row + 1, col)),
  |row, col| if col == 0 { None } else { Some((row, col - 1)) },
  |row, col| if row == 0 { None } else { Some((row - 1, col)) },
];
