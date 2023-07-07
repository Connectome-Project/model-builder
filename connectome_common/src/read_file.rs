use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines())
}

#[allow(dead_code)]
pub fn assemble_relative_path(relative_path: &str) -> PathBuf {
    let path_buf = env::current_dir().unwrap();
    let current_directory = path_buf.as_path();
    let path_to_txt = Path::new(relative_path);
    let combined_path = current_directory.join(path_to_txt);
    return combined_path;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_txt_file() {
        let combined_path = assemble_relative_path("src/example.txt");

        let mut lines = read_lines(combined_path).unwrap();
        let lines_iter = lines.next().unwrap().unwrap();
        assert_eq!(lines_iter,"A connectome is a comprehensive map of neural connections in the brain, and may be thought of as its \"wiring diagram\". ");
    }
}
