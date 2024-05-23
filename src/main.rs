use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let cli_arguments: Vec<String> = env::args().collect();
    let config = Config::build(&cli_arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
