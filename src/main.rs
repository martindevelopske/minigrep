use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("There was a problem reading the file: {}", err);
        process::exit(1);
    });
    println!("{:?}", args);
    println!("searching for {} ", config.query);
    println!("in file {} ", config.file);
    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
