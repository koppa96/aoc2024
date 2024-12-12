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
    result += edges(a) * area(a);
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

fn edges(area: &Vec<(usize, usize)>) -> usize {
  if area.len() == 1 {
    return 4;
  }

  let (top_left, bottom_right) = bounding_rect(area);
  if top_left.0 == bottom_right.0 || top_left.1 == bottom_right.1 {
    // Just a single line
    return 4;
  }

  let rows = (bottom_right.0 - top_left.0) + 3;
  let cols = (bottom_right.1 - top_left.1) + 3;
  let mut shape: Vec<Vec<bool>> = Vec::with_capacity(rows);

  for i in 0..rows {
    let mut line: Vec<bool> = Vec::with_capacity(cols);
    for j in 0..cols {
      if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
        line.push(false);
      } else if is_within(area, &(top_left.0 + i - 1, top_left.1 + j - 1)) {
        line.push(true)
      } else {
        line.push(false)
      }
    }

    shape.push(line);
  }

  let mut edges = 0;
  for i in 0..rows - 1 {
    let mut continuous = false;

    for j in 1..cols {
      if shape[i][j] != shape[i][j - 1] {
        continuous = false;
      }

      if shape[i][j] != shape[i + 1][j] {
        if !continuous {
          edges += 1;
        }

        continuous = true;
      } else {
        continuous = false;
      }
    }
  }

  for j in 0..cols - 1 {
    let mut continuous = false;

    for i in 1..rows {
      if shape[i][j] != shape[i - 1][j] {
        continuous = false;
      }

      if shape[i][j] != shape[i][j + 1] {
        if !continuous {
          edges += 1;
        }

        continuous = true;
      } else {
        continuous = false;
      }
    }
  }

  edges
}

fn is_within(area: &Vec<(usize, usize)>, pos: &(usize, usize)) -> bool {
  for area_pos in area {
    if area_pos == pos {
      return true;
    }
  }

  false
}

fn bounding_rect(area: &Vec<(usize, usize)>) -> ((usize, usize), (usize, usize)) {
  let mut top_left = (area[0].0, area[0].1);
  let mut bottom_right = (area[0].0, area[0].1);

  for i in 1..area.len() {
    if area[i].0 < top_left.0 {
      top_left.0 = area[i].0;
    }

    if area[i].1 < top_left.1 {
      top_left.1 = area[i].1
    }

    if area[i].0 > bottom_right.0 {
      bottom_right.0 = area[i].0
    }

    if area[i].1 > bottom_right.1 {
      bottom_right.1 = area[i].1
    }
  }

  (top_left, bottom_right)
}
