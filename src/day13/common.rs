use regex::Regex;

const A_PRICE: u64 = 3;
const B_PRICE: u64 = 1;

pub struct Game {
  a: (u64, u64),
  b: (u64, u64),
  prize: (u64, u64),
}

impl Game {
  pub fn parse(data: &[String], base_prize: u64) -> Result<Self, Box<dyn std::error::Error>> {
    let regex = Regex::new("\\d+")?;
    let mut game = Self {
      a: (0, 0),
      b: (0, 0),
      prize: (base_prize, base_prize),
    };

    let mut matches: Vec<_> = regex.find_iter(&data[0]).collect();
    game.a.0 = matches[0].as_str().parse()?;
    game.a.1 = matches[1].as_str().parse()?;

    matches = regex.find_iter(&data[1]).collect();
    game.b.0 = matches[0].as_str().parse()?;
    game.b.1 = matches[1].as_str().parse()?;

    matches = regex.find_iter(&data[2]).collect();
    game.prize.0 += matches[0].as_str().parse::<u64>()?;
    game.prize.1 += matches[1].as_str().parse::<u64>()?;

    Ok(game)
  }

  pub fn min_cost_to_win(&self) -> Option<u64> {
    let p0: i64 = self.prize.0 as i64;
    let p1: i64 = self.prize.1 as i64;
    let a0: i64 = self.a.0 as i64;
    let a1: i64 = self.a.1 as i64;
    let b0: i64 = self.b.0 as i64;
    let b1: i64 = self.b.1 as i64;

    let b_count = (a1 * p0 - a0 * p1) / (a1 * b0 - a0 * b1);
    if b_count < 0 || (a1 * p0 - a0 * p1) % (a1 * b0 - a0 * b1) != 0 {
      return None;
    }

    let a_count = (p1 - b1 * b_count) / a1;
    if a_count < 0 || (p1 - b1 * b_count) % a1 != 0 {
      return None;
    }

    Some((a_count as u64 * A_PRICE) + (b_count as u64 * B_PRICE))
  }
}
