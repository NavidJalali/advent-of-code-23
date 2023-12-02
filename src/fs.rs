use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_lines(path: &str) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines().flat_map(|line| line.ok()))
}
