use crate::day22::common::next;
use crate::utils;
use crate::utils::read_lines;
use std::collections::HashMap;

pub fn solve(input_path: String) -> utils::Result {
  let mut diffs = HashMap::new();
  for line in read_lines(input_path)? {
    let mut num: usize = line.parse()?;
    let mut price_change_buf = (None, None, None, None);
    let mut prev_price = num % 10;
    let mut current_diffs = HashMap::new();
    for _ in 0..2000 {
      num = next(num);
      let price = num % 10;
      price_change_buf = (
        price_change_buf.1,
        price_change_buf.2,
        price_change_buf.3,
        Some(price as i32 - prev_price as i32),
      );

      if price_change_buf.0.is_some() {
        if !current_diffs.contains_key(&price_change_buf) {
          current_diffs.insert(price_change_buf, price);
        }
      }

      prev_price = price;
    }

    for (key, value) in current_diffs.drain() {
      match diffs.get(&key) {
        Some(sum) => {
          diffs.insert(key, sum + value);
        }
        None => {
          diffs.insert(key, value);
        }
      }
    }
  }

  let mut max_value = None;
  for value in diffs.values() {
    if let Some(max_val) = max_value {
      if value > max_val {
        max_value = Some(value);
      }
    } else {
      max_value = Some(value);
    }
  }

  println!("{}", max_value.unwrap());

  Ok(())
}
