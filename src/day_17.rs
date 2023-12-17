use std::{
  collections::{BinaryHeap, HashSet},
  fmt::Debug,
  io,
  thread::sleep,
  vec,
};

use crate::fs::read_day;

fn parse_input() -> io::Result<Vec<Vec<u32>>> {
  let result = read_day(17)?
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  Ok(result)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
}

impl Debug for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let result = format!("({}, {})", self.x, self.y);
    f.write_str(&result)
  }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Velocity {
  vx: i32,
  vy: i32,
}

impl Velocity {
  fn new(vx: i32, vy: i32) -> Self {
    Self { vx, vy }
  }

  // Exclude forward and backward
  fn possible_turns(&self) -> Vec<Velocity> {
    vec![
      Velocity::new(1, 0),
      Velocity::new(-1, 0),
      Velocity::new(0, 1),
      Velocity::new(0, -1),
    ]
    .into_iter()
    .filter(|v| !((self.vx, self.vy) == (v.vx, v.vy) || (self.vx, self.vy) == (-v.vx, -v.vy)))
    .collect()
  }
}

impl Debug for Velocity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let result = format!("({}, {})", self.vx, self.vy);
    f.write_str(&result)
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
  position: Position,
  velocity: Velocity,
  steps_taken_in_same_direction: u8,
}

impl State {
  fn standing_still(&self) -> bool {
    self.velocity.vx == 0 && self.velocity.vy == 0
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
  state: State,
  heat_loss: u32,
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(other.heat_loss.cmp(&self.heat_loss))
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.heat_loss.cmp(&self.heat_loss)
  }
}

pub fn part_1() -> io::Result<u32> {
  let input = parse_input()?;

  let width = input[0].len();
  let height = input.len();

  let mut seen = HashSet::new();
  let mut priority_queue = BinaryHeap::new();

  let destination = Position::new(width as i32 - 1, height as i32 - 1);

  priority_queue.push(Node {
    state: State {
      position: Position::new(0, 0),
      velocity: Velocity::new(0, 0),
      steps_taken_in_same_direction: 0,
    },
    heat_loss: 0,
  });

  let heat_loss = loop {
    let current = match priority_queue.pop() {
      Some(node) => node,
      None => break None,
    };

    let Node { state, heat_loss } = current;

    if state.position == destination {
      break Some(heat_loss);
    }

    // Check bounds
    if state.position.x < 0
      || state.position.x >= width as i32
      || state.position.y < 0
      || state.position.y >= height as i32
    {
      continue;
    }

    if seen.contains(&state) {
      continue;
    }

    seen.insert(state.clone());

    if state.steps_taken_in_same_direction < 3 && !state.standing_still() {
      let next_position = Position::new(
        state.position.x + state.velocity.vx,
        state.position.y + state.velocity.vy,
      );

      if next_position.x >= 0
        && next_position.x < width as i32
        && next_position.y >= 0
        && next_position.y < height as i32
      {
        let next_state = State {
          position: next_position,
          velocity: state.velocity,
          steps_taken_in_same_direction: state.steps_taken_in_same_direction + 1,
        };

        let heat_loss_at_position = input[next_position.y as usize][next_position.x as usize];

        priority_queue.push(Node {
          state: next_state,
          heat_loss: heat_loss + heat_loss_at_position,
        });
      }
    }

    for new_velocity in state.velocity.possible_turns() {
      let next_position = Position::new(
        state.position.x + new_velocity.vx,
        state.position.y + new_velocity.vy,
      );

      if next_position.x >= 0
        && next_position.x < width as i32
        && next_position.y >= 0
        && next_position.y < height as i32
      {
        let next_state = State {
          position: next_position,
          velocity: new_velocity,
          steps_taken_in_same_direction: 1,
        };

        let heat_loss_at_position = input[next_position.y as usize][next_position.x as usize];

        priority_queue.push(Node {
          state: next_state,
          heat_loss: heat_loss + heat_loss_at_position,
        });
      }
    }
  };

  heat_loss.ok_or(io::Error::new(io::ErrorKind::Other, "No path found"))
}

pub fn part_2() -> io::Result<u32> {
  let input = parse_input()?;

  let width = input[0].len();
  let height = input.len();

  let mut seen = HashSet::new();
  let mut priority_queue = BinaryHeap::new();

  let destination = Position::new(width as i32 - 1, height as i32 - 1);

  priority_queue.push(Node {
    state: State {
      position: Position::new(0, 0),
      velocity: Velocity::new(0, 0),
      steps_taken_in_same_direction: 0,
    },
    heat_loss: 0,
  });

  let heat_loss = loop {
    let current = match priority_queue.pop() {
      Some(node) => node,
      None => break None,
    };

    let Node { state, heat_loss } = current;

    if state.position == destination && state.steps_taken_in_same_direction >= 4 {
      break Some(heat_loss);
    }

    // Check bounds
    if state.position.x < 0
      || state.position.x >= width as i32
      || state.position.y < 0
      || state.position.y >= height as i32
    {
      continue;
    }

    if seen.contains(&state) {
      continue;
    }

    seen.insert(state.clone());

    if state.steps_taken_in_same_direction < 10 && !state.standing_still() {
      let next_position = Position::new(
        state.position.x + state.velocity.vx,
        state.position.y + state.velocity.vy,
      );

      if next_position.x >= 0
        && next_position.x < width as i32
        && next_position.y >= 0
        && next_position.y < height as i32
      {
        let next_state = State {
          position: next_position,
          velocity: state.velocity,
          steps_taken_in_same_direction: state.steps_taken_in_same_direction + 1,
        };

        let heat_loss_at_position = input[next_position.y as usize][next_position.x as usize];

        priority_queue.push(Node {
          state: next_state,
          heat_loss: heat_loss + heat_loss_at_position,
        });
      }
    }

    if state.steps_taken_in_same_direction >= 4 || state.position == Position::new(0, 0) {
      for new_velocity in state.velocity.possible_turns() {
        let next_position = Position::new(
          state.position.x + new_velocity.vx,
          state.position.y + new_velocity.vy,
        );

        if next_position.x >= 0
          && next_position.x < width as i32
          && next_position.y >= 0
          && next_position.y < height as i32
        {
          let next_state = State {
            position: next_position,
            velocity: new_velocity,
            steps_taken_in_same_direction: 1,
          };

          let heat_loss_at_position = input[next_position.y as usize][next_position.x as usize];

          priority_queue.push(Node {
            state: next_state,
            heat_loss: heat_loss + heat_loss_at_position,
          });
        }
      }
    }
  };

  heat_loss.ok_or(io::Error::new(io::ErrorKind::Other, "No path found"))
}
