use crate::day12::common::DIRECTIONS;
use crate::utils;
use crate::utils::read_lines;
use std::collections::HashMap;

pub fn solve(input_path: String) -> utils::Result {
  let mut register: HashMap<char, Vec<Vec<(usize, usize)>>> = HashMap::new();
  for (row, line) in read_lines(input_path)?.enumerate() {
    for (col, char) in line.chars().enumerate() {
      match register.get_mut(&char) {
        Some(areas) => {
          insert_and_merge(areas, (row, col));
        }
        None => {
          register.insert(char, vec![vec![(row, col)]]);
        }
      }
    }
  }

  let mut result = 0;
  for a in register.values().flatten() {
    result += perimeter(a) * area(a);
  }

  println!("{result}");

  Ok(())
}

fn insert_and_merge(areas: &mut Vec<Vec<(usize, usize)>>, (row, col): (usize, usize)) {
  let mut inserted_at = None;
  let mut areas_to_merge: Vec<usize> = Vec::new();

  for i in 0..areas.len() {
    if belongs_to(&areas[i], (row, col)) {
      match inserted_at {
        Some(_) => {
          areas_to_merge.push(i);
        }
        None => {
          areas[i].push((row, col));
          inserted_at = Some(i);
        }
      }
    }
  }

  match inserted_at {
    Some(pos) => {
      for area_idx in areas_to_merge.iter().rev() {
        let area = areas.remove(*area_idx);
        merge_into(&mut areas[pos], area);
      }
    }
    None => areas.push(vec![(row, col)]),
  }
}

fn merge_into(target: &mut Vec<(usize, usize)>, src: Vec<(usize, usize)>) {
  for (row, col) in src {
    target.push((row, col));
  }
}

fn belongs_to(area: &Vec<(usize, usize)>, pos: (usize, usize)) -> bool {
  for area_pos in area {
    if is_next_to(area_pos, &pos) {
      return true;
    }
  }

  false
}

fn is_next_to((row1, col1): &(usize, usize), (row2, col2): &(usize, usize)) -> bool {
  for dir in DIRECTIONS {
    if let Some((next_row, next_col)) = dir(*row1, *col1) {
      if next_row == *row2 && next_col == *col2 {
        return true;
      }
    }
  }

  false
}

fn area(area: &Vec<(usize, usize)>) -> usize {
  area.len()
}

fn perimeter(area: &Vec<(usize, usize)>) -> usize {
  let mut perimeter = 0;
  for pos in area {
    let mut edges_without_neighbors = 4;
    for other in area {
      if is_next_to(pos, other) {
        edges_without_neighbors -= 1;
      }
    }

    perimeter += edges_without_neighbors;
  }

  perimeter
}
