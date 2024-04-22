// Taken from Chapter 12 of The Rust Programming Language
use std::env;
use std::process;
use ch12;

fn main() {
    // get args from CLI
    let args: Vec<String> = env::args().collect(); // env.args() returns an iterator
    let config = ch12::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });
    println!("Searching for `{}` in file `{}`...", config.query, config.file_path);

    // run main program
    if let Err(e) = ch12::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
