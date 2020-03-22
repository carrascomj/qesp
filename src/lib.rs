use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Config {
    pub dir: String,
    pub recursive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let dir = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query!"),
        };

        let recursive = env::var("RECURSIVE").is_err();

        Ok(Config { dir, recursive })
    }
}

fn trim(from: &str) -> std::io::Result<String> {
    let to: String = from
        .chars()
        .filter(|&x| (x !=' ') | (x != '(') | (x != ')')
    ).collect();
    fs::rename(from, &to)?; // Rename a.txt to b.txt
    Ok(to)
}

pub fn qesp(config: Config) -> Result<(), Box<dyn Error>> {
    let target = fs::read_dir(config.dir)?;
    let recursive = config.recursive;

    for file in target {
        let file = file?;
        let mut path = String::from(file.path().to_str().unwrap());
        path = trim(&path)?;
        if Path::new(&path).is_dir() & recursive {
            let dir = String::from(path);
            qesp(Config { dir, recursive })?;
        }
    }
    Ok(())
}
