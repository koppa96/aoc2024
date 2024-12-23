use crate::utils;
use crate::utils::read_lines;
use std::collections::HashMap;

pub fn solve(input_path: String) -> utils::Result {
  let mut graph: HashMap<String, Vec<String>> = HashMap::new();
  for line in read_lines(input_path)? {
    let parts: Vec<_> = line.split("-").collect();
    match graph.get_mut(&parts[0].to_string()) {
      Some(neighbors) => {
        neighbors.push(parts[1].to_string());
      }
      None => {
        graph.insert(parts[0].to_string(), vec![parts[1].to_string()]);
      }
    };

    match graph.get_mut(&parts[1].to_string()) {
      Some(neighbors) => {
        neighbors.push(parts[0].to_string());
      }
      None => {
        graph.insert(parts[1].to_string(), vec![parts[0].to_string()]);
      }
    };
  }

  let mut count = 0;
  for (computer, neighbors) in graph.iter() {
    for i in 0..(neighbors.len() - 1) {
      let n1 = &neighbors[i];
      for j in (i + 1)..neighbors.len() {
        let n2 = &neighbors[j];
        if graph[n1].contains(&n2)
          && (computer.starts_with("t") || n1.starts_with("t") || n2.starts_with("t"))
        {
          count += 1;
        }
      }
    }
  }

  println!("{}", count / 3);

  Ok(())
}
