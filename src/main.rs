use std::env;
use std::process;
use qesp::Config;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if config.dir == "--help" || config.dir == "-h" {
        println!("Script to remove annoying characters of names in a directory.
        qesp [target_dir] [-r | --recursive]
    - target_dir: default '.';
    - -r, --recursive: recursively attempts to rename whole directory tree;
    - -h, --help: prints this usage text and exit.\n")
} if let Err(e) = qesp::qesp(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}
