use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day1() {
    let filename = "input";
    let lines: Vec<String> = read_file(filename);
    let lines_clone = lines.clone();
    
    part1(lines);
    part2(lines_clone);
}

fn part1(lines: Vec<String>) {
    println!("Part 1");

    let input = lines.iter().map(|l| l.parse::<i32>().unwrap());
    let fuel_draw = input.map(|x| calculate_fuel(x));
    let fuel_draw_total: i32 = fuel_draw.sum();

    println!("Total fuel: {}\n", fuel_draw_total);
}

fn part2(lines: Vec<String>) {
    println!("Part 2");

    let input = lines.iter().map(|l| l.parse::<i32>().unwrap());
    let fuel_draw = input.map(|x| total_fuel(x));
    let fuel_draw_total: i32 = fuel_draw.sum();

    println!("Total fuel: {}", fuel_draw_total);
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

pub fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn total_fuel(mass: i32) -> i32 {
    let mut total = 0;
    let mut fuel_mass = calculate_fuel(mass);
    while fuel_mass > 0 {
        total += fuel_mass;
        fuel_mass = calculate_fuel(fuel_mass);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_is_twelve() {
        assert_eq!(2, calculate_fuel(12));
    }

    #[test]
    fn mass_is_fourteen() {
        assert_eq!(2, calculate_fuel(14));
    }

    #[test]
    fn mass_test_3() {
        assert_eq!(654, calculate_fuel(1969));
    }

    #[test]
    fn mass_test_4() {
        assert_eq!(33583, calculate_fuel(100756));
    }

    #[test]
    fn test_total_fuel_1() {
        assert_eq!(2, total_fuel(12));
    }

    #[test]
    fn test_total_fuel_2() {
        assert_eq!(966, total_fuel(1969));
    }

    #[test]
    fn test_total_fuel_3() {
        assert_eq!(50346, total_fuel(100756));
    }
}