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

  let mut max_clique = bron_kerbosch(
    &graph,
    Vec::new(),
    &mut graph.keys().map(|k| k.as_str()).collect(),
    &mut Vec::new(),
  );

  max_clique.sort();
  println!("{}", max_clique.join(","));

  Ok(())
}

fn bron_kerbosch<'a>(
  graph: &'a HashMap<String, Vec<String>>,
  r: Vec<&'a str>,
  p: &mut Vec<&'a str>,
  x: &mut Vec<&'a str>,
) -> Vec<&'a str> {
  if p.is_empty() && x.is_empty() {
    return r;
  }

  let mut max = Vec::new();
  while !p.is_empty() {
    let mut new_r = r.clone();
    new_r.push(p[0]);

    let mut new_p = Vec::new();
    for n in &graph[p[0]] {
      if p.contains(&n.as_str()) {
        new_p.push(n.as_str());
      }
    }

    let mut new_x = Vec::new();
    for n in &graph[p[0]] {
      if x.contains(&n.as_str()) {
        new_x.push(n.as_str());
      }
    }

    let clique = bron_kerbosch(graph, new_r, &mut new_p, &mut new_x);
    if clique.len() > max.len() {
      max = clique;
    }

    x.push(p[0]);
    p.remove(0);
  }

  max
}
