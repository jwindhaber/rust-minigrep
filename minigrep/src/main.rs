use minigrep_lib;
use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let found_lines = minigrep_lib::grep(&args);

    println!("Found results:");

    for line in found_lines {
        println!("Line {}: {}", line.0, line.1 )
    }

}





