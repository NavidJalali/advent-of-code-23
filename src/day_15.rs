use std::{fmt::Debug, io};

use crate::fs::read_day;

fn hash(input: &str) -> u64 {
  let mut hash = 0;
  for c in input.chars() {
    let ascii = c as u64;
    hash = (hash + ascii) * 17;
    hash = hash % 256;
  }
  hash
}

pub fn part_1() -> io::Result<u64> {
  let result = read_day(15)?
    .flat_map(|line| {
      line
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    })
    .map(|s| hash(s.trim()))
    .sum();

  Ok(result)
}

#[derive(Clone)]
struct Entry {
  label: String,
  focal_length: u8,
}

impl Debug for Entry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let string = format!("[{} {}]", self.label, self.focal_length);
    write!(f, "{}", string)
  }
}

pub fn part_2() -> io::Result<usize> {
  // An array of exactly 256 elements. Each is a vector of Entry structs.
  let mut hash_table: Vec<Vec<Entry>> = vec![vec![]; 256];

  read_day(15)?
    .flat_map(|line| {
      line
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    })
    .for_each(|s| {
      // {label}-
      if s.ends_with('-') {
        let label = s.trim_end_matches('-').to_string();
        let box_index = hash(&label);

        // remove the entry from the hash table

        hash_table[box_index as usize] = hash_table[box_index as usize]
          .iter()
          .filter(|entry| entry.label != label)
          .cloned()
          .collect::<Vec<Entry>>();
      }
      // {label}={focal_length}
      else if s.contains('=') {
        let (label, focal_length) = s.split_once('=').unwrap();
        let focal_length = focal_length.parse::<u8>().unwrap();
        let box_index = hash(label);
        let entry = Entry {
          label: label.to_string(),
          focal_length,
        };

        // update or insert
        let mut found = false;
        for (index, existing_entry) in hash_table[box_index as usize].iter_mut().enumerate() {
          if existing_entry.label == label {
            hash_table[box_index as usize][index] = entry.clone();
            found = true;
            break;
          }
        }

        if !found {
          hash_table[box_index as usize].push(entry);
        }
      } else {
        unreachable!("Invalid input: {}", s)
      }
    });

  let result = hash_table
    .iter()
    .enumerate()
    .flat_map(|(index, entries)| {
      entries
        .iter()
        .enumerate()
        .map(|(slot, entry)| (index + 1) * (slot + 1) * entry.focal_length as usize)
        .collect::<Vec<usize>>()
    })
    .sum();

  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hash_correct() {
    let string = "HASH";
    assert_eq!(hash(string), 52);
  }
}
