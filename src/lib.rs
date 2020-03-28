use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

fn parse_re(src: &str) -> Result<Regex, regex::Error> {
    Regex::new(src)
}

/// Remove annoying characters of names in a directory.
#[derive(StructOpt, Debug)]
#[structopt(name = "qesp")]
pub struct Config {
    /// Target directory
    #[structopt(default_value = ".")]
    pub dir: String,
    /// recursively attempts to rename whole directory tree
    #[structopt(long = "recursive", short = "r")]
    pub recursive: bool,
    /// regex annoying characters to be removed
    #[structopt(default_value = "[ ()]", long = "pattern", short = "p", parse(try_from_str = parse_re))]
    pub pattern: Regex,
}

/// Trim annoying characters from a string
fn trim(from: &str, pat: &Regex) -> String {
    pat.replace_all(from, "").to_string()
}

/// Process arguments and call qesp
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    qesp(config.dir, config.recursive, &config.pattern)
}

/// Trim annoying characters of filenames in a directory.
///
/// # Examples
/// ```
/// use qesp::{Config, qesp};
/// use regex::Regex;
/// # use std::fs;
/// # use std::path::Path;
///
/// # fs::create_dir(Path::new("annoying name for dir(very annoying)")).unwrap();
/// let recursive = false;
/// let dir = String::from(".");
/// qesp(dir, recursive, &Regex::new("[ ()]").unwrap()).unwrap();
/// # assert!(Path::new("annoyingnamefordirveryannoying").is_dir());
/// # fs::remove_dir("annoyingnamefordirveryannoying").unwrap();
/// ```
pub fn qesp(dir: String, recursive: bool, pattern: &Regex) -> Result<(), Box<dyn Error>> {
    let target = fs::read_dir(dir)?;

    for file in target {
        let file = file?;
        let from = String::from(file.path().to_str().unwrap());
        let path = trim(&from, pattern);
        fs::rename(from, &path)?; // Rename a.txt to b.txt
        if Path::new(&path).is_dir() & recursive {
            let dir = String::from(path);
            qesp(dir, recursive, pattern)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_line() {
        let line = "annoying name for dir(very annoying)";
        let to = trim(line, &Regex::new("[ ()]").unwrap());
        assert_eq!("annoyingnamefordirveryannoying", to)
    }
}
