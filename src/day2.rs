use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day2() {
    let filename = "resources/day1_input";
    let lines: Vec<String> = read_file(filename);
    let lines_clone = lines.clone();
    
    part1(lines);
}

fn part1(lines: Vec<String>) {
    println!("Day 2: Part 1");

    for line in &lines {
      println!("{}", line);
    } 

    // let input = lines.iter().map(|l| l.parse::<i32>().unwrap());
    // let fuel_draw = input.map(|x| calculate_fuel(x));
    // let fuel_draw_total: i32 = fuel_draw.sum();
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

fn run_intcode(intcode: Vec<i32>) -> Vec<i32> {
  vec![2,0,0,0,99]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let vec = vec![2,0,0,0,99];
        assert_eq!(run_intcode(vec), [2,0,0,0,99]);
    }

}