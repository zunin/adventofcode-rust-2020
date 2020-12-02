use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

pub fn get_input_txt() -> Vec<String> {
    let file = File::open(env::current_dir().unwrap().join("src").join("input.txt")).unwrap();

    BufReader::new(file).lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}