use crate::day25::task1::Reading::{Key, Lock};
use crate::utils;
use crate::utils::read_lines;

enum Reading {
  Lock,
  Key,
}

pub fn solve(input_path: String) -> utils::Result {
  let mut locks = Vec::new();
  let mut keys = Vec::new();
  let mut reading = None;
  let mut current: Vec<u32> = vec![0; 5];
  for line in read_lines(input_path)? {
    if line.len() == 0 {
      match reading {
        Some(Lock) => {
          locks.push(current);
        }
        Some(Key) => {
          for i in 0..current.len() {
            current[i] -= 1;
          }

          keys.push(current);
        }
        _ => {}
      }

      current = vec![0; 5];
      reading = None;
      continue;
    }

    if reading.is_none() {
      reading = if line == "....." {
        Some(Key)
      } else {
        Some(Lock)
      };
      continue;
    }

    for (i, ch) in line.chars().enumerate() {
      if ch == '#' {
        current[i] += 1;
      }
    }
  }

  match reading {
    Some(Lock) => {
      locks.push(current);
    }
    Some(Key) => {
      for i in 0..current.len() {
        current[i] -= 1;
      }

      keys.push(current);
    }
    _ => {}
  }

  let mut count = 0;
  for lock in &locks {
    for key in &keys {
      if fits_lock(key, lock) {
        count += 1;
      }
    }
  }

  println!("{count}");

  Ok(())
}

fn fits_lock(key: &Vec<u32>, lock: &Vec<u32>) -> bool {
  if lock.len() != key.len() {
    return false;
  }

  for i in 0..lock.len() {
    if lock[i] + key[i] > 5 {
      return false;
    }
  }

  true
}
