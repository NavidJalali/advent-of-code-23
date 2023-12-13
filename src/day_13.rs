use std::{fmt::Debug, io};

use crate::fs::read_day;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
  Ash,
  Rock,
}

impl Debug for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Tile::Ash => write!(f, "."),
      Tile::Rock => write!(f, "#"),
    }
  }
}

#[derive(Clone, PartialEq, Eq)]
struct Map {
  tiles: Vec<Vec<Tile>>,
}

impl Debug for Map {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in &self.tiles {
      for tile in row {
        write!(f, "{:?}", tile)?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

impl Map {
  fn from_file() -> io::Result<Vec<Map>> {
    let result = read_day(13)?
      .map(|line| line.trim().to_string())
      .collect::<Vec<_>>()
      .split(|line| line.is_empty())
      .map(|map| {
        map
          .iter()
          .map(|line| {
            line
              .chars()
              .map(|c| match c {
                '#' => Tile::Rock,
                '.' => Tile::Ash,
                _ => panic!("Invalid input"),
              })
              .collect()
          })
          .collect::<Vec<_>>()
      })
      .map(|tiles| Map { tiles })
      .collect::<Vec<_>>();

    Ok(result)
  }

  fn transpose(&self) -> Self {
    let current_width = self.tiles[0].len();
    let current_height = self.tiles.len();

    let mut new_tiles = vec![vec![Tile::Ash; current_height]; current_width];

    for (x, row) in self.tiles.iter().enumerate() {
      for (y, tile) in row.iter().enumerate() {
        new_tiles[y][x] = *tile;
      }
    }

    Map { tiles: new_tiles }
  }
}

fn find_mirror(map: &Map, diff: usize) -> Option<usize> {
  for i in 1..map.tiles.len() {
    // if mirror is between (i - 1) and i
    let lower = map.tiles.iter().skip(i);
    let flipped_upper = map.tiles.iter().take(i).rev();

    let differences = lower
      .zip(flipped_upper)
      .map(|(lower, upper)| {
        lower
          .iter()
          .zip(upper.iter())
          .filter(|(lower, upper)| lower != upper)
          .count()
      })
      .sum::<usize>();

    if differences == diff {
      return Some(i);
    }
  }
  None
}

pub fn part_1() -> io::Result<usize> {
  let maps = Map::from_file()?;

  let result = maps
    .iter()
    .map(|map| {
      println!("{:?}", map);
      find_mirror(&map, 0)
        .map(|m| m * 100)
        .or_else(|| find_mirror(&map.transpose(), 0))
        .unwrap()
    })
    .sum::<usize>();
  Ok(result)
}

pub fn part_2() -> io::Result<usize> {
  let maps = Map::from_file()?;

  let result = maps
    .iter()
    .map(|map| {
      println!("{:?}", map);
      find_mirror(&map, 1)
        .map(|m| m * 100)
        .or_else(|| find_mirror(&map.transpose(), 1))
        .unwrap()
    })
    .sum::<usize>();

  Ok(result)
}
