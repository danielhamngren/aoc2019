use permutator::Permutation;
use std::collections::VecDeque;

use crate::utils;

pub fn day7() {
  let filename = "resources/day7_input";
  let lines: Vec<String> = utils::read_file(filename);

  part1(&lines);
  part2(&lines);
}

fn part1(lines: &Vec<String>) {
  println!("Day 7: Part 1");

  let program = preprocessing(lines);

  let phases = vec![0, 1, 2, 3, 4];
  let result = find_max_thuster(&program, &phases);

  println!("Result part 1: {}", result);
}

fn part2(lines: &Vec<String>) {
  println!("Day 7: Part 2");

  let mut program = preprocessing(lines);
  let phases = vec![9, 8, 7, 6, 5];

  let output = find_max_thuster_feedback_loop(&program, &phases);

  println!("Result part 2: {}", output);
}

fn find_max_thuster(program: &Vec<i32>, phases: &Vec<i32>) -> i32 {
  let mut prog = program.clone();

  let mut max_thruster = 0;

  for phase_setting in create_permutations(&phases) {
    let mut output = 0;

    for setting in &phase_setting {
      output = run_intcode(&mut prog, vec![output, *setting]);
      // println!("{} {}", *setting, output);
    }

    if output > max_thruster {
      max_thruster = output;
    }
  }

  max_thruster
}

#[derive(Debug)]
struct Program {
  code: Vec<i32>,
  pc: usize,
  input: VecDeque<i32>,
  running: bool,
  output: i32,
}

fn find_max_thuster_feedback_loop(program: &Vec<i32>, phases: &Vec<i32>) -> i32 {
  let mut max_thruster = 0;
  let mut output = 0;

  for phase_setting in create_permutations(&phases) {
    let mut programA = Program {
      code: program.clone(),
      pc: 0,
      input: VecDeque::new(),
      running: true,
      output: 0,
    };
    programA.input.push_back(phase_setting[0]);
    programA.input.push_back(0);

    let mut programB = Program {
      code: program.clone(),
      pc: 0,
      input: VecDeque::new(),
      running: true,
      output: 0,
    };
    programB.input.push_back(phase_setting[1]);

    let mut programC = Program {
      code: program.clone(),
      pc: 0,
      input: VecDeque::new(),
      running: true,
      output: 0,
    };
    programC.input.push_back(phase_setting[2]);

    let mut programD = Program {
      code: program.clone(),
      pc: 0,
      input: VecDeque::new(),
      running: true,
      output: 0,
    };
    programD.input.push_back(phase_setting[3]);

    let mut programE = Program {
      code: program.clone(),
      pc: 0,
      input: VecDeque::new(),
      running: true,
      output: 0,
    };
    programE.input.push_back(phase_setting[4]);

    while programA.running
      && programB.running
      && programC.running
      && programD.running
      && programE.running
    {
      run_program(&mut programA);

      programB.input.push_back(programA.output);
      run_program(&mut programB);

      programC.input.push_back(programB.output);
      run_program(&mut programC);

      programD.input.push_back(programC.output);
      run_program(&mut programD);

      programE.input.push_back(programD.output);
      run_program(&mut programE);

      programA.input.push_back(programE.output);
    }

    if programE.output > max_thruster {
      max_thruster = programE.output;
    }
  }
  max_thruster
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

fn run_program(program: &mut Program) -> i32 {
  while program.running {
    if program.pc >= program.code.len() {
      program.running = false;
    }
    // handle immediate/position mode params
    let current_instuction = program.code[program.pc];

    let opcode = current_instuction % 100;
    let mode = vec![
      current_instuction / 100 % 10,
      current_instuction / 1000 % 10,
    ];

    match opcode {
      1 => {
        //  add
        let (first_param, second_param) = get_params(&mode, &program.code, program.pc);
        let result = first_param + second_param;
        let store_address = program.code[program.pc + 3] as usize;
        program.code[store_address] = result;
        program.pc += 4;
      }
      2 => {
        // multiply
        let (first_param, second_param) = get_params(&mode, &program.code, program.pc);
        let result = first_param * second_param;
        let store_address = program.code[program.pc + 3] as usize;
        program.code[store_address] = result;
        program.pc += 4;
      }
      3 => {
        // input instruction
        if program.input.is_empty() {
          return program.output;
        }
        let store_address = program.code[program.pc + 1] as usize;
        let next_input = program.input.pop_front().expect("No input in input vector");
        program.code[store_address] = next_input;
        program.pc += 2
      }
      4 => {
        // output instruction
        let param = get_next_param(&mode, &program.code, program.pc);
        let fetch_address = program.code[program.pc + 1] as usize;
        program.output = param;
        program.pc += 2
      }
      5 => {
        // jump-if-true
        let (first_param, second_param) = get_params(&mode, &program.code, program.pc);
        if first_param != 0 {
          program.pc = second_param as usize;
        } else {
          program.pc += 3
        }
      }
      6 => {
        // jump-if-false
        let (first_param, second_param) = get_params(&mode, &program.code, program.pc);
        if first_param == 0 {
          program.pc = second_param as usize;
        } else {
          program.pc += 3
        }
      }
      7 => {
        // less than
        let (first_param, second_param) = get_params(&mode, &program.code, program.pc);
        let store_address = program.code[program.pc + 3] as usize;

        if first_param < second_param {
          program.code[store_address] = 1;
        } else {
          program.code[store_address] = 0;
        }
        program.pc += 4;
      }
      8 => {
        // equals
        let (first_param, second_param) = get_params(&mode, &program.code, program.pc);
        let store_address = program.code[program.pc + 3] as usize;

        if first_param == second_param {
          program.code[store_address] = 1;
        } else {
          program.code[store_address] = 0;
        }
        program.pc += 4;
      }
      99 => {
        // halt
        program.running = false;
        return program.output;
      }
      _ => {
        println!("Match error!");
        program.running = false;
      }
    }
    // Assume intcodes are correctly written,
    // no need to check for reads and writes outside of vector.
  }
  program.output
}

fn run_intcode(intcode: &mut Vec<i32>, program_input: Vec<i32>) -> i32 {
  let mut done = false;
  let mut pc = 0; // short for program counter
  let mut output = 0;
  let mut input = program_input.clone();
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
    // print!("pc: {} opcode: {}, mode: {:?} ", pc, opcode, mode);

    match opcode {
      1 => {
        //  add
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        let result = first_param + second_param;
        let store_address = intcode[pc + 3] as usize;
        intcode[store_address] = result;
        pc += 4;
        // println!("{} {} {}", first_param, second_param, store_address);
      }
      2 => {
        // multiply
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        let result = first_param * second_param;
        let store_address = intcode[pc + 3] as usize;
        intcode[store_address] = result;
        pc += 4;
        // println!("{} {} {}", first_param, second_param, store_address);
      }
      3 => {
        // input instruction
        let store_address = intcode[pc + 1] as usize;
        let next_input = input.pop().expect("No input in input vector");
        intcode[store_address] = next_input;
        // println!("address {} input {}", store_address, next_input);
        pc += 2
      }
      4 => {
        // output instruction
        let param = get_next_param(&mode, intcode, pc);
        let fetch_address = intcode[pc + 1] as usize;
        output = param;
        // println!("output {}", output);
        pc += 2
      }
      5 => {
        // jump-if-true
        // let param = get_next_param(&mode, intcode, pc);
        let (first_param, second_param) = get_params(&mode, intcode, pc);
        // println!("{} {}", first_param, second_param);
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
        // println!("{} {}", first_param, second_param);
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
        // println!("{} {} {}", first_param, second_param, store_address);

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
        // println!("{} {} {}", first_param, second_param, store_address);

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

fn create_permutations(input: &Vec<i32>) -> Vec<Vec<i32>> {
  //
  let mut data = input.clone();
  let mut res = Vec::new();
  res.push(input.clone());

  data.permutation().for_each(|p| {
    // call multiple times. It'll print [2, 1, 3], [3, 1, 2],
    // [1, 3, 2], [2, 3, 1], and [3, 2, 1] respectively.
    res.push(p);
  });

  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_max_thruster_feedback_loop1() {
    let phases = vec![9, 8, 7, 6, 5];

    let program = vec![
      3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
      1005, 28, 6, 99, 0, 0, 5,
    ];

    let result = find_max_thuster_feedback_loop(&program, &phases);
    assert_eq!(result, 139629729);
  }

  #[test]
  fn test_find_max_thruster_feedback_loop2() {
    let phases = vec![9, 8, 7, 6, 5];

    let program = vec![
      3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
      54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001,
      56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];

    let result = find_max_thuster_feedback_loop(&program, &phases);
    assert_eq!(result, 18216);
  }

  #[test]
  fn test_find_max_thruster1() {
    let phases = vec![0, 1, 2, 3, 4];

    let program = vec![
      3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];

    let result = find_max_thuster(&program, &phases);
    assert_eq!(result, 43210);
  }

  #[test]
  fn test_find_max_thruster2() {
    let phases = vec![0, 1, 2, 3, 4];

    let program = vec![
      3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
      0, 0,
    ];

    let result = find_max_thuster(&program, &phases);
    assert_eq!(result, 54321);
  }

  #[test]
  fn test_find_max_thruster3() {
    let phases = vec![0, 1, 2, 3, 4];

    let program = vec![
      3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
      31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];

    let result = find_max_thuster(&program, &phases);
    assert_eq!(result, 65210);
  }

  #[test]
  fn test_permutations() {
    let input = vec![1, 2, 3];

    let result = create_permutations(&input);
    assert_eq!(result.len(), 6);
  }

  #[test]
  fn example_1() {
    let mut vec = vec![1, 0, 0, 0, 99];
    run_intcode(&mut vec, vec![1]);
    assert_eq!(vec, [2, 0, 0, 0, 99]);
  }

  #[test]
  fn example_2() {
    let mut vec = vec![2, 3, 0, 3, 99];
    run_intcode(&mut vec, vec![1]);
    assert_eq!(vec, [2, 3, 0, 6, 99]);
  }

  #[test]
  fn example_3() {
    let mut vec = vec![2, 4, 4, 5, 99, 0];
    run_intcode(&mut vec, vec![1]);
    assert_eq!(vec, [2, 4, 4, 5, 99, 9801]);
  }

  #[test]
  fn example_4() {
    let mut vec = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run_intcode(&mut vec, vec![1]);
    assert_eq!(vec, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }
  #[test]
  fn example_5() {
    let mut vec = vec![1002, 4, 3, 4, 33];
    run_intcode(&mut vec, vec![1]);
    assert_eq!(vec, [1002, 4, 3, 4, 99]);
  }
  #[test]
  fn equal_to_8_position_false() {
    let mut vec = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = run_intcode(&mut vec, vec![1]);
    assert_eq!(output, 0);
  }
  #[test]
  fn equal_to_8_position_true() {
    let mut vec = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = run_intcode(&mut vec, vec![8]);
    assert_eq!(output, 1);
  }
  #[test]
  fn equal_to_8_immediate_fale() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let output = run_intcode(&mut vec, vec![1]);
    assert_eq!(output, 0);
  }
  #[test]
  fn equal_to_8_immediate_true() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let output = run_intcode(&mut vec, vec![8]);
    assert_eq!(output, 1);
  }
  #[test]
  fn less_than_8_position_false() {
    let mut vec = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = run_intcode(&mut vec, vec![9]);
    assert_eq!(output, 0);
  }
  #[test]
  fn less_than_8_position_true() {
    let mut vec = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = run_intcode(&mut vec, vec![6]);
    assert_eq!(output, 1);
  }
  #[test]
  fn less_than_8_immediate_fale() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let output = run_intcode(&mut vec, vec![9]);
    assert_eq!(output, 0);
  }
  #[test]
  fn less_than_8_immediate_true() {
    let mut vec = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let output = run_intcode(&mut vec, vec![7]);
    assert_eq!(output, 1);
  }
  #[test]
  fn jump_check_if_zero_position_true() {
    let mut vec = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let output = run_intcode(&mut vec, vec![0]);
    assert_eq!(output, 0);
  }
  #[test]
  fn jump_check_if_zero_position_false() {
    let mut vec = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let output = run_intcode(&mut vec, vec![-3]);
    assert_eq!(output, 1);
  }
  #[test]
  fn jump_check_if_zero_immediate_true() {
    let mut vec = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    let output = run_intcode(&mut vec, vec![0]);
    assert_eq!(output, 0);
  }
  #[test]
  fn jump_check_if_zero_immediate_false() {
    let mut vec = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    let output = run_intcode(&mut vec, vec![-3]);
    assert_eq!(output, 1);
  }
  #[test]
  fn large_test_is_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let output = run_intcode(&mut vec, vec![8]);
    assert_eq!(output, 1000);
  }
  #[test]
  fn large_test_below_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let output = run_intcode(&mut vec, vec![7]);
    assert_eq!(output, 999);
  }
  #[test]
  fn large_test_above_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let output = run_intcode(&mut vec, vec![9]);
    assert_eq!(output, 1001);
  }
}
