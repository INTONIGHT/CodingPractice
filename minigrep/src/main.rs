use std::env;
use std::fs;
use std::error::Error;
use minigrep::Config;

//this is for being able to do a cargo run with -- 
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
   /**
    * let contents = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });
    */
    
    if let Err(e) = minigrep::run(config) {
        println!("application error: {e}");
        process::exit(1);
    }
}

