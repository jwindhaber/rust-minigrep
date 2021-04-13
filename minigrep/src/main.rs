use std::env;
use minigrep_lib;

fn main() {
    let arguments = minigrep_lib::parse_grep_arguments(env::args());

    println!("Arguments: {:?}", arguments);
}

