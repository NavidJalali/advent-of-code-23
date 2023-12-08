use std::{collections::HashMap, io};

use crate::fs::read_day;

#[derive(Debug, Clone, Copy)]
enum Direction {
  Left,
  Right,
}

#[derive(Debug)]
struct Directions {
  underlying: Vec<Direction>,
}

impl Directions {
  fn at(&self, index: usize) -> Direction {
    self.underlying[index % self.underlying.len()]
  }
}

impl TryFrom<Vec<Direction>> for Directions {
  type Error = ();
  fn try_from(value: Vec<Direction>) -> Result<Self, Self::Error> {
    if value.len() > 0 {
      Ok(Directions { underlying: value })
    } else {
      Err(())
    }
  }
}

#[derive(Debug)]
struct Tree {
  underlying: HashMap<String, (String, String)>,
}

impl Tree {
  fn new() -> Self {
    Tree {
      underlying: HashMap::new(),
    }
  }

  fn insert(&mut self, key: String, value: (String, String)) {
    self.underlying.insert(key, value);
  }

  fn get_left(&self, key: &str) -> Option<&String> {
    self.underlying.get(key).map(|(left, _)| left)
  }

  fn get_right(&self, key: &str) -> Option<&String> {
    self.underlying.get(key).map(|(_, right)| right)
  }
}

#[derive(Debug)]
struct Input {
  tree: Tree,
  directions: Directions,
}

fn parse_input() -> io::Result<Input> {
  let mut lines = read_day(8)?;

  let directions = lines
    .next()
    .map(|line| {
      line
        .trim()
        .chars()
        .map(|c| match c {
          'L' => Direction::Left,
          'R' => Direction::Right,
          _ => panic!("Invalid direction"),
        })
        .collect::<Vec<_>>()
    })
    .map(Directions::try_from)
    .map(Result::unwrap)
    .ok_or(io::Error::new(io::ErrorKind::Other, "No directions found"))?;

  let _ = lines.next();

  let tree = lines.fold(Tree::new(), |mut tree, line| {
    let [node, value] = line
      .trim()
      .split(" = ")
      .map(|s| s.trim())
      .collect::<Vec<_>>()[..2]
    else {
      panic!("Invalid line")
    };

    let [left, right] = value
      .trim_start_matches('(')
      .trim_end_matches(')')
      .split(", ")
      .map(|s| s.trim())
      .collect::<Vec<_>>()[..2]
    else {
      panic!("Invalid value")
    };

    tree.insert(node.to_string(), (left.to_string(), right.to_string()));

    tree
  });

  Ok(Input { tree, directions })
}

pub fn part_1() -> io::Result<usize> {
  let input = parse_input()?;

  let mut i = 0;
  let mut node = "AAA".to_string();

  loop {
    let direction = input.directions.at(i);
    let next_node = match direction {
      Direction::Left => input.tree.get_left(&node),
      Direction::Right => input.tree.get_right(&node),
    };

    match next_node {
      None => {
        return Err(io::Error::new(
          io::ErrorKind::Other,
          format!("Hit a dead end at node {} going {:?}", node, direction),
        ))
      }
      Some(next_node) => {
        if next_node == "ZZZ" {
          break;
        } else {
          node = next_node.to_string();
          i += 1;
        }
      }
    }
  }

  Ok(i + 1)
}

fn lcm(first: usize, second: usize) -> usize {
  first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
  let mut max = first;
  let mut min = second;
  if min > max {
    let val = max;
    max = min;
    min = val;
  }

  loop {
    let res = max % min;
    if res == 0 {
      return min;
    }

    max = min;
    min = res;
  }
}

pub fn part_2() -> io::Result<usize> {
  let input = parse_input()?;

  let start_nodes = input
    .tree
    .underlying
    .keys()
    .filter(|key| key.ends_with("A"))
    .collect::<Vec<_>>();

  fn find_steps_to_terminal_node(input: &Input, node: &String) -> usize {
    let mut i = 0;
    let mut node = node;

    loop {
      let direction = input.directions.at(i);
      let next_node = match direction {
        Direction::Left => input.tree.get_left(node),
        Direction::Right => input.tree.get_right(node),
      };

      match next_node {
        None => panic!("Hit a dead end"),
        Some(next_node) => {
          if next_node.ends_with("Z") {
            break;
          } else {
            node = &next_node;
            i += 1;
          }
        }
      }
    }

    i + 1
  }

  let terminal_steps = start_nodes
    .iter()
    .map(|node| find_steps_to_terminal_node(&input, node))
    .collect::<Vec<_>>();

  let mut x = terminal_steps[0];
  for i in 1..terminal_steps.len() {
    x = lcm(x, terminal_steps[i]);
  }

  Ok(x)
}
