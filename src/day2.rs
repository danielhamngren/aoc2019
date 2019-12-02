use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day2() {
    let filename = "resources/day2_input";
    let lines: Vec<String> = read_file(filename);
    let lines_clone = lines.clone();
    
    part1(lines);
}

fn part1(lines: Vec<String>) {
    println!("Day 2: Part 1");

    let mut program = preprocessing(lines);

    run_intcode(&mut program);

    println!("Result part 1: {}", program[0]);


    // let input = lines.iter().map(|l| l.parse::<i32>().unwrap());
    // let fuel_draw = input.map(|x| calculate_fuel(x));
    // let fuel_draw_total: i32 = fuel_draw.sum();
}

fn preprocessing(lines: Vec<String>) -> Vec<usize>{
    let mut program = lines[0].split(",").collect::<Vec<&str>>()
                          .iter().map(|l| l.parse::<usize>().unwrap())
                          .collect::<Vec<usize>>();

    program[1] = 12;
    program[2] = 2;

    program
}

fn read_file(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("file not found");

    let reader = BufReader::new(f);

    // https://stackoverflow.com/a/53610493
    // I can use unwrap because I'm sure that every line will be read correctly.
    // I'm basing this on faith alone. Would probably be good to have some
    // additional error checking here.
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn run_intcode(intcode: &mut Vec<usize>) {
  let mut done = false;
  let mut pc = 0; // short for program counter
  while !done {
    println!("pc: {}", pc);
    match intcode[pc] {
      1 => {
        println!("Match 1, add");
        let result = intcode[intcode[pc+1]] + intcode[intcode[pc+2]];
        let store_address = intcode[pc+3];
        intcode[store_address] = result;
        pc += 4;

      },
      2 => {
        println!("Match 2, multiply");
        let result = intcode[intcode[pc+1]] * intcode[intcode[pc+2]];
        let store_address = intcode[pc+3];
        intcode[store_address] = result;
        pc += 4;
      },
      99 => {
        println!{"Match 99, terminate"}
        done = true;
      },
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
        let mut vec = vec![1,0,0,0,99];
        run_intcode(&mut vec);
        assert_eq!(vec, [2,0,0,0,99]);
    }

    #[test]
    fn example_2() {
        let mut vec = vec![2,3,0,3,99];
        run_intcode(&mut vec);
        assert_eq!(vec, [2,3,0,6,99]);
    }

     #[test]
    fn example_3() {
        let mut vec = vec![2,4,4,5,99,0];
        run_intcode(&mut vec);
        assert_eq!(vec, [2,4,4,5,99,9801]);
    }

    #[test]
    fn example_4() {
        let mut vec = vec![1,1,1,4,99,5,6,0,99];
        run_intcode(&mut vec);
        assert_eq!(vec, [30,1,1,4,2,5,6,0,99]);
    }
}