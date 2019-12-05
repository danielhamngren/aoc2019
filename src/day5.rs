use crate::utils;

pub fn day5() {
  let filename = "resources/day5_input";
  let lines: Vec<String> = utils::read_file(filename);

  part1(&lines);
  part2(&lines);
}

fn part1(lines: &Vec<String>) {
  println!("Day 5: Part 1");

  let mut program = preprocessing(lines);
  let output = run_intcode(&mut program, 1);

  println!("Result part 1: {}", output);
}

fn part2(lines: &Vec<String>) {
  println!("Day 5: Part 2");

  let mut program = preprocessing(lines);

  let output = run_intcode(&mut program, 5);

  println!("Result part 2: {}", output);
}

fn preprocessing(lines: &Vec<String>) -> Vec<i32> {
  let mut program = lines[0]
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|l| l.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  program
}

fn run_intcode(intcode: &mut Vec<i32>, input: i32) -> i32 {
  let mut done = false;
  let mut pc = 0; // short for program counter
  let mut output = 0;
  while !done {
    if pc >= intcode.len() {
      done = true;
    }
    // handle immediate/position mode params
    let current_instuction = intcode[pc];

    let opcode = current_instuction % 100;
    let mode = vec![
      current_instuction / 100 % 10,
      current_instuction / 1000 % 10,
    ];
    print!("pc: {} opcode: {}, mode: {:?} ", pc, opcode, mode);

    match opcode {
      1 => {
        //  add
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        let result = first_param + second_param;
        let store_address = intcode[pc + 3] as usize;
        intcode[store_address] = result;
        pc += 4;
        println!("{} {} {}", first_param, second_param, store_address);
      }
      2 => {
        // multiply
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        let result = first_param * second_param;
        let store_address = intcode[pc + 3] as usize;
        intcode[store_address] = result;
        pc += 4;
        println!("{} {} {}", first_param, second_param, store_address);
      }
      3 => {
        // input instruction
        let store_address = intcode[pc + 1] as usize;
        intcode[store_address] = input;
        println!("address {} input {}", store_address, input);
        pc += 2
      }
      4 => {
        // output instruction
        let param = get_next_param(&mode, intcode, pc);
        let fetch_address = intcode[pc + 1] as usize;
        output = param;
        println!("output {}", output);
        pc += 2
      }
      5 => {
        // jump-if-true
        // let param = get_next_param(&mode, intcode, pc);
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        println!("{} {}", first_param, second_param);
        if first_param != 0 {
          // pc = intcode[pc + 2] as usize;
          pc = second_param as usize;
        } else {
          pc += 3
        }
      }
      6 => {
        // jump-if-false
        // let param = get_next_param(&mode, intcode, pc);
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        println!("{} {}", first_param, second_param);
        if first_param == 0 {
          // pc = intcode[pc + 2] as usize;
          pc = second_param as usize;
        } else {
          pc += 3
        }
      }
      7 => {
        // less than
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        let store_address = intcode[pc + 3] as usize;
        println!("{} {} {}", first_param, second_param, store_address);

        if first_param < second_param {
          intcode[store_address] = 1;
        } else {
          intcode[store_address] = 0;
        }
        pc += 4;
      }
      8 => {
        // equals
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        let store_address = intcode[pc + 3] as usize;
        println!("{} {} {}", first_param, second_param, store_address);

        if first_param == second_param {
          intcode[store_address] = 1;
        } else {
          intcode[store_address] = 0;
        }
        pc += 4;
      }
      99 => {
        // halt
        done = true;
      }
      _ => {
        println!("Match error!");
        done = true;
      }
    }
    // Assume intcodes are correctly written,
    // no need to check for reads and writes outside of vector.
  }
  output
}

fn get_next_param(mode: &Vec<i32>, intcode: &Vec<i32>, pc: usize) -> (i32) {
  match mode[0] {
    0 => intcode[intcode[pc + 1] as usize],
    1 => intcode[pc + 1],
    _ => {
      println!("mode match error");
      0
    }
  }
}

fn get_params(mode: &Vec<i32>, intcode: &Vec<i32>, pc: usize) -> (i32, i32) {
  let first_param = match mode[0] {
    0 => intcode[intcode[pc + 1] as usize],
    1 => intcode[pc + 1],
    _ => {
      println!("mode match error");
      0
    }
  };
  let second_param = match mode[1] {
    0 => intcode[intcode[pc + 2] as usize],
    1 => intcode[pc + 2],
    _ => {
      println!("mode match error");
      0
    }
  };
  (first_param, second_param)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let mut vec = vec![1, 0, 0, 0, 99];
    run_intcode(&mut vec, 1);
    assert_eq!(vec, [2, 0, 0, 0, 99]);
  }

  #[test]
  fn example_2() {
    let mut vec = vec![2, 3, 0, 3, 99];
    run_intcode(&mut vec, 1);
    assert_eq!(vec, [2, 3, 0, 6, 99]);
  }

  #[test]
  fn example_3() {
    let mut vec = vec![2, 4, 4, 5, 99, 0];
    run_intcode(&mut vec, 1);
    assert_eq!(vec, [2, 4, 4, 5, 99, 9801]);
  }

  #[test]
  fn example_4() {
    let mut vec = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run_intcode(&mut vec, 1);
    assert_eq!(vec, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }
  #[test]
  fn example_5() {
    let mut vec = vec![1002, 4, 3, 4, 33];
    run_intcode(&mut vec, 1);
    assert_eq!(vec, [1002, 4, 3, 4, 99]);
  }
  #[test]
  fn equal_to_8_position_false() {
    let mut vec = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = run_intcode(&mut vec, 1);
    assert_eq!(output, 0);
  }
  #[test]
  fn equal_to_8_position_true() {
    let mut vec = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = run_intcode(&mut vec, 8);
    assert_eq!(output, 1);
  }
  #[test]
  fn equal_to_8_immediate_fale() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let output = run_intcode(&mut vec, 1);
    assert_eq!(output, 0);
  }
  #[test]
  fn equal_to_8_immediate_true() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let output = run_intcode(&mut vec, 8);
    assert_eq!(output, 1);
  }
  #[test]
  fn less_than_8_position_false() {
    let mut vec = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = run_intcode(&mut vec, 9);
    assert_eq!(output, 0);
  }
  #[test]
  fn less_than_8_position_true() {
    let mut vec = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = run_intcode(&mut vec, 6);
    assert_eq!(output, 1);
  }
  #[test]
  fn less_than_8_immediate_fale() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let output = run_intcode(&mut vec, 9);
    assert_eq!(output, 0);
  }
  #[test]
  fn less_than_8_immediate_true() {
    let mut vec = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let output = run_intcode(&mut vec, 7);
    assert_eq!(output, 1);
  }
  #[test]
  fn jump_check_if_zero_position_true() {
    let mut vec = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let output = run_intcode(&mut vec, 0);
    assert_eq!(output, 0);
  }
  #[test]
  fn jump_check_if_zero_position_false() {
    let mut vec = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let output = run_intcode(&mut vec, -3);
    assert_eq!(output, 1);
  }
  #[test]
  fn jump_check_if_zero_immediate_true() {
    let mut vec = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    let output = run_intcode(&mut vec, 0);
    assert_eq!(output, 0);
  }
  #[test]
  fn jump_check_if_zero_immediate_false() {
    let mut vec = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    let output = run_intcode(&mut vec, -3);
    assert_eq!(output, 1);
  }
  #[test]
  fn large_test_is_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let output = run_intcode(&mut vec, 8);
    assert_eq!(output, 1000);
  }
  #[test]
  fn large_test_below_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let output = run_intcode(&mut vec, 7);
    assert_eq!(output, 999);
  }
  #[test]
  fn large_test_above_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let output = run_intcode(&mut vec, 9);
    assert_eq!(output, 1001);
  }
}
