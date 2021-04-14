use std::fs::File;
use std::io::BufReader;

pub fn read_file(filename: &str) -> Result<BufReader<File>, &str> {

    let file = match File::open(filename) {
        Ok(file) => file,
        _ => return Err("Could not open file"),
    };

    Ok(BufReader::new(file))
}