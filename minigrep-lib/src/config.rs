#[derive(Debug)]
pub struct Config {
    pub search_pattern: String,
    pub filename: String,
}


pub fn build_config_from_args(args: &[String]) -> Result<Config, &'static str> {

    if args.len() < 2 {
        return Err("not enough arguments provided")
    }

    let search_pattern = match args.first() {
        Some(pattern) => pattern,
        None => return Err("Search pattern not provided"),
    }.clone();

    let filename = match args.get(1) {
        Some(pattern) => pattern,
        None => return Err("Search pattern not provided"),
    }.clone();

    return Ok(Config {search_pattern, filename});

}


#[cfg(test)]
mod tests {
    use crate::config::build_config_from_args;

    #[test]
    fn produces_correct_config() {
        let arguments = [String::from("query"), String::from("/some/path.file")];
        assert!(build_config_from_args(&arguments).is_ok());
    }

    #[test]
    fn produces_error() {
        let arguments = [String::from("query")];
        assert!(build_config_from_args(&arguments).is_err());
    }

}