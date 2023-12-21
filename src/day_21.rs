use std::{
  collections::{HashSet, VecDeque},
  fmt::Debug,
  io
};

use crate::fs::read_day;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  fn up(&self) -> Self {
    Self::new(self.x, self.y - 1)
  }

  fn down(&self) -> Self {
    Self::new(self.x, self.y + 1)
  }

  fn left(&self) -> Self {
    Self::new(self.x - 1, self.y)
  }

  fn right(&self) -> Self {
    Self::new(self.x + 1, self.y)
  }

  fn neighbors(&self) -> Vec<Self> {
    vec![self.up(), self.down(), self.left(), self.right()]
  }
}

impl Debug for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn parse_input() -> io::Result<(Point, Vec<Vec<char>>)> {
  let result: Vec<Vec<char>> = read_day(21)?
    .map(|line| line.trim().chars().collect())
    .collect();

  let starting_position = result
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'S')
        .map(move |(x, _)| Point::new(x as i32, y as i32))
    })
    .next()
    .unwrap();

  Ok((starting_position, result))
}

fn bfs_fill(start: Point, grid: &Vec<Vec<char>>, steps: u32) -> usize {
  let mut reachable = HashSet::new();
  let mut seen = HashSet::new();
  let mut queue = VecDeque::new();

  // Assuming the grid is square
  let grid_size = grid.len();

  queue.push_back((start, steps));
  seen.insert(start.clone());

  while let Some((point, remaining_steps)) = queue.pop_front() {
    if remaining_steps % 2 == 0 {
      reachable.insert(point);
    }

    if remaining_steps == 0 {
      continue;
    }

    for neighbor in point.neighbors() {
      
      // Out of bounds on the x axis
      if neighbor.x < 0
        || neighbor.x >= grid_size as i32 
        // Out of bounds on the y axis
        || neighbor.y < 0
        || neighbor.y >= grid_size as i32
        // is a rock
        || grid[neighbor.y as usize][neighbor.x as usize] == '#'
        // already seen
        || seen.contains(&neighbor)
      {
        // Skip
        continue;
      } else {
        seen.insert(neighbor.clone());
        queue.push_back((neighbor, remaining_steps - 1));
      }
    }
  }

  reachable.len()
}

pub fn part_1() -> io::Result<usize> {
  let (start, grid) = parse_input()?;
  let steps_to_take = 64;
  let result = bfs_fill(start, &grid, steps_to_take);
  Ok(result)
}


/*
            🟨
          🟨🟥🟨
        🟨🟥⬜️🟥🟨
      🟨🟥⬜️🟥⬜️🟥🟨
        🟨🟥⬜️🟥🟨
          🟨🟥🟨
            🟨
            
             <----->
            r = steps/size - 1
*/      

pub fn part_2() -> io::Result<usize> {
  let (start, grid) = parse_input()?;
  let steps_to_take = 26501365;

  let size = grid.len();
  // Assert grid is square[]
  assert_eq!(size, grid[0].len());
  // Assert start is at center
  assert_eq!(start, Point::new(grid.len() as i32 / 2, grid.len() as i32 / 2));
  // Assert width is odd
  assert_eq!(grid.len() % 2, 1);
  // Assert we can reach the far end of the farthest parallel universe
  assert_eq!(steps_to_take % size, size / 2);

  let grid_radius = steps_to_take / size - 1;

  // Drawing for reference in drawings/day_21.png

  // Red parts
  // (2, 3) -> 9, (4, 5) -> 16
  // Round down to nearest 2k, add one, square
  let tiles_starting_with_odd_steps = (grid_radius / 2 * 2 + 1).pow(2);
  let points_in_odd_tiles = bfs_fill(start, &grid, (size * 2 + 1) as u32); // Big enough odd number to fill entire tile

  // Green parts
  // (1, 2) -> 4, (3, 4) -> 9
  // Round up to nearest 2k, square
  let tiles_starting_with_even_steps = ((grid_radius + 1) / 2 * 2).pow(2);
  let points_in_even_tiles = bfs_fill(start, &grid, (size * 2) as u32); // Big enough even number to fill entire tile

  // Blue parts (corners)
  let corner_top = bfs_fill(Point::new(start.x, size as i32 - 1), &grid, size as u32 - 1); // Size - 1 because we have enough steps to hit the farthest wall.
  let corner_right = bfs_fill(Point::new(0, start.y), &grid, size as u32 - 1);
  let corner_bottom = bfs_fill(Point::new(start.x, 0), &grid, size as u32 - 1);
  let corner_left = bfs_fill(Point::new(size as i32 - 1, start.y), &grid, size as u32 - 1);

  // Orange parts (tiny triangles)
  let tiny_triangles = grid_radius + 1;
  let tiny_top_right = bfs_fill(Point::new(0, size as i32 - 1), &grid, (size / 2) as u32 - 1);
  let tiny_bottom_right = bfs_fill(Point::new(0, 0), &grid, (size / 2) as u32 - 1);
  let tiny_bottom_left = bfs_fill(Point::new(size as i32 - 1, 0), &grid, (size / 2) as u32 - 1);
  let tiny_top_left = bfs_fill(Point::new(size as i32 - 1, size as i32 - 1), &grid, (size / 2) as u32 - 1);

  // Yellow parts (chipped squares)
  let chipped_squares = grid_radius;
  let chipped_top_right = bfs_fill(Point::new(0, size as i32 - 1), &grid, (3 * size / 2) as u32 - 1);
  let chipped_bottom_right = bfs_fill(Point::new(0, 0), &grid, (3 * size / 2) as u32 - 1);
  let chipped_bottom_left = bfs_fill(Point::new(size as i32 - 1, 0), &grid, (3 * size / 2) as u32 - 1);
  let chipped_top_left = bfs_fill(Point::new(size as i32 - 1, size as i32 - 1), &grid, (3 * size / 2) as u32 - 1);

  let result = tiles_starting_with_odd_steps * points_in_odd_tiles
    + tiles_starting_with_even_steps * points_in_even_tiles
    + corner_top
    + corner_right
    + corner_bottom
    + corner_left
    + tiny_triangles * (tiny_top_right + tiny_bottom_right + tiny_bottom_left + tiny_top_left)
    + chipped_squares * (chipped_top_right + chipped_bottom_right + chipped_bottom_left + chipped_top_left);

  println!("{grid_radius}");
  println!("{tiles_starting_with_odd_steps} {points_in_odd_tiles}");
  println!("{tiles_starting_with_even_steps} {points_in_even_tiles}");
  println!("{corner_top} {corner_right} {corner_bottom} {corner_left}");
  println!("{tiny_top_right} {tiny_top_left} {tiny_bottom_right} {tiny_bottom_left}");
  println!("{chipped_top_right} {chipped_top_left} {chipped_bottom_right} {chipped_bottom_left}");

  Ok(result)
}
