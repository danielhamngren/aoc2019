mod utils;

pub fn day3() {
    let filename = "resources/day3_input";
    let lines: Vec<String> = utils::read_file(filename);
    
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    println!("Day 2: Part 1");

    let vec1 = preprocessing(lines[0]);
    let vec2 = preprocessing(lines[1]);

    let answer = run(vec1, vec2);

    println!("Result part 1: {}", program[0]);
}

fn run(vec1: &Vec<usize>, vec2: &Vec<usize>) -> i32 {
  999
}


fn preprocessing(lines: &Vec<String>) -> Vec<usize>{
    let mut program = lines[0].split(",").collect::<Vec<&str>>()
                          .iter().map(|l| l.parse::<usize>().unwrap())
                          .collect::<Vec<usize>>();

    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let vec1_input = vec!["R8","U5","L5","D3"];
        let vec2_input = vec!["U7","R6","D4","L4"];

        let vec1 = preprocessing(vec1_input);
        let vec2 = preprocessing(vec2_input);

        let answer = run(vec1, vec2);
        assert_eq!(answer, 6);
    }
}