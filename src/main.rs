use std::env;
use std::process;

fn main() {
    let mut args = env::args();

    let conf = minigrep::Configs::new(&mut args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
