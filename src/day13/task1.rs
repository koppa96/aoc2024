use crate::day13::common::Game;
use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let lines = read_lines(input_path)?
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();
  let games = lines.chunks(3).collect::<Vec<_>>();

  let mut sum = 0;
  for game_data in games {
    let game = Game::parse(game_data, 0)?;
    if let Some(cost) = game.min_cost_to_win() {
      sum += cost;
    }
  }

  println!("{sum}");

  Ok(())
}
