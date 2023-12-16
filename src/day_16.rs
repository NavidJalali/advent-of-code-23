use std::{
  collections::{HashSet, VecDeque},
  io, result,
};

use crate::fs::read_day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn moving_horizontally(&self) -> bool {
    match self {
      Direction::Left | Direction::Right => true,
      _ => false,
    }
  }

  fn moving_vertically(&self) -> bool {
    match self {
      Direction::Up | Direction::Down => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  x: i64,
  y: i64,
}

impl Position {
  pub fn new(x: i64, y: i64) -> Self {
    Self { x, y }
  }

  pub fn move_in_direction(&self, direction: Direction) -> Self {
    match direction {
      Direction::Up => Position::new(self.x, self.y - 1),
      Direction::Down => Position::new(self.x, self.y + 1),
      Direction::Left => Position::new(self.x - 1, self.y),
      Direction::Right => Position::new(self.x + 1, self.y),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
  position: Position,
  direction: Direction,
}

fn solve(matrix: &Vec<Vec<char>>, initial_beam: Beam) -> usize {
  let width = matrix[0].len();
  let height = matrix.len();

  let mut seen = HashSet::<Beam>::new();

  let mut queue = VecDeque::new();

  queue.push_back(initial_beam);

  while let Some(beam) = queue.pop_front() {
    let Beam {
      position,
      direction,
    } = beam;

    let Position { x, y } = position.move_in_direction(direction);

    // Check bounds
    if x < 0 || x >= width as i64 || y < 0 || y >= height as i64 {
      continue;
    }

    let current = matrix[y as usize][x as usize];

    if current == '.'
      || (current == '-' && direction.moving_horizontally())
      || (current == '|' && direction.moving_vertically())
    {
      let beam = Beam {
        position: Position::new(x, y),
        direction,
      };
      // We just pass through this cell
      if !seen.contains(&beam) {
        seen.insert(beam);
        queue.push_back(beam);
      }
    } else if current == '/' {
      let new_direction = match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
      };

      let beam = Beam {
        position: Position::new(x, y),
        direction: new_direction,
      };

      if !seen.contains(&beam) {
        seen.insert(beam);
        queue.push_back(beam);
      }
    } else if current == '\\' {
      let new_direction = match direction {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
      };

      let beam = Beam {
        position: Position::new(x, y),
        direction: new_direction,
      };

      if !seen.contains(&beam) {
        seen.insert(beam);
        queue.push_back(beam);
      }
    } else {
      let splitted_beams = if current == '-' && direction.moving_vertically() {
        vec![
          Beam {
            position: Position::new(x, y),
            direction: Direction::Left,
          },
          Beam {
            position: Position::new(x, y),
            direction: Direction::Right,
          },
        ]
      } else if current == '|' && direction.moving_horizontally() {
        vec![
          Beam {
            position: Position::new(x, y),
            direction: Direction::Up,
          },
          Beam {
            position: Position::new(x, y),
            direction: Direction::Down,
          },
        ]
      } else {
        unreachable!("Invalid input: {}", current)
      };

      for beam in splitted_beams {
        if !seen.contains(&beam) {
          seen.insert(beam);
          queue.push_back(beam);
        }
      }
    }
  }

  let result = seen
    .iter()
    .map(|beam| beam.position)
    .collect::<HashSet<_>>()
    .len();

  result
}

pub fn part_1() -> io::Result<usize> {
  let matrix = read_day(16)?
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let initial_beam = Beam {
    position: Position::new(-1, 0),
    direction: Direction::Right,
  };

  Ok(solve(&matrix, initial_beam))
}

pub fn part_2() -> io::Result<usize> {
  let matrix = read_day(16)?
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let width = matrix[0].len();
  let height = matrix.len();

  let mut initial_beams = vec![];

  // Add left edge. X = -1 Y = 0 .. height Heading Right
  for y in 0..height {
    initial_beams.push(Beam {
      position: Position::new(-1, y as i64),
      direction: Direction::Right,
    });
  }

  // Add right edge. X = width Y = 0 .. height Heading Left
  for y in 0..height {
    initial_beams.push(Beam {
      position: Position::new(width as i64, y as i64),
      direction: Direction::Left,
    });
  }

  // Add top edge. X = 0 .. width Y = -1 Heading Down
  for x in 0..width {
    initial_beams.push(Beam {
      position: Position::new(x as i64, -1),
      direction: Direction::Down,
    });
  }

  // Add bottom edge. X = 0 .. width Y = height Heading Up
  for x in 0..width {
    initial_beams.push(Beam {
      position: Position::new(x as i64, height as i64),
      direction: Direction::Up,
    });
  }

  let result = initial_beams
    .into_iter()
    .map(|beam| solve(&matrix, beam))
    .max()
    .unwrap();

  Ok(result)
}
