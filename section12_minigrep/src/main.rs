use std::env;
use std::process;

use section12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:\n  {}", err);
        process::exit(1);
    });
    if let Err(e) = section12_minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
