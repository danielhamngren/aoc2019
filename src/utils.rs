use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// terminal_hermit: @namatoj my structure is day/language/[01]/, so i have multiple projects for each day. it's not always the best because the two puzzles of a day might share code, but i don't think you need to share code between puzzles of different days.
// but if you do want to to share code in your type of structure you can put a pub before every struct/function you want to use in another file, put a mod util; in your main.rs and a use crate::util::somefunction; in a file you want to use a function from util.rs

pub fn read_file(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("file not found");

    let reader = BufReader::new(f);

    // https://stackoverflow.com/a/53610493
    // I can use unwrap because I'm sure that every line will be read correctly.
    // I'm basing this on faith alone. Would probably be good to have some
    // additional error checking here.
    reader.lines().collect::<Result<_, _>>().unwrap()
}