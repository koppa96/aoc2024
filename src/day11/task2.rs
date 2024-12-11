use crate::utils;
use crate::utils::read_lines;
use std::collections::HashMap;

pub fn solve(input_path: String) -> utils::Result {
  let stones: Vec<_> = read_lines(input_path)?
    .next()
    .unwrap()
    .split(" ")
    .map(|part| part.parse::<u64>())
    .flatten()
    .collect();

  let mut map = to_map(stones);
  for _i in 0..75 {
    map = blink(map);
  }

  println!("{}", count_stones(&map));

  Ok(())
}

fn blink(map: HashMap<u64, u64>) -> HashMap<u64, u64> {
  let mut new_stones = HashMap::new();
  for (key, value) in map {
    if key == 0 {
      increase_key(&mut new_stones, 1, value);
    } else if let Some((first, second)) = split(key) {
      increase_key(&mut new_stones, first, value);
      increase_key(&mut new_stones, second, value);
    } else {
      increase_key(&mut new_stones, key * 2024, value);
    }
  }

  new_stones
}

fn increase_key(map: &mut HashMap<u64, u64>, key: u64, by: u64) {
  match map.get(&key) {
    Some(count) => {
      map.insert(key, count + by);
    }
    None => {
      map.insert(key, by);
    }
  }
}

fn to_map(stones: Vec<u64>) -> HashMap<u64, u64> {
  let mut map = HashMap::new();
  for stone in stones {
    increase_key(&mut map, stone, 1);
  }

  map
}

fn count_stones(map: &HashMap<u64, u64>) -> u64 {
  let mut sum = 0;
  for (_, value) in map {
    sum += *value
  }

  sum
}

fn split(stone: u64) -> Option<(u64, u64)> {
  let digits = stone.ilog10() + 1;
  if digits % 2 == 0 {
    let divisor = 10u64.pow(digits / 2);
    return Some((stone / divisor, stone % divisor));
  }

  None
}
