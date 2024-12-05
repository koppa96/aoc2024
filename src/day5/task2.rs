use crate::day5::common::{find_first_violation, Rule};
use crate::utils;
use crate::utils::read_lines;

pub fn solve(input_path: String) -> utils::Result {
  let mut rules: Vec<Rule> = Vec::new();
  let mut reading_rules = true;
  let mut sum = 0;
  for line in read_lines(input_path)? {
    if line == "" {
      reading_rules = false;
    } else if reading_rules {
      rules.push(Rule::parse(&line))
    } else {
      let mut pages: Vec<_> = line
        .split(",")
        .map(|part| part.parse::<i32>())
        .flatten()
        .collect();

      if find_first_violation(&pages, &rules) != None {
        reorder_pages(&mut pages, &rules);
        sum += pages[pages.len() / 2];
      }
    }
  }

  println!("{sum}");

  Ok(())
}

fn reorder_pages(pages: &mut Vec<i32>, rules: &Vec<Rule>) {
  while let Some((first, second)) = find_first_violation(pages, rules) {
    pages.swap(first, second);
  }
}
