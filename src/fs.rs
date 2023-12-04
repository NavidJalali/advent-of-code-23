use std::{
  fs::File,
  io::{self, BufRead},
};

pub fn read_day(day: u8) -> io::Result<impl Iterator<Item = String>> {
  let file = File::open(format!("input/day_{}.txt", day))?;
  Ok(io::BufReader::new(file).lines().flat_map(|line| line.ok()))
}
