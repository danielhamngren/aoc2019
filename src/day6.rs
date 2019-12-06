use crate::utils;
use std::collections::HashMap;

pub fn day6() {
  let filename = "resources/day6_input";
  let lines: Vec<String> = utils::read_file(filename);

  part1(&lines);
}

fn part1(lines: &Vec<String>) {
  println!("Day 6: Part 1");

  let tuples = preprocessing(lines);
  let orbit_pairs = preprocessing(&lines);

  let mut graph = HashMap::new();
  populate_graph(orbit_pairs, &mut graph);

  let total_connections = calculate_connections(&graph);

  println!("Result part 1: {}", total_connections);
}

fn part2(lines: &Vec<String>) {
  println!("Day 6: Part 1");

  let tuples = preprocessing(lines);
  let orbit_pairs = preprocessing(&lines);

  let mut graph = HashMap::new();
  populate_graph(orbit_pairs, &mut graph);

  let total_connections = calculate_connections(&graph);

  println!("Result part 1: {}", total_connections);
}

fn preprocessing(lines: &Vec<String>) -> Vec<(String, String)> {
  let mut pairs = Vec::new();
  for line in lines {
    let pair = line
      .split(")")
      .map(|e| String::from(e))
      .collect::<Vec<String>>();
    pairs.push((pair[0].clone(), pair[1].clone()));
  }
  pairs
}

fn populate_graph(pairs: Vec<(String, String)>, graph: &mut HashMap<String, String>) {
  for pair in pairs {
    let parent = pair.0;
    let child = pair.1;

    graph.insert(child, parent);
  }
}

fn connections_to_root_node(key: String, graph: &HashMap<String, String>) -> u32 {
  if graph.contains_key(&key) {
    let new_key = graph.get(&key).unwrap();
    return connections_to_root_node(new_key.to_string(), graph) + 1;
  }
  0
}

fn calculate_connections(graph: &HashMap<String, String>) -> u32 {
  let mut total_connections = 0;
  for key in graph.keys() {
    let connections = connections_to_root_node(key.to_string(), &graph);
    total_connections += connections;
  }
  total_connections
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let filename = "resources/day6_test_input";
    let lines: Vec<String> = utils::read_file(filename);
    let orbit_pairs = preprocessing(&lines);

    let mut graph = HashMap::new();
    populate_graph(orbit_pairs, &mut graph);

    let total_connections = calculate_connections(&graph);

    assert_eq!(total_connections, 42);
  }
}
