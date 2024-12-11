use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut stones: Vec<_> = read_lines(input_path)?
    .next()
    .unwrap()
    .split(" ")
    .map(|part| part.parse::<u64>())
    .flatten()
    .collect();

  for _i in 0..25 {
    blink(&mut stones);
  }

  println!("{}", stones.len());

  Ok(())
}

fn blink(list: &mut Vec<u64>) {
  let mut i = 0;
  while i < list.len() {
    if list[i] == 0 {
      list[i] = 1;
      i += 1;
    } else if let Some((first, second)) = split(list[i]) {
      list[i] = first;
      list.insert(i + 1, second);
      i += 2;
    } else {
      list[i] *= 2024;
      i += 1;
    }
  }
}

fn split(stone: u64) -> Option<(u64, u64)> {
  let digits = stone.ilog10() + 1;
  if digits % 2 == 0 {
    let divisor = 10u64.pow(digits / 2);
    return Some((stone / divisor, stone % divisor));
  }

  None
}
