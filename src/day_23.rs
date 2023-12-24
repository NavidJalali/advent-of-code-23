use std::{
  collections::{HashMap, HashSet},
  fmt::Debug,
  io,
};

use crate::fs::read_day;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn new(x: i32, y: i32) -> Point {
    Point { x, y }
  }

  fn up(&self) -> Point {
    Point::new(self.x, self.y - 1)
  }

  fn down(&self) -> Point {
    Point::new(self.x, self.y + 1)
  }

  fn left(&self) -> Point {
    Point::new(self.x - 1, self.y)
  }

  fn right(&self) -> Point {
    Point::new(self.x + 1, self.y)
  }

  fn neighbors(&self) -> Vec<Point> {
    vec![self.up(), self.down(), self.left(), self.right()]
  }
}

impl Debug for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn parse_input() -> io::Result<Vec<Vec<char>>> {
  let result = read_day(23)?.map(|line| line.chars().collect()).collect();
  Ok(result)
}

fn find_path(grid: &Vec<Vec<char>>, row: usize) -> Point {
  grid[row]
    .iter()
    .enumerate()
    .find(|(_, c)| **c == '.')
    .map(|(index, _)| Point::new(index as i32, row as i32))
    .expect("No start found")
}

fn in_bounds_and_not_rock(grid: &Vec<Vec<char>>, point: &Point) -> bool {
  let (x, y) = (point.x, point.y);
  x >= 0
    && y >= 0
    && x < grid[0].len() as i32
    && y < grid.len() as i32
    && grid[y as usize][x as usize] != '#'
}

fn dfs_fill(
  grid: &Vec<Vec<char>>,
  vertices: &HashSet<Point>,
  seen: &mut HashSet<Point>,
  graph: &mut HashMap<Point, HashMap<Point, usize>>,
  start: Point,
  current: Point,
  steps_taken: usize,
  get_dirs: impl Fn(&Point) -> Vec<Point> + Clone,
) {
  if steps_taken != 0 && vertices.contains(&current) {
    graph.get_mut(&start).unwrap().insert(current, steps_taken);
  } else {
    let dirs = get_dirs(&current);

    for next_point in dirs {
      if in_bounds_and_not_rock(grid, &next_point) && !seen.contains(&next_point) {
        seen.insert(next_point);
        dfs_fill(
          grid,
          vertices,
          seen,
          graph,
          start,
          next_point,
          steps_taken + 1,
          get_dirs.clone(),
        );
      }
    }
  }
}

fn bruteforce_longest_path(
  graph: &HashMap<Point, HashMap<Point, usize>>,
  start: Point,
  end: Point,
  seen: &mut HashSet<Point>,
) -> Option<usize> {
  if start == end {
    return Some(0);
  } else {
    let mut max = None;
    for (next_point, steps) in graph.get(&start).unwrap().iter() {
      if !seen.contains(next_point) {
        seen.insert(*next_point);
        let result = bruteforce_longest_path(graph, *next_point, end, seen);
        seen.remove(next_point);
        if let Some(result) = result {
          let total_steps = result + steps;
          if max.is_none() || total_steps > max.unwrap() {
            max = Some(total_steps);
          }
        }
      }
    }
    max
  }
}

fn edge_contracted_vertices(grid: &Vec<Vec<char>>) -> HashSet<Point> {
  grid
    .iter()
    .enumerate()
    .flat_map(|(row_index, row)| {
      row
        .iter()
        .enumerate()
        .filter_map(|(column_index, c)| match c {
          '#' => None, // Ignore rocks
          _ => {
            let current = Point::new(column_index as i32, row_index as i32);
            let neighbors = current.neighbors();
            let non_rock_inbound_neighbors = neighbors
              .iter()
              .filter(|neighbor| in_bounds_and_not_rock(&grid, *neighbor))
              .count();

            if non_rock_inbound_neighbors >= 3 {
              Some(current)
            } else {
              None
            }
          }
        })
        .collect::<Vec<_>>()
    })
    .collect::<HashSet<_>>()
}

pub fn part_1() -> io::Result<usize> {
  let grid = parse_input()?;
  let start = find_path(&grid, 0);
  let end = find_path(&grid, grid.len() - 1);

  let mut vertices = edge_contracted_vertices(&grid);
  vertices.insert(start);
  vertices.insert(end);

  let mut graph: HashMap<Point, HashMap<Point, usize>> = HashMap::from_iter(
    vertices
      .iter()
      .map(|&vertex| (vertex, HashMap::<Point, usize>::new())),
  );

  let get_dirs = |point: &Point| {
    let c = grid[point.y as usize][point.x as usize];
    match c {
      '^' => vec![point.up()],
      'v' => vec![point.down()],
      '<' => vec![point.left()],
      '>' => vec![point.right()],
      '.' => point.neighbors(),
      _ => vec![],
    }
  };

  for vertex in vertices.iter() {
    let mut seen = HashSet::new();
    seen.insert(*vertex);
    dfs_fill(
      &grid, &vertices, &mut seen, &mut graph, *vertex, *vertex, 0, get_dirs,
    );
  }

  let mut seen = HashSet::new();
  seen.insert(start);
  let result = bruteforce_longest_path(&graph, start, end, &mut seen).unwrap();

  Ok(result)
}

pub fn part_2() -> io::Result<usize> {
  let grid = parse_input()?;
  let start = find_path(&grid, 0);
  let end = find_path(&grid, grid.len() - 1);

  let mut vertices = edge_contracted_vertices(&grid);
  vertices.insert(start);
  vertices.insert(end);

  let mut graph: HashMap<Point, HashMap<Point, usize>> = HashMap::from_iter(
    vertices
      .iter()
      .map(|&vertex| (vertex, HashMap::<Point, usize>::new())),
  );

  let get_dirs = |point: &Point| {
    let c = grid[point.y as usize][point.x as usize];
    match c {
      '^' => point.neighbors(),
      'v' => point.neighbors(),
      '<' => point.neighbors(),
      '>' => point.neighbors(),
      '.' => point.neighbors(),
      _ => vec![],
    }
  };

  for vertex in vertices.iter() {
    let mut seen = HashSet::new();
    seen.insert(*vertex);
    dfs_fill(
      &grid, &vertices, &mut seen, &mut graph, *vertex, *vertex, 0, get_dirs,
    );
  }

  let mut seen = HashSet::new();
  seen.insert(start);
  let result = bruteforce_longest_path(&graph, start, end, &mut seen).unwrap();

  Ok(result)
}
