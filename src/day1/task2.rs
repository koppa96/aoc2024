use crate::day1::common::read_input;
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let input = read_input(input_path)?;

  let mut sum = 0;
  for item in &input.list1 {
    sum += get_similarity_score(item, &input.list2);
  }

  println!("{sum}");

  Ok(())
}

fn get_similarity_score(item: &i32, other_list: &Vec<i32>) -> i32 {
  let occurrences = other_list.iter().filter(|other| **other == *item).count();

  *item * (occurrences as i32)
}
