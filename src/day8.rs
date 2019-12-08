use crate::utils;
use std::collections::HashMap;

pub fn day8() {
  let filename = "resources/day8_input";
  let lines: Vec<String> = utils::read_file(filename);

  part1(&lines);
  part2(&lines);
}

fn part1(lines: &Vec<String>) {
  println!("Day 8: Part 1");
  let layers = preprocessing(&lines);

  let index_of_least_zeros = find_index_with_least_zeros(&layers);

  let ones = get_number_of_element(&layers[index_of_least_zeros], 1);
  let twoes = get_number_of_element(&layers[index_of_least_zeros], 2);

  println!("Result part 1: {}", ones * twoes);
}

fn part2(lines: &Vec<String>) {
  println!("Day 8: Part 1");
  let layers = preprocessing(&lines);

  let master_image = collapse_layers(&layers);

  for (i, pixel) in master_image.data.iter().enumerate() {
    if *pixel == 0 {
      print!("░");
    } else {
      print!("▓");
    }
    if (i + 1) % 25 == 0 {
      println!("");
    }
  }
  println!("");
}

fn collapse_layers(layers: &Vec<Layer>) -> Layer {
  let mut master_layer = layers[0].clone();

  for layer in layers {
    for i in 0..layer.data.len() {
      if master_layer.data[i] == 2 {
        master_layer.data[i] = layer.data[i];
      }
    }
  }

  master_layer
}

#[derive(Debug, Clone)]
struct Layer {
  data: Vec<u32>,
}

fn preprocessing(lines: &Vec<String>) -> Vec<Layer> {
  let height = 6;
  let width = 25;

  let mut layers: Vec<Layer> = Vec::new();

  println!("len lines[0] {}", lines[0].len());
  println!("layers {}", lines[0].len() / (height * width));

  let layer_amount = lines[0].len() / (height * width);

  for i in 0..layer_amount {
    layers.push(Layer { data: Vec::new() });
  }

  for (i, c) in lines[0].chars().enumerate() {
    let l = i / (height * width);
    layers[l].data.push(c.to_digit(10).expect("digit failure"));
    c.to_digit(10).expect("to digit failure");
  }

  layers
}

fn find_index_with_least_zeros(layers: &Vec<Layer>) -> usize {
  let mut least_number_of_zeros = layers[0].data.len();
  let mut index_with_least = 0;

  for (i, layer) in layers.iter().enumerate() {
    let number_of_zeros = get_number_of_element(layer, 0);

    if number_of_zeros < least_number_of_zeros {
      least_number_of_zeros = number_of_zeros;
      index_with_least = i;
    }
  }
  index_with_least
}

fn get_number_of_element(layer: &Layer, element: u32) -> usize {
  layer.data.iter().filter(|&x| *x == element).count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let filename = "resources/day8_input";
    let lines: Vec<String> = utils::read_file(filename);
    let layers = preprocessing(&lines);

    let index_of_least_zeros = find_index_with_least_zeros(&layers);

    let ones = get_number_of_element(&layers[index_of_least_zeros], 1);
    let twoes = get_number_of_element(&layers[index_of_least_zeros], 2);

    println!("{} * {} = {}", ones, twoes, ones * twoes);

    assert_eq!(6, 42);
  }
}
