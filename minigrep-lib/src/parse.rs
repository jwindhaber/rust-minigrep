use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn find_lines(search_pattern: &str, reader: BufReader<File>) -> Vec<(usize,String)> {

    let found_lines: Vec<(usize,String)> = reader.lines()
        .enumerate()
        .map(|(pos, res) | (pos, res.unwrap()))
        .filter(|(_, res) | res.contains(search_pattern) )
        .collect();

    return found_lines;
}

