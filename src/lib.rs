use std::error::Error;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

/// Rremove annoying characters of names in a directory.
#[derive(StructOpt, Debug)]
#[structopt(name = "qesp")]
pub struct Config {
    /// Target directory
    #[structopt(default_value = ".")]
    pub dir: String,
    /// recursively attempts to rename whole directory tree
    #[structopt(long = "recursive", short = "r")]
    pub recursive: bool,
}

/// Trim annoying characters from a string
fn trim(from: &str) -> String {
    // let to: String = from
    //     .chars()
    //     .filter(|&x| (x !=' ') & (x != '(') & (x != ')')
    // ).collect();
    from.chars()
        .filter(|x| match x {
            ' ' | '(' | ')' => false,
            _ => true,
        })
        .collect()
}

/// Trim annoying characters of filenames in a directory.
///
/// # Examples
/// ```
/// use qesp::{Config, qesp};
/// # use std::fs;
/// # use std::path::Path;
///
/// # fs::create_dir(Path::new("annoying name for dir(very annoying)")).unwrap();
/// let recursive = false;
/// let dir = String::from(".");
/// qesp(Config{ dir, recursive }).unwrap();
/// # assert!(Path::new("annoyingnamefordirveryannoying").is_dir());
/// # fs::remove_dir("annoyingnamefordirveryannoying").unwrap();
/// ```
pub fn qesp(config: Config) -> Result<(), Box<dyn Error>> {
    let target = fs::read_dir(config.dir)?;
    let recursive = config.recursive;

    for file in target {
        let file = file?;
        let from = String::from(file.path().to_str().unwrap());
        let path = trim(&from);
        fs::rename(from, &path)?; // Rename a.txt to b.txt
        if Path::new(&path).is_dir() & recursive {
            let dir = String::from(path);
            qesp(Config { dir, recursive })?;
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
        let to = trim(line);
        assert_eq!("annoyingnamefordirveryannoying", to)
    }
}
