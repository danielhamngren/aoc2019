use permutator::Permutation;
use std::collections::{HashMap, VecDeque};

use crate::utils;

// TODO:
// 1) Store program in dictionary Check!
// 2) Test if i64 is large enough integer
// 3) Output should be VecDeque. Check!
// 4) Implement instruction 9 Check!
// 5) Change old unit tests Check!

pub fn day9() {
  let filename = "resources/day9_input";
  let lines: Vec<String> = utils::read_file(filename);

  part1(&lines);
  part2(&lines);
}

fn part1(lines: &Vec<String>) {
  println!("Day 9: Part 1");

  let mut code = preprocessing_i64(lines);

  let mut program = prepare_program_i64(&mut code, &vec![1]);
  run_program(&mut program);

  for element in program.output {
    println!("output: {}", element);
  }
}

fn part2(lines: &Vec<String>) {
  println!("Day 9: Part 2");

  let mut code = preprocessing_i64(lines);

  let mut program = prepare_program_i64(&mut code, &vec![2]);
  run_program(&mut program);

  for element in program.output {
    println!("output: {}", element);
  }
}

fn prepare_program(code: &Vec<i32>, inputs: &Vec<i32>) -> Program {
  let mut program = Program {
    code: HashMap::new(),
    pc: 0,
    input: VecDeque::new(),
    running: true,
    output: VecDeque::new(),
    relative_base: 0,
  };
  for (i, instruction) in code.iter().enumerate() {
    program.code.insert(i as i64, *instruction as i64);
  }

  for input in inputs {
    program.input.push_back(*input as i64);
  }
  program
}

fn prepare_program_i64(code: &Vec<i64>, inputs: &Vec<i64>) -> Program {
  let mut program = Program {
    code: HashMap::new(),
    pc: 0,
    input: VecDeque::new(),
    running: true,
    output: VecDeque::new(),
    relative_base: 0,
  };
  for (i, instruction) in code.iter().enumerate() {
    program.code.insert(i as i64, *instruction);
  }

  for input in inputs {
    program.input.push_back(*input);
  }
  program
}

fn find_max_thuster(code: &Vec<i32>, phases: &Vec<i32>) -> i64 {
  let mut max_thruster = 0;

  for phase_setting in create_permutations(&phases) {
    let mut output = 0_i64;

    for setting in &phase_setting {
      let mut program = prepare_program(code, &vec![*setting, output as i32]);

      run_program(&mut program);
      output = program.output.pop_front().expect("output error");
    }

    if output > max_thruster {
      max_thruster = output;
    }
  }

  max_thruster
}

#[derive(Debug)]
struct Program {
  code: HashMap<i64, i64>,
  pc: i64,
  input: VecDeque<i64>,
  running: bool,
  output: VecDeque<i64>,
  relative_base: i64,
}

fn find_max_thuster_feedback_loop(program_code: &Vec<i32>, phases: &Vec<i32>) -> i64 {
  let mut max_thruster = 0;

  for phase_setting in create_permutations(&phases) {
    let mut programs: Vec<Program> = Vec::new();
    for i in 0..5 {
      programs.push(prepare_program(program_code, &vec![]));
      programs[i].input.push_back(phase_setting[i] as i64);
    }
    programs[0].input.push_back(0);

    let mut output = 0;
    while programs[4].running {
      for i in 0..programs.len() {
        run_program(&mut programs[i]);
        let length = programs.len();
        output = programs[i].output.pop_front().expect("Output queue erros");
        programs[(i + 1) % length].input.push_back(output);
      }
    }

    if output > max_thruster {
      max_thruster = output;
    }
  }
  max_thruster
}

fn preprocessing(lines: &Vec<String>) -> Vec<i32> {
  let program = lines[0]
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|l| l.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  program
}

fn preprocessing_i64(lines: &Vec<String>) -> Vec<i64> {
  let program = lines[0]
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|l| l.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();

  program
}

fn run_program(program: &mut Program) {
  while program.running {
    // handle immediate/position mode params
    let current_instuction = program.code.get(&program.pc).expect("Memory access error");

    let opcode = current_instuction % 100;
    let mode = vec![
      current_instuction / 100 % 10,
      current_instuction / 1000 % 10,
      current_instuction / 10000 % 10,
    ];

    // println!("");
    // println!("opcode: {}", opcode);
    // println!("{:?}", program);
    match opcode {
      1 => {
        //  add
        let (first_param, second_param) =
          get_params(&mode, &program.code, program.pc, program.relative_base);
        let result = first_param + second_param;
        let mut store_address = *program
          .code
          .get(&(program.pc + 3))
          .expect("Memory access error");
        if mode[2] == 2 {
          store_address += program.relative_base;
        }
        program.code.insert(store_address, result);
        program.pc += 4;
      }
      2 => {
        // multiply
        let (first_param, second_param) =
          get_params(&mode, &program.code, program.pc, program.relative_base);
        let result = first_param * second_param;
        let mut store_address = *program
          .code
          .get(&(program.pc + 3))
          .expect("Memory access error");
        if mode[2] == 2 {
          store_address += program.relative_base;
        }
        program.code.insert(store_address, result);
        program.pc += 4;
      }
      3 => {
        // input instruction
        // println!("{:?}", program);
        if program.input.is_empty() {
          return;
        }
        let mut store_address = *program
          .code
          .get(&(program.pc + 1))
          .expect("Memory access error");
        if mode[0] == 2 {
          store_address += program.relative_base;
        }

        let next_input = program.input.pop_front().expect("No input in input vector");
        program.code.insert(store_address, next_input);
        program.pc += 2;
      }
      4 => {
        // output instruction
        let param = get_next_param(&mode, &program.code, program.pc, program.relative_base);
        program.output.push_back(param);
        program.pc += 2
      }
      5 => {
        // jump-if-true
        let (first_param, second_param) =
          get_params(&mode, &program.code, program.pc, program.relative_base);
        if first_param != 0 {
          program.pc = second_param;
        } else {
          program.pc += 3
        }
      }
      6 => {
        // jump-if-false
        let (first_param, second_param) =
          get_params(&mode, &program.code, program.pc, program.relative_base);
        if first_param == 0 {
          program.pc = second_param;
        } else {
          program.pc += 3
        }
      }
      7 => {
        // less than
        let (first_param, second_param) =
          get_params(&mode, &program.code, program.pc, program.relative_base);
        let mut store_address = *program
          .code
          .get(&(program.pc + 3))
          .expect("Memory access error");
        if mode[2] == 2 {
          store_address += program.relative_base;
        }

        if first_param < second_param {
          program.code.insert(store_address, 1);
        } else {
          program.code.insert(store_address, 0);
        }
        program.pc += 4;
      }
      8 => {
        // equals
        let (first_param, second_param) =
          get_params(&mode, &program.code, program.pc, program.relative_base);
        let mut store_address = *program
          .code
          .get(&(program.pc + 3))
          .expect("Memory access error");

        if mode[2] == 2 {
          store_address += program.relative_base;
        }

        if first_param == second_param {
          program.code.insert(store_address, 1);
        } else {
          program.code.insert(store_address, 0);
        }
        program.pc += 4;
      }
      9 => {
        // set relative base
        let param = get_next_param(&mode, &program.code, program.pc, program.relative_base);
        program.relative_base += param;
        program.pc += 2
      }
      99 => {
        // halt
        program.running = false;
        return;
      }
      _ => {
        println!("Match error! opcode: {}", opcode);
        program.running = false;
      }
    }
    // Assume intcodes are correctly written,
    // no need to check for reads and writes outside of vector.
  }
}

fn get_next_param(mode: &Vec<i64>, intcode: &HashMap<i64, i64>, pc: i64, rel_base: i64) -> (i64) {
  match mode[0] {
    0 => *intcode
      .get(&(intcode.get(&(pc + 1)).unwrap_or(&0)))
      .unwrap_or(&0),
    1 => *intcode.get(&(pc + 1)).unwrap_or(&0),
    2 => *intcode
      .get(&((intcode.get(&(pc + 1)).unwrap_or(&0)) + rel_base))
      .unwrap_or(&0),
    _ => {
      println!("mode match error");
      0
    }
  }
}

fn get_params(mode: &Vec<i64>, intcode: &HashMap<i64, i64>, pc: i64, rel_base: i64) -> (i64, i64) {
  // println!("mode: {:?}", mode);
  let first_param = match mode[0] {
    0 => {
      let param_address = pc + 1;

      // println!("param_addr: {}", param_address);

      let param = intcode.get(&param_address).unwrap_or(&0);

      // println!("param: {}", param);
      // *param
      *intcode.get(&param).unwrap_or(&0) //expect("memory read error")
    }
    1 => *intcode.get(&(pc + 1)).unwrap_or(&0),
    2 => *intcode
      .get(&((intcode.get(&(pc + 1)).unwrap_or(&0)) + rel_base))
      .unwrap_or(&0),
    _ => {
      println!("mode match error");
      0
    }
  };
  let second_param = match mode[1] {
    0 => *intcode
      .get(&(intcode.get(&(pc + 2)).unwrap_or(&0)))
      .unwrap_or(&0),
    1 => *intcode.get(&(pc + 2)).unwrap_or(&0),
    2 => *intcode
      .get(&((intcode.get(&(pc + 2)).unwrap_or(&0)) + rel_base))
      .unwrap_or(&0),
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
  fn test_203_instruction() {
    let mut vec = vec![109_i64, 5, 203, 0, 99];
    let mut program = prepare_program_i64(&mut vec, &vec![44]);
    run_program(&mut program);

    println!("{:?}", program.code);
    let expected = [109_i64, 5, 203, 0, 99, 44];
    for (k, v) in &program.code {
      assert_eq!(v, expected.get(*k as usize).unwrap());
    }
  }

  #[test]
  fn test_large_number_parameter() {
    let mut vec = vec![104, 1125899906842624_i64, 99];
    let mut program = prepare_program_i64(&mut vec, &vec![]);
    run_program(&mut program);

    assert_eq!(program.output.pop_front().unwrap(), 1125899906842624_i64);
  }

  #[test]
  fn test_multiplication_of_large_number() {
    let mut vec = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut program = prepare_program(&mut vec, &vec![]);
    run_program(&mut program);

    assert_eq!(program.output.pop_front().unwrap(), 1219070632396864_i64);
  }
  #[test]
  fn copy_self_to_output() {
    let mut vec = vec![
      109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut program = prepare_program(&mut vec, &vec![1]);
    run_program(&mut program);

    println!("output: {:?}", program.output);
    for (k, v) in program.output.iter().enumerate() {
      assert_eq!(*v, *vec.get(k as usize).unwrap() as i64);
    }
  }

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
    let mut program = prepare_program(&mut vec, &vec![1]);
    run_program(&mut program);

    let expected = [2, 0, 0, 0, 99];
    for (k, v) in &program.code {
      assert_eq!(v, expected.get(*k as usize).unwrap());
    }
  }

  #[test]
  fn example_2() {
    let mut vec = vec![2, 3, 0, 3, 99];
    let mut program = prepare_program(&mut vec, &vec![1]);
    run_program(&mut program);
    let expected = [2, 3, 0, 6, 99];
    for (k, v) in &program.code {
      assert_eq!(v, expected.get(*k as usize).unwrap());
    }
  }

  #[test]
  fn example_3() {
    let mut vec = vec![2, 4, 4, 5, 99, 0];
    let mut program = prepare_program(&mut vec, &vec![1]);
    run_program(&mut program);
    let expected = [2, 4, 4, 5, 99, 9801];
    for (k, v) in &program.code {
      assert_eq!(v, expected.get(*k as usize).unwrap());
    }
  }

  #[test]
  fn example_4() {
    let mut vec = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let mut program = prepare_program(&mut vec, &vec![1]);
    run_program(&mut program);
    let expected = [30, 1, 1, 4, 2, 5, 6, 0, 99];
    for (k, v) in &program.code {
      assert_eq!(v, expected.get(*k as usize).unwrap());
    }
  }
  #[test]
  fn example_5() {
    let mut vec = vec![1002, 4, 3, 4, 33];
    let mut program = prepare_program(&mut vec, &vec![1]);
    run_program(&mut program);
    let expected = vec![1002, 4, 3, 4, 99];
    for (k, v) in &program.code {
      assert_eq!(v, expected.get(*k as usize).unwrap());
    }
  }
  #[test]
  fn equal_to_8_position_false() {
    let mut vec = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut program = prepare_program(&mut vec, &vec![1]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 0);
  }
  #[test]
  fn equal_to_8_position_true() {
    let mut vec = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut program = prepare_program(&mut vec, &vec![8]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 1);
  }
  #[test]
  fn equal_to_8_immediate_fale() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut program = prepare_program(&mut vec, &vec![1]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 0);
  }
  #[test]
  fn equal_to_8_immediate_true() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut program = prepare_program(&mut vec, &vec![8]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 1);
  }
  #[test]
  fn less_than_8_position_false() {
    let mut vec = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut program = prepare_program(&mut vec, &vec![9]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 0);
  }
  #[test]
  fn less_than_8_position_true() {
    let mut vec = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut program = prepare_program(&mut vec, &vec![6]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 1);
  }
  #[test]
  fn less_than_8_immediate_fale() {
    let mut vec = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut program = prepare_program(&mut vec, &vec![9]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 0);
  }
  #[test]
  fn less_than_8_immediate_true() {
    let mut vec = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let mut program = prepare_program(&mut vec, &vec![7]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 1);
  }
  #[test]
  fn jump_check_if_zero_position_true() {
    let mut vec = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let mut program = prepare_program(&mut vec, &vec![0]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 0);
  }
  #[test]
  fn jump_check_if_zero_position_false() {
    let mut vec = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let mut program = prepare_program(&mut vec, &vec![-3]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 1);
  }
  #[test]
  fn jump_check_if_zero_immediate_true() {
    let mut vec = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    let mut program = prepare_program(&mut vec, &vec![0]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 0);
  }
  #[test]
  fn jump_check_if_zero_immediate_false() {
    let mut vec = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    let mut program = prepare_program(&mut vec, &vec![-3]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 1);
  }
  #[test]
  fn large_test_is_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let mut program = prepare_program(&mut vec, &vec![8]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 1000);
  }
  #[test]
  fn large_test_below_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let mut program = prepare_program(&mut vec, &vec![7]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 999);
  }
  #[test]
  fn large_test_above_8() {
    let mut vec = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let mut program = prepare_program(&mut vec, &vec![9]);
    run_program(&mut program);
    assert_eq!(program.output.pop_back().expect("output error"), 1001);
  }
}
