use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file.");
    println!("With text:\n{}", contents);
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