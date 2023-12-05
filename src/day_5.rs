use std::{fmt::Debug, io, vec};

use crate::fs::read_day;

#[derive(Clone, Copy, PartialEq)]
pub struct Interval {
  left: u64,
  right: u64,
}

impl Debug for Interval {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}, {}]", self.left, self.right)
  }
}

impl Interval {
  fn intersection(&self, other: &Interval) -> Option<Interval> {
    if self.left > other.right || self.right < other.left {
      None
    } else {
      Some(Interval {
        left: self.left.max(other.left),
        right: self.right.min(other.right),
      })
    }
  }
}

// Would include all numbers between left_interval and right_interval inclusive
#[derive(Debug)]
pub struct IntervalSearchTreeNode {
  // Data for interval search
  data: Interval,
  interval: Interval,
  max_to_the_right: u64,
  // BST data
  left: Box<IntervalSearchTree>,
  right: Box<IntervalSearchTree>,
}

pub struct IntervalSearchTree {
  root: Option<IntervalSearchTreeNode>,
}

impl Debug for IntervalSearchTree {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.string_repr())
  }
}

impl IntervalSearchTree {
  pub fn new() -> Self {
    Self { root: None }
  }

  pub fn string_repr(&self) -> String {
    match self.root {
      None => String::from("Leaf"),
      Some(ref node) => {
        let left = node.left.string_repr();
        let right = node.right.string_repr();

        format!(
          "Node(L:{}, R:{}, M:{}, Left:{}, Right:{})",
          node.interval.left, node.interval.right, node.max_to_the_right, left, right
        )
      }
    }
  }

  pub fn insert(&mut self, left: u64, right: u64, data: Interval) {
    match self.root {
      None => {
        self.root = Some(IntervalSearchTreeNode {
          data,
          interval: Interval { left, right },
          max_to_the_right: right,
          left: Box::new(IntervalSearchTree::new()),
          right: Box::new(IntervalSearchTree::new()),
        });
      }

      Some(ref mut node) => {
        // Binary search tree insertion with "left" as the key, but we have to update the max_to_the_right at each node
        if left < node.interval.left {
          node.max_to_the_right = node.max_to_the_right.max(right);
          node.left.insert(left, right, data);
        } else {
          node.max_to_the_right = node.max_to_the_right.max(right);
          node.right.insert(left, right, data);
        }
      }
    }
  }

  pub fn search(&self, point: u64) -> Option<&IntervalSearchTreeNode> {
    match self.root {
      None => None,
      Some(ref node) => {
        if point < node.interval.left {
          // Would be on the left side of the tree
          node.left.search(point)
        } else {
          // On the root or the right side of the tree
          if point <= node.interval.right {
            // On the root
            Some(node)
          } else {
            // On the right side of the tree but we can check if it exceeds the max_to_the_right to immediately return None
            if point <= node.max_to_the_right {
              node.right.search(point)
            } else {
              None
            }
          }
        }
      }
    }
  }

  fn insert_from_triplet(&mut self, out_start: u64, in_start: u64, range_length: u64) {
    let left = in_start;
    let right = in_start + range_length - 1;
    let data = Interval {
      left: out_start,
      right: out_start + range_length - 1,
    };
    self.insert(left, right, data);
  }

  pub fn find_output(&self, point: u64) -> u64 {
    self.search(point).map_or_else(
      || point,
      |node| {
        let offset = point - node.interval.left;
        node.data.left + offset
      },
    )
  }

  pub fn nodes(&self) -> Vec<&IntervalSearchTreeNode> {
    match self.root {
      None => vec![],
      Some(ref node) => {
        let mut result = vec![node];
        result.extend(node.left.nodes());
        result.extend(node.right.nodes());
        result
      }
    }
  }

  // This absolutely sucks balls but its fast enough for the input
  pub fn remap(&self, input: &Interval) -> Vec<Interval> {
    let mut nodes = self.nodes();
    nodes.sort_by(|a, b| a.interval.left.cmp(&b.interval.left));

    let mut result = vec![];

    let mut previous_end = input.left;

    for node in nodes {
      // If we have no intersection at all we can skip this node. Otherwise:
      if let Some(intersection) = node.interval.intersection(input) {
        // The space between this interval and the last interval needs to be filled by
        // creating an interval that maps the same space in the output

        let map_direct_left = previous_end;
        let map_direct_right = intersection.left;

        if map_direct_right > map_direct_left {
          result.push(Interval {
            left: map_direct_left,
            right: map_direct_right - 1,
          });
        }

        previous_end = intersection.right + 1;

        // next add the projection of the interval onto the output
        result.push(Interval {
          left: intersection.left + node.data.left - node.interval.left,
          right: intersection.right + node.data.left - node.interval.left,
        });
      }
    }

    // If there is space left at the end we need to fill it
    if previous_end <= input.right {
      result.push(Interval {
        left: previous_end,
        right: input.right,
      });
    }

    result
  }
}

#[derive(Debug)]
struct Input {
  seeds: Vec<u64>,
  seed_to_soil: IntervalSearchTree,
  soil_to_fertilizer: IntervalSearchTree,
  fertilizer_to_water: IntervalSearchTree,
  water_to_light: IntervalSearchTree,
  light_to_temperature: IntervalSearchTree,
  temperature_to_humidity: IntervalSearchTree,
  humidity_to_location: IntervalSearchTree,
}

impl Input {
  fn submit(&mut self, state: &ParseState, triplet: (u64, u64, u64)) {
    match state {
      ParseState::Seeds => panic!("Cannot submit triplets to seeds!"),
      ParseState::SeedToSoil => self
        .seed_to_soil
        .insert_from_triplet(triplet.0, triplet.1, triplet.2),
      ParseState::SoilToFertilizer => self
        .soil_to_fertilizer
        .insert_from_triplet(triplet.0, triplet.1, triplet.2),
      ParseState::FertilizerToWater => self
        .fertilizer_to_water
        .insert_from_triplet(triplet.0, triplet.1, triplet.2),
      ParseState::WaterToLight => self
        .water_to_light
        .insert_from_triplet(triplet.0, triplet.1, triplet.2),
      ParseState::LightToTemperature => self
        .light_to_temperature
        .insert_from_triplet(triplet.0, triplet.1, triplet.2),
      ParseState::TemperatureToHumidity => self
        .temperature_to_humidity
        .insert_from_triplet(triplet.0, triplet.1, triplet.2),
      ParseState::HumidityToLocation => self
        .humidity_to_location
        .insert_from_triplet(triplet.0, triplet.1, triplet.2),
    }
  }
}

impl Default for Input {
  fn default() -> Self {
    Self {
      seeds: vec![],
      seed_to_soil: IntervalSearchTree::new(),
      soil_to_fertilizer: IntervalSearchTree::new(),
      fertilizer_to_water: IntervalSearchTree::new(),
      water_to_light: IntervalSearchTree::new(),
      light_to_temperature: IntervalSearchTree::new(),
      temperature_to_humidity: IntervalSearchTree::new(),
      humidity_to_location: IntervalSearchTree::new(),
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum ParseState {
  Seeds,
  SeedToSoil,
  SoilToFertilizer,
  FertilizerToWater,
  WaterToLight,
  LightToTemperature,
  TemperatureToHumidity,
  HumidityToLocation,
}

impl ParseState {
  fn next(&self) -> Option<Self> {
    match self {
      ParseState::Seeds => Some(ParseState::SeedToSoil),
      ParseState::SeedToSoil => Some(ParseState::SoilToFertilizer),
      ParseState::SoilToFertilizer => Some(ParseState::FertilizerToWater),
      ParseState::FertilizerToWater => Some(ParseState::WaterToLight),
      ParseState::WaterToLight => Some(ParseState::LightToTemperature),
      ParseState::LightToTemperature => Some(ParseState::TemperatureToHumidity),
      ParseState::TemperatureToHumidity => Some(ParseState::HumidityToLocation),
      ParseState::HumidityToLocation => None,
    }
  }
}

fn parse_input<F>(seed_parser: F) -> io::Result<Input>
where
  F: Fn(String) -> Vec<u64>,
{
  let mut input = Input::default();
  let mut state = Some(ParseState::Seeds);

  let mut result = read_day(5)?.filter_map(|s| {
    let trimmed = s.trim();
    if trimmed.is_empty() {
      None
    } else {
      Some(trimmed.to_string())
    }
  });

  let seeds_raw = result.next().expect("Expected seeds");

  let seeds = seed_parser(seeds_raw);

  input.seeds = seeds;

  for line in result {
    if !line.is_empty() {
      // Check if we are at a boundary. That is when the first charachter is a letter
      let first_char = line.chars().next().expect("Expected a first char");
      if first_char.is_alphabetic() {
        state = state.and_then(|s| s.next());
      } else {
        let triplet = line
          .split(" ")
          .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
              None
            } else {
              trimmed.parse().ok()
            }
          })
          .collect::<Vec<u64>>();

        if triplet.len() != 3 {
          panic!("Expected a triplet but got {:?}", triplet);
        }

        let s = &state.expect("Expected a state");

        input.submit(s, (triplet[0], triplet[1], triplet[2]));
      }
    }
  }

  Ok(input)
}

fn find_min_location<F>(seed_parser: F) -> io::Result<u64>
where
  F: Fn(String) -> Vec<u64>,
{
  let input = parse_input(seed_parser).unwrap();

  let result = input
    .seeds
    .iter()
    .map(|seed| {
      let soil = input.seed_to_soil.find_output(*seed);
      let fertilizer = input.soil_to_fertilizer.find_output(soil);
      let water = input.fertilizer_to_water.find_output(fertilizer);
      let light = input.water_to_light.find_output(water);
      let temperature = input.light_to_temperature.find_output(light);
      let humidity = input.temperature_to_humidity.find_output(temperature);
      let location = input.humidity_to_location.find_output(humidity);
      location
    })
    .min();

  result.ok_or(io::Error::new(io::ErrorKind::Other, "No result"))
}

// 486613012
pub fn part_1() -> io::Result<u64> {
  find_min_location(|line: String| {
    line
      .split(" ")
      .filter_map(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
          None
        } else {
          trimmed.parse().ok()
        }
      })
      .collect::<Vec<u64>>()
  })
}

// 56931769
pub fn part_2_bruteforce() -> io::Result<u64> {
  find_min_location(|line| {
    let values = line
      .split(" ")
      .filter_map(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
          None
        } else {
          Some(trimmed.parse().ok()?)
        }
      })
      .collect::<Vec<u64>>();

    let mut seeds = vec![];

    let mut index = 0;

    while index + 1 < values.len() {
      let first = values[index];
      let second = values[index + 1];

      for i in 0..second {
        seeds.push(first + i);
      }

      index += 2;
    }

    seeds
  })
}

pub fn part_2() -> io::Result<u64> {
  let seed_pairs = |line: String| {
    line
      .split(" ")
      .filter_map(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
          None
        } else {
          trimmed.parse().ok()
        }
      })
      .collect::<Vec<u64>>()
  };

  let input = parse_input(seed_pairs).unwrap();

  let mut seed_intervals = vec![];

  let mut index = 0;
  while index + 1 < input.seeds.len() {
    let first = input.seeds[index];
    let second = input.seeds[index + 1];

    let interval = Interval {
      left: first,
      right: first + second - 1,
    };

    seed_intervals.push(interval);

    index += 2;
  }

  let result = seed_intervals
    .iter()
    .flat_map(|interval| input.seed_to_soil.remap(interval))
    .flat_map(|interval| input.soil_to_fertilizer.remap(&interval))
    .flat_map(|interval| input.fertilizer_to_water.remap(&interval))
    .flat_map(|interval| input.water_to_light.remap(&interval))
    .flat_map(|interval| input.light_to_temperature.remap(&interval))
    .flat_map(|interval| input.temperature_to_humidity.remap(&interval))
    .flat_map(|interval| input.humidity_to_location.remap(&interval))
    .map(|interval| interval.left)
    .min();

  result.ok_or(io::Error::new(io::ErrorKind::Other, "No result"))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn insert_works() {
    let mut ist = IntervalSearchTree::new();

    let i = Interval { left: 1, right: 2 };

    ist.insert(17, 19, i);
    ist.insert(5, 8, i);
    ist.insert(21, 24, i);
    ist.insert(4, 8, i);
    ist.insert(15, 18, i);
    ist.insert(7, 10, i);
    ist.insert(16, 22, i);

    assert_eq!(
      ist.string_repr(),
      "Node(L:17, R:19, M:24, Left:Node(L:5, R:8, M:22, Left:Node(L:4, R:8, M:8, Left:Leaf, Right:Leaf), Right:Node(L:15, R:18, M:22, Left:Node(L:7, R:10, M:10, Left:Leaf, Right:Leaf), Right:Node(L:16, R:22, M:22, Left:Leaf, Right:Leaf))), Right:Node(L:21, R:24, M:24, Left:Leaf, Right:Leaf))"
    );
  }

  #[test]
  fn search() {
    let mut ist = IntervalSearchTree::new();

    ist.insert(17, 19, Interval { left: 1, right: 1 });
    ist.insert(5, 8, Interval { left: 2, right: 2 });
    ist.insert(21, 24, Interval { left: 3, right: 3 });
    ist.insert(4, 8, Interval { left: 4, right: 4 });
    ist.insert(15, 18, Interval { left: 5, right: 5 });
    ist.insert(7, 10, Interval { left: 6, right: 6 });
    ist.insert(16, 22, Interval { left: 7, right: 7 });

    assert_eq!(
      ist.search(22).map(|node| node.data),
      Some(Interval { left: 3, right: 3 })
    );
    assert_eq!(
      ist.search(9).map(|node| node.data),
      Some(Interval { left: 6, right: 6 })
    );
    assert_eq!(
      ist.search(4).map(|node| node.data),
      Some(Interval { left: 4, right: 4 })
    );
    assert_eq!(ist.search(20).map(|node| node.data), None);
    assert_eq!(ist.search(1).map(|node| node.data), None);
    assert_eq!(ist.search(25).map(|node| node.data), None);
  }

  #[test]
  fn test_remap() {
    let mut ist = IntervalSearchTree::new();
    ist.insert(0, 1, Interval { left: 1, right: 2 });

    let result = ist.remap(&Interval { left: 0, right: 3 });

    assert_eq!(
      result,
      vec![
        Interval { left: 1, right: 2 },
        Interval { left: 2, right: 3 }
      ]
    );

    let mut ist = IntervalSearchTree::new();

    ist.insert(
      0,
      2,
      Interval {
        left: 10,
        right: 12,
      },
    );
    ist.insert(
      4,
      6,
      Interval {
        left: 14,
        right: 16,
      },
    );
    ist.insert(
      8,
      10,
      Interval {
        left: 18,
        right: 20,
      },
    );

    let result = ist.remap(&Interval { left: 1, right: 9 });

    assert_eq!(
      result,
      vec![
        Interval {
          left: 11,
          right: 12
        },
        Interval { left: 3, right: 3 },
        Interval {
          left: 14,
          right: 16
        },
        Interval { left: 7, right: 7 },
        Interval {
          left: 18,
          right: 19
        },
      ]
    );
  }
}
