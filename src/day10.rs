use crate::utils;
use num::rational::Ratio;
use std::cmp::Eq;
use std::collections::HashSet;

pub fn day10() {
  let filename = "resources/day10_input";
  let lines: Vec<String> = utils::read_file(filename);

  part1(&lines);
  // part2(&lines);
}

fn part1(lines: &Vec<String>) {
  println!("Day 10: Part 1");
  let asteroids = preprocessing(&lines);

  let (best_location, result) = find_best_location(asteroids);

  println!("location: {:?}, result: {}", best_location, result);

  println!("Result part 1: {}", result);
}

fn part2(lines: &Vec<String>) {
  println!("Day 8: Part 1");
  let layers = preprocessing(&lines);

  // let master_image = collapse_layers(&layers);

  // for (i, pixel) in master_image.data.iter().enumerate() {
  //   if *pixel == 0 {
  //     print!("░");
  //   } else {
  //     print!("▓");
  //   }
  //   if (i + 1) % 25 == 0 {
  //     println!("");
  //   }
  // }
  // println!("");
}

// #[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Asteroid {
  x: i32,
  y: i32,
}

fn preprocessing(lines: &Vec<String>) -> HashSet<Asteroid> {
  println!("len lines {}", lines.len());

  println!("len lines[0] {}", lines[0].len());
  // println!("layers {}", lines[0].len() / (height * width));

  let mut asteroids: HashSet<Asteroid> = HashSet::new();

  for y in 0..lines.len() {
    for (x, c) in lines[y].chars().enumerate() {
      if c == '#' {
        asteroids.insert(Asteroid {
          x: x as i32,
          y: y as i32,
        });
      }
    }
  }

  asteroids
}

fn check_candidate_visible_asteroids(candidate: &Asteroid, asteroids: &HashSet<Asteroid>) -> usize {
  let mut pos_directions: HashSet<Ratio<i32>> = HashSet::new();
  let mut neg_directions: HashSet<Ratio<i32>> = HashSet::new();
  for asteroid in asteroids.iter() {
    if asteroid == candidate {
      continue;
    }
    let mut dy = asteroid.y - candidate.y;
    let mut dx = asteroid.x - candidate.x;

    if dx == 0 {
      if dy < 0 {
        dy = i32::min_value();
      } else {
        dy = i32::max_value();
      }
      dx = 1;
    }
    if dx < 0 {
      neg_directions.insert(Ratio::from(dy) / dx);
    } else {
      pos_directions.insert(Ratio::from(dy) / dx);
    }
  }
  // println!("{:?}", directions);
  pos_directions.len() + neg_directions.len()
}

fn find_best_location(asteroids: HashSet<Asteroid>) -> (Asteroid, usize) {
  let mut best_candidate = Asteroid { x: 0, y: 0 };
  let mut max_visible_asteroids = 0;

  for candidate in &asteroids {
    let visible_asteroids = check_candidate_visible_asteroids(candidate, &asteroids);
    if visible_asteroids > max_visible_asteroids {
      max_visible_asteroids = visible_asteroids;
      best_candidate = candidate.clone();
    }
  }

  (best_candidate, max_visible_asteroids)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let filename = "resources/day10_input_test1";
    let lines: Vec<String> = utils::read_file(filename);
    let asteroids = preprocessing(&lines);

    let (best_location, result) = find_best_location(asteroids);

    println!("location: {:?}, result: {}", best_location, result);

    assert_eq!(result, 8);
  }
  #[test]
  fn example_2() {
    let filename = "resources/day10_input_test2";
    let lines: Vec<String> = utils::read_file(filename);
    let asteroids = preprocessing(&lines);

    let (best_location, result) = find_best_location(asteroids);

    println!("location: {:?}, result: {}", best_location, result);

    assert_eq!(result, 33);
  }
  #[test]
  fn example_3() {
    let filename = "resources/day10_input_test3";
    let lines: Vec<String> = utils::read_file(filename);
    let asteroids = preprocessing(&lines);

    let (best_location, result) = find_best_location(asteroids);

    println!("location: {:?}, result: {}", best_location, result);

    assert_eq!(result, 35);
  }
  #[test]
  fn example_4() {
    let filename = "resources/day10_input_test4";
    let lines: Vec<String> = utils::read_file(filename);
    let asteroids = preprocessing(&lines);

    let (best_location, result) = find_best_location(asteroids);

    println!("location: {:?}, result: {}", best_location, result);

    assert_eq!(result, 41);
  }
  #[test]
  fn example_5() {
    let filename = "resources/day10_input_test5";
    let lines: Vec<String> = utils::read_file(filename);
    let asteroids = preprocessing(&lines);

    let (best_location, result) = find_best_location(asteroids);

    println!("location: {:?}, result: {}", best_location, result);

    assert_eq!(result, 210);
  }
}
