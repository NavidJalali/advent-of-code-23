use std::io;

use crate::fs::read_day;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MemoKey {
  conditions: Vec<u8>,
  damages: Vec<u64>,
}

impl MemoKey {
  fn new(conditions: &[u8], damages: &[u64]) -> Self {
    Self {
      conditions: conditions.iter().copied().collect(),
      damages: damages.iter().copied().collect(),
    }
  }
}

type Cache = std::collections::HashMap<MemoKey, u64>;

fn go(cache: &mut Cache, conditions: &[u8], damages: &[u64]) -> u64 {
  let key = MemoKey::new(conditions, damages);

  if let Some(&result) = cache.get(&key) {
    return result;
  }

  if conditions.is_empty() {
    // No more hot springs left.
    if damages.is_empty() {
      // No more damage counts left. This is a valid configuration and it counts as 1
      return 1;
    } else {
      // There are still damage counts left. This is an invalid configuration and it counts as 0
      return 0;
    }
  }

  if damages.is_empty() {
    // There can be no more damages.
    let damaged_count = conditions.iter().filter(|&&c| c == b'#').count();

    if damaged_count > 0 {
      // There are still hot springs left. This is an invalid configuration and it counts as 0
      return 0;
    } else {
      return 1;
    }
  }

  let mut total = 0;

  let if_operational = |cache: &mut Cache| go(&mut (*cache), &conditions[1..], damages);

  let if_broken = |cache: &mut Cache| {
    // We have to do some checks:
    let damage = damages[0] as usize;
    let springs_left = conditions.len();

    if damage <= springs_left // are there enough springs left to satisfy the broken count?
      && conditions.iter().take(damage as usize).all(|&c| c != b'.') // There are no operational springs for the next N
      && (
        damage as usize == conditions.len() // There are no more springs left
        || conditions[damage] != b'#' // The next spring is not broken
      )
    {
      // Skip N springs
      let mut conds: &[u8] = &conditions[damage..];
      // if there is room for one more skip we do it
      if conds.len() > 0 {
        conds = &conds[1..];
      }

      // Skip this damage count
      let damages: &[u64] = &damages[1..];
      go(&mut (*cache), conds, damages)
    } else {
      0
    }
  };

  // now lets check the first condition
  match conditions[0] {
    // If its a functional hot spring
    b'.' => {
      // We skip it
      total += if_operational(&mut (*cache));
    }
    // If its a damaged hot spring
    b'#' => {
      total += if_broken(&mut (*cache));
    }
    // If we dont know what it is
    b'?' => {
      // We try both
      total += if_operational(&mut (*cache));
      total += if_broken(&mut (*cache));
    }
    _ => unreachable!(),
  }

  cache.insert(key, total);

  total
}

pub fn part_1() -> io::Result<u64> {
  let result = Input::from_file()?
    .into_iter()
    .map(|input| {
      let Input {
        conditions,
        damages,
      } = input;
      let conditions = conditions.as_bytes();
      let damages = damages.as_slice();
      let mut cache = Cache::new();
      go(&mut cache, conditions, damages)
    })
    .sum::<u64>();

  Ok(result)
}

pub fn part_2() -> io::Result<u64> {
  let result = Input::from_file()?
    .into_iter()
    .map(|input| {
      let mut input = input;
      input.unfold();
      let Input {
        conditions,
        damages,
      } = input;
      let conditions = conditions.as_bytes();
      let damages = damages.as_slice();
      let mut cache = Cache::new();
      go(&mut cache, conditions, damages)
    })
    .sum::<u64>();

  Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
  conditions: String,
  damages: Vec<u64>,
}

impl Input {
  fn from_file() -> io::Result<Vec<Self>> {
    let result = read_day(12)?
      .map(|line| {
        let (conds, nums) = line.trim().split_once(" ").expect("Invalid input");
        let nums = nums
          .split(",")
          .map(|s| s.trim().parse().expect("Invalid input"))
          .collect();
        Self {
          conditions: conds.to_string(),
          damages: nums,
        }
      })
      .collect();

    Ok(result)
  }

  fn unfold(&mut self) {
    let conditions = self.conditions.clone();
    let damages = self.damages.clone();
    for _ in 0..4 {
      self.conditions = format!("{}?{}", self.conditions, conditions.clone());
      self.damages.extend(damages.clone());
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn can_unfold() {
    let conditions = "???.###".to_string();
    let damages = vec![1, 1, 3];

    let mut input = Input {
      conditions,
      damages,
    };

    input.unfold();

    let expected = Input {
      conditions: "???.###????.###????.###????.###????.###".to_string(),
      damages: vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
    };

    assert_eq!(input, expected);
  }
}
