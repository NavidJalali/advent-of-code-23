use std::{
  collections::{HashMap, HashSet},
  fmt::Debug,
  io,
  str::FromStr,
};

use crate::fs::read_day;

struct Position {
  x: i32,
  y: i32,
  z: i32,
}

impl Position {
  fn new(x: i32, y: i32, z: i32) -> Self {
    Self { x, y, z }
  }
}

impl Debug for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!("({}, {}, {})", self.x, self.y, self.z))
  }
}

impl FromStr for Position {
  type Err = io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split(',');
    let x = parts
      .next()
      .unwrap()
      .parse::<i32>()
      .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    let y = parts
      .next()
      .unwrap()
      .parse::<i32>()
      .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    let z = parts
      .next()
      .unwrap()
      .parse::<i32>()
      .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    Ok(Self::new(x, y, z))
  }
}

struct Brick {
  start: Position,
  end: Position,
}

impl Brick {
  fn new(start: Position, end: Position) -> Self {
    Self { start, end }
  }

  // Birds-eye view overlap
  fn overlaps_xy(&self, other: &Self) -> bool {
    let overlaps_x = self.start.x.max(other.start.x) <= self.end.x.min(other.end.x);
    let overlaps_y = self.start.y.max(other.start.y) <= self.end.y.min(other.end.y);
    overlaps_x && overlaps_y
  }

  fn height(&self) -> usize {
    (self.end.z - self.start.z + 1) as usize
  }
}

impl Debug for Brick {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!("[{:?}, {:?}]", self.start, self.end))
  }
}

impl FromStr for Brick {
  type Err = io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split('~');

    let start = parts
      .next()
      .unwrap()
      .parse::<Position>()
      .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    let end = parts
      .next()
      .unwrap()
      .parse::<Position>()
      .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    Ok(Self::new(start, end))
  }
}

fn parse_input() -> io::Result<Vec<Brick>> {
  let result = read_day(22)?
    .map(|line| line.parse::<Brick>())
    .collect::<io::Result<Vec<Brick>>>()?;

  Ok(result)
}

fn make_bricks_fall(bricks: &mut Vec<Brick>) {
  bricks.sort_by(|a, b| a.end.z.cmp(&b.end.z));

  for index in 0..bricks.len() {
    let mut max_z = 1;
    let brick = &bricks[index];
    for check_index in 0..index {
      let check = &bricks[check_index];
      if brick.overlaps_xy(check) {
        max_z = max_z.max(check.end.z + 1);
      }
    }

    bricks[index].end.z -= brick.start.z - max_z;
    bricks[index].start.z = max_z;
  }

  bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
}

struct SupportMaps {
  key_supports_values: HashMap<usize, HashSet<usize>>,
  key_is_supported_by_values: HashMap<usize, HashSet<usize>>,
}

impl From<&Vec<Brick>> for SupportMaps {
  fn from(bricks: &Vec<Brick>) -> Self {
    let mut key_supports_values = HashMap::new();
    let mut key_is_supported_by_values = HashMap::new();

    for index in 0..bricks.len() {
      key_supports_values.insert(index, HashSet::<usize>::new());
      key_is_supported_by_values.insert(index, HashSet::<usize>::new());
    }

    for (top_index, top) in bricks.iter().enumerate() {
      for (index, brick) in bricks.iter().take(top_index).enumerate() {
        // `brick` supports `top` iff they overlap on the xy plane and `brick`s end z coordinate is exactly 1 less than `top`s start z coordinate
        if brick.overlaps_xy(top) && brick.end.z + 1 == top.start.z {
          key_supports_values
            .get_mut(&index)
            .unwrap()
            .insert(top_index);

          key_is_supported_by_values
            .get_mut(&top_index)
            .unwrap()
            .insert(index);
        }
      }
    }

    Self {
      key_supports_values,
      key_is_supported_by_values,
    }
  }
}

pub fn part_1() -> io::Result<usize> {
  let mut bricks = parse_input()?;
  make_bricks_fall(&mut bricks);
  let support_maps = SupportMaps::from(&bricks);

  let mut total = 0;

  for index in 0..bricks.len() {
    let bricks_supported_by_this_brick = support_maps.key_supports_values.get(&index).unwrap();

    let is_supported_by_other_bricks =
      bricks_supported_by_this_brick
        .iter()
        .all(|supported_by_this_brick_idx| {
          support_maps
            .key_is_supported_by_values
            .get(supported_by_this_brick_idx)
            .unwrap()
            .len()
            > 1
        });

    if is_supported_by_other_bricks {
      total += 1;
    }
  }

  Ok(total)
}

fn find_all_collapsing(collapsing: &mut HashSet<usize>, support_maps: &SupportMaps, index: usize) {
  let unvisited_supported_by_index = support_maps
    .key_supports_values
    .get(&index)
    .unwrap()
    .difference(collapsing)
    .map(|x| *x)
    .collect::<Vec<usize>>();

  for supported_by_index in unvisited_supported_by_index {
    // if all bricks supporting `supported_by_index` are in `collapsing`, then `supported_by_index` is also collapsing
    let all_bricks_supporting_collpasing = support_maps
      .key_is_supported_by_values
      .get(&supported_by_index)
      .unwrap()
      .is_subset(collapsing);

    if all_bricks_supporting_collpasing {
      collapsing.insert(supported_by_index);
      find_all_collapsing(collapsing, support_maps, supported_by_index);
    }
  }
}

pub fn part_2() -> io::Result<usize> {
  let mut bricks = parse_input()?;
  make_bricks_fall(&mut bricks);
  let support_maps = SupportMaps::from(&bricks);

  let mut total = 0;
  for index in 0..bricks.len() {
    let mut collapsing = HashSet::new();
    collapsing.insert(index);
    find_all_collapsing(&mut collapsing, &support_maps, index);
    total += collapsing.len() - 1;
  }

  Ok(total)
}
