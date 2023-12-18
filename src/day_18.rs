use crate::fs::read_day;
use std::io;

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn from_single_char(c: &char) -> Option<Self> {
    match c {
      'U' => Some(Direction::Up),
      'D' => Some(Direction::Down),
      'L' => Some(Direction::Left),
      'R' => Some(Direction::Right),
      _ => None,
    }
  }
}

#[derive(Debug, Copy, Clone)]
struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn new(x: i64, y: i64) -> Self {
    Self { x, y }
  }

  fn move_in_direction(&self, dir: Direction, count: i64) -> Self {
    let Point { x, y } = self;
    match dir {
      Direction::Up => Self::new(*x, *y - count),
      Direction::Down => Self::new(*x, *y + count),
      Direction::Left => Self::new(*x - count, *y),
      Direction::Right => Self::new(*x + count, *y),
    }
  }
}

fn parse_input_part_1() -> io::Result<Vec<((Direction, i64), (Direction, i64))>> {
  let result = read_day(18)?
    .map(|line| {
      let [dir, count_pt_1, pt_2_input] = line.split(' ').collect::<Vec<_>>()[..3] else {
        panic!("Expected 3 elements")
      };
      let dir = dir.chars().collect::<Vec<_>>()[0];
      let dir = Direction::from_single_char(&dir).expect("Expected valid direction");

      let count = count_pt_1.parse::<i64>().expect("Expected valid count");

      let pt_1 = (dir, count);

      // (#2f4433)
      // Count is 5 first 5 digits as hex
      let pt_2_count = &pt_2_input[2..7];
      let pt_2_count = i64::from_str_radix(pt_2_count, 16).expect("Expected valid hex");

      // Direction is last digit, 0 -> R, 1 -> D, 2 -> L, 3 -> U
      let pt_2_dir = &pt_2_input[7..8];
      let pt_2_dir = match pt_2_dir {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("Expected valid direction"),
      };

      let pt_2 = (pt_2_dir, pt_2_count);

      (pt_1, pt_2)
    })
    .collect();
  Ok(result)
}

// Shoelace formula

// Xi-1 Yi-1
// Xi   Yi
// Xi+1 Yi+1

// ... + (Xi-1) * (Yi) - (Xi) * (Yi-1) + Xi * (Yi+1) - (Xi+1) * (Yi) + ...

// Sum over i of Xi * (Yi+1 - Yi-1)
fn shoelace_area(points: &Vec<Point>) -> i64 {
  let length = points.len();
  let mut area: i64 = 0;
  for i in 0..length {
    area +=
      points[i].x * ((points[(i + 1 + length) % length].y) - (points[(i + length - 1) % length].y));
  }

  area.abs() / 2
}

pub fn part_1() -> io::Result<i64> {
  let input = parse_input_part_1()?;

  let mut points = vec![Point::new(0, 0)];

  let mut boundary = 0;

  for ((dir, count), _) in input {
    boundary += count;
    let last_point = points.last_mut().unwrap();
    let next_point = last_point.move_in_direction(dir, count);
    points.push(next_point);
  }

  let area = shoelace_area(&points);

  // Picks theorem:
  // A = i + b/2 - 1    ->
  // i = A - b/2 + 1

  let interior = area - boundary / 2 + 1;

  Ok(interior + boundary)
}

pub fn part_2() -> io::Result<i64> {
  let input = parse_input_part_1()?;

  let mut points = vec![Point::new(0, 0)];

  let mut boundary = 0;

  for (_, (dir, count)) in input {
    boundary += count;
    let last_point = points.last_mut().unwrap();
    let next_point = last_point.move_in_direction(dir, count);
    points.push(next_point);
  }

  let area = shoelace_area(&points);

  let interior = area - boundary / 2 + 1;

  Ok(interior + boundary)
}
