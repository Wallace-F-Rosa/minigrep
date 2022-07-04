use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // println!("\x1b[93mError\x1b[0m");
    // println!("\x1b[31mError\x1b[0m");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("\x1b[31mProblem parsing arguments:\x1b[0m {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("\x1b[31mApplication error:\x1b[0m {}", e);
        process::exit(1);
    }
}
