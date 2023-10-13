use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines())
}

pub fn assemble_relative_path(relative_path: &str) -> PathBuf {
    let path_buf = env::current_dir().unwrap();
    let current_directory = path_buf.as_path();
    let path_to_txt = Path::new(relative_path);
    let combined_path = current_directory.join(path_to_txt);
    return combined_path;
}
