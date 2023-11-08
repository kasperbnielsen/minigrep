use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(cfg) {
        eprintln!("Apllication error {err}");
        process::exit(1);
    }
}
