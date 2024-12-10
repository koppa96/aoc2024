pub static DIRECTIONS: [fn(
  row: usize,
  col: usize,
  board: &Vec<Vec<u32>>,
) -> Option<(usize, usize)>; 4] = [
  |row, col, board| {
    if col == board[row].len() - 1 {
      None
    } else {
      Some((row, col + 1))
    }
  },
  |row, col, board| {
    if row == board.len() - 1 {
      None
    } else {
      Some((row + 1, col))
    }
  },
  |row, col, _| if col == 0 { None } else { Some((row, col - 1)) },
  |row, col, _| if row == 0 { None } else { Some((row - 1, col)) },
];
