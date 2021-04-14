use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn find_lines(search_pattern: &str, reader: BufReader<File>) -> Vec<String> {

    let found_lines: Vec<String> = reader.lines()
        .map(|result| result.unwrap())
        .filter(|line| line.contains(search_pattern))
        .collect();

    found_lines
}