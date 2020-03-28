use qesp::Config;
use std::process;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    if let Err(e) = qesp::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
