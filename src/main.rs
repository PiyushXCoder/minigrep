use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = minigrep::Configs::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Error: {}", e);
        process::exit(0);
    }
}
