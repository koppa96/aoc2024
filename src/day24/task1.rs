use crate::day24::common::{BinaryOperation, Const, Operator, Resolver};
use crate::utils;
use crate::utils::read_lines;
use regex::Regex;
use std::collections::HashMap;

pub fn solve(input_path: String) -> utils::Result {
  let mut lines = read_lines(input_path)?;
  let mut wires: HashMap<String, Box<dyn Resolver>> = HashMap::new();
  while let Some(line) = lines.next() {
    if line.len() == 0 {
      break;
    }

    let mut parts = line.split(": ");
    let name = parts.next().unwrap();
    let value = parts.next().unwrap() == "1";
    wires.insert(name.to_string(), Box::new(Const { value }));
  }

  let expr_regex = Regex::new("^(\\w+) (\\w+) (\\w+) -> (\\w+)$")?;
  while let Some(line) = lines.next() {
    let captures = expr_regex
      .captures(&line)
      .ok_or("Expected the line to be a logical expression")?;
    let lhs = captures.get(1).unwrap().as_str().to_string();
    let op = captures.get(2).unwrap().as_str();
    let rhs = captures.get(3).unwrap().as_str().to_string();
    let result = captures.get(4).unwrap().as_str().to_string();

    wires.insert(
      result,
      Box::new(BinaryOperation {
        lhs,
        rhs,
        op: match op {
          "AND" => Operator::And,
          "OR" => Operator::Or,
          "XOR" => Operator::Xor,
          _ => panic!("Unknown logical operator: {}", op),
        },
      }),
    );
  }

  let mut results: Vec<_> = wires.keys().filter(|k| k.starts_with("z")).collect();
  results.sort_by(|a, b| b.cmp(a));

  let mut value: usize = 0;
  for result in results {
    if wires[result].resolve(&wires) {
      value = (value << 1) | 1
    } else {
      value = value << 1
    }
  }

  println!("{value}");

  Ok(())
}
