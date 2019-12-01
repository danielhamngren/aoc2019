use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let filename = "input";
    println!("In file {}", filename);

    let f = File::open(filename).expect("file not found");

    let reader = BufReader::new(f);

    // https://stackoverflow.com/a/53610493
    // I can use unwrap because I'm sure that every line will be read correctly.
    // I'm basing this on faith alone. Would probably be good to have some
    // additional error checking here.
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let input = lines.iter().map(|l| l.parse::<i32>().unwrap());

    let fuel_draw = input.map(|x| calculate_fuel(x));

    let fuel_draw_total: i32 = fuel_draw.sum();

    println!("Total fuel: {}", fuel_draw_total);
}

pub fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
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
}