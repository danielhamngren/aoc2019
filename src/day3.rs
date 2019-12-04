use crate::utils;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;

pub fn day3() {
  let filename = "resources/day3_input";
  let lines: Vec<String> = utils::read_file(filename);

  part1(&lines);
  part2(&lines);
}

fn part1(lines: &Vec<String>) {
  println!("Day 3: Part 1");

  let (vec1, vec2) = preprocessing(&lines);

  let result = run(&vec1, &vec2);

  println!("Result part 1: {}", result);
}

fn part2(lines: &Vec<String>) {
  println!("Day 3: Part 2");

  let (vec1, vec2) = preprocessing(&lines);

  let result = run2(&vec1, &vec2);

  println!("Result part 2: {}", result);
}

fn run(vec1: &Vec<Instruction>, vec2: &Vec<Instruction>) -> i32 {
  let cable_1 = create_point_set(vec1);
  let cable_2 = create_point_set(vec2);

  let mut closest_distance = i32::max_value();
  for point in cable_1 {
    if cable_2.contains(&point) {
      // println!("crossing: {:?}", point);
      let manhattan = point.x.abs() + point.y.abs();
      if manhattan < closest_distance {
        closest_distance = manhattan;
      }
    }
  }
  closest_distance
}

fn run2(vec1: &Vec<Instruction>, vec2: &Vec<Instruction>) -> i32 {
  let cable_1 = create_point_set2(vec1);
  let cable_2 = create_point_set2(vec2);

  let mut closest_distance = i32::max_value();
  for (point_1, distance_1) in cable_1 {
    if cable_2.contains_key(&point_1) {
      let distance_2 = cable_2.get(&point_1).unwrap();
      // println!(
      //   "crossing: {:?}, {} + {} = {}",
      //   point_1,
      //   distance_1,
      //   distance_2,
      //   distance_1 + distance_2
      // );
      let distance_sum = distance_1 + distance_2;
      if distance_sum < closest_distance {
        closest_distance = distance_sum;
      }
    }
  }
  closest_distance
}

fn create_point_set(collection: &Vec<Instruction>) -> HashSet<Point> {
  let mut points = HashSet::new();

  let mut last_point = Point { x: 0, y: 0 };
  for instruction in collection {
    let new_points = create_points(&last_point, instruction);
    for point in &new_points {
      points.insert(*point);
    }
    last_point = Point::clone(new_points.last().unwrap());
  }
  return points;
}

fn create_point_set2(collection: &Vec<Instruction>) -> HashMap<Point, i32> {
  let mut points = HashMap::new();

  let mut last_point = Point { x: 0, y: 0 };
  let mut coord_distance = 0;
  for instruction in collection {
    let new_points = create_points2(&last_point, coord_distance, instruction);
    for (point, coord_distance) in &new_points {
      if !points.contains_key(point) {
        points.insert(*point, *coord_distance);
      }
    }
    let point_tuple = new_points.last().unwrap();
    last_point = Point::clone(&point_tuple.0);
    coord_distance = point_tuple.1;
  }
  points
}

fn create_points2(start: &Point, coord_distance: i32, instr: &Instruction) -> Vec<(Point, i32)> {
  let mut new_points = Vec::new();
  let dist = instr.distance + 1;
  for len in 1..dist {
    let len_i32 = i32::try_from(len).unwrap();
    let distance = coord_distance + len_i32;

    let new_point = match instr.direction {
      Direction::Up => Point {
        x: start.x,
        y: start.y + len_i32,
      },
      Direction::Down => Point {
        x: start.x,
        y: start.y - len_i32,
      },
      Direction::Right => Point {
        x: start.x + len_i32,
        y: start.y,
      },
      Direction::Left => Point {
        x: start.x - len_i32,
        y: start.y,
      },
    };
    // println!("{} {:?} {}", len_i32, new_point, distance);
    new_points.push((new_point, distance));
  }

  new_points
}

fn create_points(start: &Point, instr: &Instruction) -> Vec<Point> {
  let mut new_points = Vec::new();
  let dist = instr.distance + 1;
  for len in 1..dist {
    let len_i32 = i32::try_from(len).unwrap();
    let new_point = match instr.direction {
      Direction::Up => Point {
        x: start.x,
        y: start.y + len_i32,
      },
      Direction::Down => Point {
        x: start.x,
        y: start.y - len_i32,
      },
      Direction::Right => Point {
        x: start.x + len_i32,
        y: start.y,
      },
      Direction::Left => Point {
        x: start.x - len_i32,
        y: start.y,
      },
    };
    // println!("{} {:?}", len_i32, new_point);
    new_points.push(new_point);
  }

  new_points
}

fn preprocessing(lines: &Vec<String>) -> (Vec<Instruction>, Vec<Instruction>) {
  // let mut program = lines[0].split(",").collect::<Vec<&str>>()
  //                       .iter().map(|l| l.parse::<usize>().unwrap())
  //                       .collect::<Vec<usize>>();

  let vec1 = parse_line(&lines[0]);
  let vec2 = parse_line(&lines[1]);

  (vec1, vec2)
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug)]
struct Instruction {
  direction: Direction,
  distance: u32,
}

fn parse_line(line: &String) -> Vec<Instruction> {
  let elements = line.split(",").collect::<Vec<&str>>();
  let mut result = Vec::new();

  println!("{:?}", elements);
  for e in &elements {
    let mut chars = e.chars();
    let char_direction = chars.next().unwrap();
    let distance = chars.collect::<String>().parse::<u32>().unwrap();
    let direction = match char_direction {
      'U' => Direction::Up,
      'D' => Direction::Down,
      'L' => Direction::Left,
      'R' => Direction::Right,
      _ => panic!("Parse error!"),
    };
    result.push(Instruction {
      direction: direction,
      distance: distance,
    });
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let mut lines = Vec::new();
    lines.push(String::from("R8,U5,L5,D3"));
    lines.push(String::from("U7,R6,D4,L4"));

    let (vec1, vec2) = preprocessing(&lines);

    println!("{:?}", vec1);

    let answer = run(&vec1, &vec2);
    assert_eq!(answer, 6);
  }
  #[test]
  fn example_2() {
    let mut lines = Vec::new();
    lines.push(String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"));
    lines.push(String::from("U62,R66,U55,R34,D71,R55,D58,R83"));

    let (vec1, vec2) = preprocessing(&lines);

    println!("{:?}", vec1);

    let answer = run(&vec1, &vec2);
    assert_eq!(answer, 159);
  }
  #[test]
  fn example_3() {
    let mut lines = Vec::new();
    lines.push(String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"));
    lines.push(String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));

    let (vec1, vec2) = preprocessing(&lines);

    println!("{:?}", vec1);

    let answer = run(&vec1, &vec2);
    assert_eq!(answer, 135);
  }

  #[test]
  fn example_1_part2() {
    let mut lines = Vec::new();
    lines.push(String::from("R8,U5,L5,D3"));
    lines.push(String::from("U7,R6,D4,L4"));

    let (vec1, vec2) = preprocessing(&lines);

    println!("{:?}", vec1);

    let answer = run2(&vec1, &vec2);
    assert_eq!(answer, 30);
  }
  #[test]
  fn example_2_part2() {
    let mut lines = Vec::new();
    lines.push(String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"));
    lines.push(String::from("U62,R66,U55,R34,D71,R55,D58,R83"));

    let (vec1, vec2) = preprocessing(&lines);

    println!("{:?}", vec1);

    let answer = run2(&vec1, &vec2);
    assert_eq!(answer, 610);
  }
  #[test]
  fn example_3_part2() {
    let mut lines = Vec::new();
    lines.push(String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"));
    lines.push(String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));

    let (vec1, vec2) = preprocessing(&lines);

    println!("{:?}", vec1);

    let answer = run2(&vec1, &vec2);
    assert_eq!(answer, 410);
  }
}
