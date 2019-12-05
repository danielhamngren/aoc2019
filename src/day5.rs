use crate::utils;

pub fn day2() {
  let filename = "resources/day2_input";
  let lines: Vec<String> = read_file(filename);

  part1(&lines);
  part2(&lines);
}

fn part1(lines: &Vec<String>) {
  println!("Day 5: Part 1");

  let mut program = preprocessing(lines);
  set_initial_values(&mut program, 12, 2);

  run_intcode(&mut program);

  println!("Result part 1: {}", program[0]);
}

fn part2(lines: &Vec<String>) {
  println!("Day 5: Part 2");

  let program_input = preprocessing(lines);
  let mut program = program_input.clone();

  'outer: for noun in 0..99 {
    'inner: for verb in 0..99 {
      program = program_input.clone();
      set_initial_values(&mut program, noun, verb);
      run_intcode(&mut program);
      println!("noun {}, verb {}", noun, verb);
      let stop_criteria = 19690720;
      if program[0] == 19690720 {
        println!(
          "{} found, noun is {}, verb is {}",
          stop_criteria, noun, verb
        );
        println!("100 * noun + verb = {}", 100 * noun + verb);
        break 'outer;
      }
    }
  }
}

fn preprocessing(lines: &Vec<String>) -> Vec<usize> {
  let mut program = lines[0]
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|l| l.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  program
}

fn set_initial_values(program: &mut Vec<usize>, noun: usize, verb: usize) {
  program[1] = noun;
  program[2] = verb;
}

// fn read_file(filename: &str) -> Vec<String> {
//     let f = File::open(filename).expect("file not found");

//     let reader = BufReader::new(f);

//     // https://stackoverflow.com/a/53610493
//     // I can use unwrap because I'm sure that every line will be read correctly.
//     // I'm basing this on faith alone. Would probably be good to have some
//     // additional error checking here.
//     reader.lines().collect::<Result<_, _>>().unwrap()
// }

fn run_intcode(intcode: &mut Vec<usize>) {
  let mut done = false;
  let mut pc = 0; // short for program counter
  while !done {
    // println!("pc: {}", pc);
    match intcode[pc] {
      1 => {
        // println!("Match 1, add");
        let result = intcode[intcode[pc + 1]] + intcode[intcode[pc + 2]];
        let store_address = intcode[pc + 3];
        intcode[store_address] = result;
        pc += 4;
      }
      2 => {
        // println!("Match 2, multiply");
        let result = intcode[intcode[pc + 1]] * intcode[intcode[pc + 2]];
        let store_address = intcode[pc + 3];
        intcode[store_address] = result;
        pc += 4;
      }
      99 => {
        // println!{"Match 99, terminate"}
        done = true;
      }
      _ => {
        println!("Match error!");
      }
    }
    // Assume intcodes are correctly written,
    // no need to check for reads and writes outside of vector.
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let mut vec = vec![1, 0, 0, 0, 99];
    run_intcode(&mut vec);
    assert_eq!(vec, [2, 0, 0, 0, 99]);
  }

  #[test]
  fn example_2() {
    let mut vec = vec![2, 3, 0, 3, 99];
    run_intcode(&mut vec);
    assert_eq!(vec, [2, 3, 0, 6, 99]);
  }

  #[test]
  fn example_3() {
    let mut vec = vec![2, 4, 4, 5, 99, 0];
    run_intcode(&mut vec);
    assert_eq!(vec, [2, 4, 4, 5, 99, 9801]);
  }

  #[test]
  fn example_4() {
    let mut vec = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run_intcode(&mut vec);
    assert_eq!(vec, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }
}
