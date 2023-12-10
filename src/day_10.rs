use std::{collections::HashSet, fmt::Debug, io, vec};

use crate::fs::read_day;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
  x: i32,
  y: i32,
}

impl Debug for Coordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x + 1, self.y + 1)
  }
}

impl Coordinate {
  fn new(x: i32, y: i32) -> Self {
    Coordinate { x, y }
  }

  fn east(&self) -> Option<Self> {
    Some(Coordinate::new(self.x + 1, self.y))
  }

  fn west(&self) -> Option<Self> {
    if self.x == 0 {
      None
    } else {
      Some(Coordinate::new(self.x - 1, self.y))
    }
  }

  fn north(&self) -> Option<Self> {
    if self.y == 0 {
      None
    } else {
      Some(Coordinate::new(self.x, self.y - 1))
    }
  }

  fn south(&self) -> Option<Self> {
    Some(Coordinate::new(self.x, self.y + 1))
  }

  fn is_north_of(&self, other: Coordinate) -> bool {
    self.x == other.x && self.y == other.y - 1
  }

  fn is_south_of(&self, other: Coordinate) -> bool {
    self.x == other.x && self.y == other.y + 1
  }

  fn is_east_of(&self, other: Coordinate) -> bool {
    self.x == other.x + 1 && self.y == other.y
  }

  fn is_west_of(&self, other: Coordinate) -> bool {
    self.x == other.x - 1 && self.y == other.y
  }
}

#[derive(Debug)]
enum Tile {
  Ground,
  Pipe(PipeType),
  Start,
}

impl Tile {
  fn to_char(&self) -> char {
    match self {
      Tile::Ground => ' ',
      Tile::Pipe(pipe) => match pipe {
        PipeType::Vertical => '║',
        PipeType::Horizontal => '═',
        PipeType::NorthToEastBend => '╚',
        PipeType::NorthToWestBend => '╝',
        PipeType::SouthToEastBend => '╔',
        PipeType::SouthToWestBend => '╗',
      },
      Tile::Start => 'S',
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum PipeType {
  Vertical,
  Horizontal,
  NorthToEastBend,
  NorthToWestBend,
  SouthToEastBend,
  SouthToWestBend,
}

impl From<char> for Tile {
  fn from(c: char) -> Self {
    match c {
      '|' => Tile::Pipe(PipeType::Vertical),
      '-' => Tile::Pipe(PipeType::Horizontal),
      'L' => Tile::Pipe(PipeType::NorthToEastBend),
      'J' => Tile::Pipe(PipeType::NorthToWestBend),
      'F' => Tile::Pipe(PipeType::SouthToEastBend),
      '7' => Tile::Pipe(PipeType::SouthToWestBend),
      '.' => Tile::Ground,
      'S' => Tile::Start,
      _ => panic!("Invalid tile: {}", c),
    }
  }
}

#[derive(Debug)]
struct Map {
  tiles: Vec<Vec<Tile>>,
  start: Coordinate,
}

impl Map {
  fn new(tiles: Vec<Vec<Tile>>, start: Coordinate) -> Self {
    Map { tiles, start }
  }

  fn get(&self, coord: Coordinate) -> Option<&Tile> {
    self
      .tiles
      .get(coord.y as usize)
      .and_then(|row| row.get(coord.x as usize))
  }
}

fn parse_map() -> io::Result<Map> {
  let mut start: Option<Coordinate> = None;

  let rows = read_day(10)?
    .enumerate()
    .map(|(row, line)| {
      let row = line
        .trim()
        .chars()
        .enumerate()
        .map(|(column, c)| {
          let tile = Tile::from(c);
          if let Tile::Start = tile {
            start = Some(Coordinate::new(column as i32, row as i32));
          }
          tile
        })
        .collect::<Vec<_>>();
      row
    })
    .collect::<Vec<_>>();

  Ok(Map::new(rows, start.unwrap()))
}

fn next_via_pipe(
  previous: Coordinate,
  current: Coordinate,
  pipe: PipeType,
) -> Result<Coordinate, String> {
  match pipe {
    PipeType::Vertical => {
      if previous.is_north_of(current) {
        // We came from the north, so we can go south
        Ok(Coordinate::new(current.x, current.y + 1))
      } else if previous.is_south_of(current) {
        // We came from the south, so we can go north
        Ok(Coordinate::new(current.x, current.y - 1))
      } else {
        Err(format!("Invalid direction at {:?}", current))
      }
    }
    PipeType::Horizontal => {
      if previous.is_east_of(current) {
        // We came from the east, so we can go west
        Ok(Coordinate::new(current.x - 1, current.y))
      } else if previous.is_west_of(current) {
        // We came from the west, so we can go east
        Ok(Coordinate::new(current.x + 1, current.y))
      } else {
        Err(format!("Invalid direction at {:?}", current))
      }
    }
    PipeType::NorthToEastBend => {
      if previous.is_north_of(current) {
        // We came from the north, so we can go east
        Ok(Coordinate::new(current.x + 1, current.y))
      } else if previous.is_east_of(current) {
        // We came from the east, so we can go north
        Ok(Coordinate::new(current.x, current.y - 1))
      } else {
        Err(format!("Invalid direction at {:?}", current))
      }
    }
    PipeType::NorthToWestBend => {
      if previous.is_north_of(current) {
        // We came from the north, so we can go west
        Ok(Coordinate::new(current.x - 1, current.y))
      } else if previous.is_west_of(current) {
        // We came from the west, so we can go north
        Ok(Coordinate::new(current.x, current.y - 1))
      } else {
        Err(format!("Invalid direction at {:?}", current))
      }
    }
    PipeType::SouthToEastBend => {
      if previous.is_south_of(current) {
        // We came from the south, so we can go east
        Ok(Coordinate::new(current.x + 1, current.y))
      } else if previous.is_east_of(current) {
        // We came from the east, so we can go south
        Ok(Coordinate::new(current.x, current.y + 1))
      } else {
        Err(format!("Invalid direction at {:?}", current))
      }
    }
    PipeType::SouthToWestBend => {
      if previous.is_south_of(current) {
        // We came from the south, so we can go west
        Ok(Coordinate::new(current.x - 1, current.y))
      } else if previous.is_west_of(current) {
        // We came from the west, so we can go south
        Ok(Coordinate::new(current.x, current.y + 1))
      } else {
        Err(format!("Invalid direction at {:?}", current))
      }
    }
  }
}

fn possible_paths_from_start(map: &Map) -> Vec<Coordinate> {
  let start = map.start;

  let mut starting_points = vec![];

  // north
  let north = start.north();
  if let Some(Tile::Pipe(pipe)) = north.and_then(|c| map.get(c)) {
    let north = north.unwrap();
    match pipe {
      PipeType::Vertical => starting_points.push(north),
      PipeType::SouthToEastBend => starting_points.push(north),
      PipeType::SouthToWestBend => starting_points.push(north),
      _ => {}
    }
  }

  // east
  let east = start.east();
  if let Some(Tile::Pipe(pipe)) = east.and_then(|c| map.get(c)) {
    let east = east.unwrap();
    match pipe {
      PipeType::Horizontal => starting_points.push(east),
      PipeType::NorthToWestBend => starting_points.push(east),
      PipeType::SouthToWestBend => starting_points.push(east),
      _ => {}
    }
  }

  // south
  let south = start.south();
  if let Some(Tile::Pipe(pipe)) = south.and_then(|c| map.get(c)) {
    let south = south.unwrap();
    match pipe {
      PipeType::Vertical => starting_points.push(south),
      PipeType::NorthToEastBend => starting_points.push(south),
      PipeType::NorthToWestBend => starting_points.push(south),
      _ => {}
    }
  }

  // west
  let west = start.west();
  if let Some(Tile::Pipe(pipe)) = west.and_then(|c| map.get(c)) {
    let west = west.unwrap();
    match pipe {
      PipeType::Horizontal => starting_points.push(west),
      PipeType::NorthToEastBend => starting_points.push(west),
      PipeType::SouthToEastBend => starting_points.push(west),
      _ => {}
    }
  }

  starting_points
}

pub fn part_1() -> io::Result<usize> {
  let map = parse_map()?;

  let starting_points = possible_paths_from_start(&map);

  let mut path = vec![];

  for starting_point in starting_points {
    let mut previous = map.start;
    let mut current = starting_point;

    loop {
      let tile = map
        .get(current)
        .ok_or(format!("Invalid coordinate: {:?}", current))
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

      match tile {
        Tile::Ground => {
          // We hit the ground, invalid path
          path.clear();
          break;
        }
        Tile::Pipe(pipe) => {
          // We're still on a pipe, so we can continue
          let next = next_via_pipe(previous, current, *pipe)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

          path.push(next);

          previous = current;
          current = next;
        }
        Tile::Start => {
          path = [vec![map.start, starting_point], path].concat();
          // We hit the start again, valid path
          break;
        }
      }
    }

    if !path.is_empty() {
      break;
    }
  }

  Ok(path.len() / 2)
}

pub fn part_2() -> io::Result<usize> {
  let mut map = parse_map()?;

  let starting_points = possible_paths_from_start(&map);

  let mut path = vec![];

  for starting_point in starting_points {
    let mut previous = map.start;
    let mut current = starting_point;

    loop {
      let tile = map
        .get(current)
        .ok_or(format!("Invalid coordinate: {:?}", current))
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

      match tile {
        Tile::Ground => {
          // We hit the ground, invalid path
          path.clear();
          break;
        }
        Tile::Pipe(pipe) => {
          // We're still on a pipe, so we can continue
          let next = next_via_pipe(previous, current, *pipe)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

          path.push(next);

          previous = current;
          current = next;
        }
        Tile::Start => {
          path = [vec![map.start, starting_point], path].concat();
          let a = starting_point;
          let b = path[path.len() - 2];
          let s = map.start;

          map.tiles[s.y as usize][s.x as usize] = {
            if (a.is_north_of(s) && b.is_south_of(s)) || (b.is_north_of(s) && a.is_south_of(s)) {
              Tile::Pipe(PipeType::Vertical)
            } else if (a.is_east_of(s) && b.is_west_of(s)) || (b.is_east_of(s) && a.is_west_of(s)) {
              Tile::Pipe(PipeType::Horizontal)
            } else if (a.is_north_of(s) && b.is_east_of(s)) || (b.is_north_of(s) && a.is_east_of(s))
            {
              Tile::Pipe(PipeType::NorthToEastBend)
            } else if (a.is_north_of(s) && b.is_west_of(s)) || (b.is_north_of(s) && a.is_west_of(s))
            {
              Tile::Pipe(PipeType::NorthToWestBend)
            } else if (a.is_south_of(s) && b.is_east_of(s)) || (b.is_south_of(s) && a.is_east_of(s))
            {
              Tile::Pipe(PipeType::SouthToEastBend)
            } else if (a.is_south_of(s) && b.is_west_of(s)) || (b.is_south_of(s) && a.is_west_of(s))
            {
              Tile::Pipe(PipeType::SouthToWestBend)
            } else {
              panic!("Invalid start tile")
            }
          };
          // We hit the start again, valid path
          break;
        }
      }
    }

    if !path.is_empty() {
      break;
    }
  }

  let visited = path.iter().cloned().collect::<HashSet<_>>();

  let mut area_inside = 0;

  for row in 0..map.tiles.len() {
    let mut crossings = 0;
    for column in 0..map.tiles[row].len() {
      let coord = Coordinate::new(column as i32, row as i32);

      if visited.contains(&coord) {
        match map.get(coord) {
          Some(Tile::Pipe(PipeType::Vertical)) => {
            crossings += 1;
          }
          Some(Tile::Pipe(PipeType::SouthToEastBend)) => {
            crossings += 1;
          }
          Some(Tile::Pipe(PipeType::SouthToWestBend)) => {
            crossings += 1;
          }
          _ => {}
        }
        print!("{}", map.get(coord).unwrap().to_char());
      } else {
        if crossings % 2 == 1 {
          area_inside += 1;
          print!("░");
        } else {
          print!(" ");
        }
      }
    }
    print!("\n");
  }

  Ok(area_inside)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_north() {
    let a = Coordinate::new(4, 5);
    let b = Coordinate::new(4, 6);

    assert!(a.is_north_of(b));
  }
}
