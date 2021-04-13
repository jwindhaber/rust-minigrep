
#[derive(Debug)]
pub struct Config {
    search_pattern: String,
    filename: String,
}

pub fn parse_grep_arguments(mut args: std::env::Args) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments provided")
    }

    // lets ignore the first one
    args.next();

    let search_pattern = match args.next() {
        Some(pattern) => pattern,
        None => return Err("Search pattern not provided"),
    };

    let filename = match args.next() {
        Some(pattern) => pattern,
        None => return Err("Search pattern not provided"),
    };

    return Ok(Config {search_pattern, filename});

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
