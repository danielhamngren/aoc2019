use crate::utils;

pub fn day3() {
    let filename = "resources/day3_input";
    let lines: Vec<String> = utils::read_file(filename);
    
    part1(&lines);
}

fn part1(lines: &Vec<String>) {
    println!("Day 2: Part 1");

    let (vec1, vec2) = preprocessing(&lines);

    let result = run(&vec1, &vec2);

    println!("Result part 1: {}", result);
}

fn run(vec1: &Vec<&str>, vec2: &Vec<&str>) -> i32 {
  999
}

fn preprocessing(lines: &Vec<String>) -> (Vec<&str>, Vec<&str>){
    let mut program = lines[0].split(",").collect::<Vec<&str>>()
                          .iter().map(|l| l.parse::<usize>().unwrap())
                          .collect::<Vec<usize>>();

    (vec!["999", "tft"], vec!["999", "tff"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let vec1_input = vec!["R8","U5","L5","D3"];
        let vec2_input = vec!["U7","R6","D4","L4"];

        let mut lines = Vec::new();
        lines.push(String::from("R8,U5,L5,D3"));
        lines.push(String::from("U7,R6,D4,L4"));

        let (vec1, vec2) = preprocessing(&lines);

        let answer = run(&vec1, &vec2);
        assert_eq!(answer, 6);
    }
}