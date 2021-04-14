use std::process;

pub mod read;
pub mod config;
pub mod parse;

pub fn grep(arguments: &[String]) -> Vec<String> {
    let config = config::build_config_from_args(arguments).unwrap_or_else(|error| {
        println!("Configuration was not parsed correctly: {}", error);
        process::exit(1);
    });

    let reader = read::read_file(&config.filename).unwrap_or_else(|error| {
        println!("Could not read file correctly: {}", error);
        process::exit(1);
    });

    let found_lines = parse::find_lines(&config.search_pattern, reader);
    found_lines
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
