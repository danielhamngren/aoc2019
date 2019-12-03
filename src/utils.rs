use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("file not found");

    let reader = BufReader::new(f);

    // https://stackoverflow.com/a/53610493
    // I can use unwrap because I'm sure that every line will be read correctly.
    // I'm basing this on faith alone. Would probably be good to have some
    // additional error checking here.
    reader.lines().collect::<Result<_, _>>().unwrap()
}