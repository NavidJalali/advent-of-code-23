use std::{
  collections::{HashMap, VecDeque},
  io, vec,
};

use crate::fs::read_day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
  High,
  Low,
}

#[derive(Debug, Clone, Copy)]
enum FlipFlopState {
  On,
  Off,
}

#[derive(Debug, Clone)]
enum ModuleType {
  FlipFlop(FlipFlopState),
  Conjunction(HashMap<String, Signal>),
}

impl From<char> for ModuleType {
  fn from(c: char) -> Self {
    match c {
      '%' => ModuleType::FlipFlop(FlipFlopState::Off),
      '&' => ModuleType::Conjunction(HashMap::new()),
      _ => panic!("Invalid module type"),
    }
  }
}

#[derive(Debug, Clone)]
struct Module {
  name: String,
  outputs: Vec<String>,
  tpe: ModuleType,
}

fn parse_input() -> io::Result<(Vec<String>, HashMap<String, Module>)> {
  let mut broadcastees = vec![];
  let mut modules: HashMap<String, Module> = HashMap::new();

  read_day(20)?.for_each(|line| {
    let (left, right) = line.split_once(" -> ").unwrap();
    let (left, right) = (left.trim(), right.trim());

    if left == "broadcaster" {
      let targets = right
        .split(", ")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
      broadcastees.extend(targets);
    } else {
      let mut left_chars = left.chars();
      let module_type = ModuleType::from(left_chars.next().unwrap());
      let module_name = left_chars.collect::<String>();
      let targets = right
        .split(", ")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

      let module = Module {
        name: module_name.clone(),
        outputs: targets,
        tpe: module_type,
      };

      modules.insert(module_name, module);
    }
  });

  // If the output of a module is a Conjunction, we need to add it to the Conjunction's state with a state Low.
  let updates = modules
    .iter()
    .flat_map(|(name, module)| {
      module
        .outputs
        .iter()
        .flat_map(|out| modules.get(out))
        .filter_map(|module| match module.tpe {
          ModuleType::Conjunction(_) => Some((name.clone(), module.name.clone())),
          _ => None,
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  for (input, concjunction_name) in updates {
    if let ModuleType::Conjunction(ref mut state) = modules.get_mut(&concjunction_name).unwrap().tpe
    {
      state.insert(input, Signal::Low);
    }
  }
  Ok((broadcastees, modules))
}

pub fn part_1() -> io::Result<i64> {
  let (broadcastees, mut modules) = parse_input()?;

  let (mut low_count, mut high_count) = (0i64, 0i64);

  for _ in 0..1000 {
    low_count += 1;

    let mut queue = VecDeque::new();

    for broadcastee in &broadcastees {
      queue.push_back(("broadcaster".to_string(), broadcastee.clone(), Signal::Low));
    }

    while let Some((sender, receiver, signal)) = queue.pop_front() {
      match signal {
        Signal::High => high_count += 1,
        Signal::Low => low_count += 1,
      }

      match modules.get_mut(&receiver) {
        Some(module) => match module.tpe {
          ModuleType::FlipFlop(ref mut state) => match signal {
            Signal::High => (), // FlipFlop ignores high signals
            Signal::Low => {
              // Flip state
              *state = match state {
                FlipFlopState::On => FlipFlopState::Off,
                FlipFlopState::Off => FlipFlopState::On,
              };

              // Send signal to outputs -> High if on, Low if off
              let signal = match state {
                FlipFlopState::On => Signal::High,
                FlipFlopState::Off => Signal::Low,
              };

              for output in &module.outputs {
                queue.push_back((module.name.clone(), output.clone(), signal.clone()));
              }
            }
          },
          ModuleType::Conjunction(ref mut state) => {
            // Update memory
            state.insert(sender, signal);

            // if all signals are high, send low signal to outputs otherwise send high signal
            let signal = if state.values().all(|x| *x == Signal::High) {
              Signal::Low
            } else {
              Signal::High
            };

            for output in &module.outputs {
              queue.push_back((module.name.clone(), output.clone(), signal.clone()));
            }
          }
        },
        None => continue,
      }
    }
  }
  Ok(low_count * high_count)
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
  let (broadcastees, mut modules) = parse_input()?;

  // Not gonna lie I have 0 ideas on how to solve this without peeking into the input.
  // For me "&gf -> rx" rx is being fed by a conjunction. It is the only input of rx.
  // For gf to send a low signal to rx, all of its inputs must be high.
  // I am going on a limb here and assuming that these inputs are gonna be high on some cycle and once they all line up, rx will receive a low signal.
  // So I will just find their cycle lengths and find their LCM. Hopefully it will produce a correct answer otherwise I am stumped.

  let rx = "rx".to_string();

  // Find gf. Wish this existed in real life.
  let Module {
    name,
    outputs: _,
    tpe: ModuleType::Conjunction(ref state),
  } = modules
    .clone()
    .values()
    .find(|module| module.outputs.contains(&rx))
    .unwrap()
    .clone()
  else {
    panic!("rx not found");
  };

  // The inputs of gf are the ones whose cycle lengths we need to find.
  let mut cycle_lengths = HashMap::<String, i64>::new();

  let mut seen = HashMap::<String, bool>::from_iter(
    state
      .keys()
      .map(|key| (key.clone(), false))
      .collect::<Vec<_>>(),
  );

  let mut button_pressed: i64 = 0;

  // Copy pasta part 1
  loop {
    if seen.values().all(|x| *x) {
      break;
    }

    button_pressed += 1;

    let mut queue = VecDeque::new();

    for broadcastee in &broadcastees {
      queue.push_back(("broadcaster".to_string(), broadcastee.clone(), Signal::Low));
    }

    while let Some((sender, receiver, signal)) = queue.pop_front() {
      match modules.get_mut(&receiver) {
        Some(module) => {
          if module.name == *name && signal == Signal::High {
            if !cycle_lengths.contains_key(&sender) {
              seen.insert(sender.clone(), true);
              cycle_lengths.insert(sender.clone(), button_pressed);
            }
          }

          if seen.values().all(|x| *x) {
            break;
          }

          match module.tpe {
            ModuleType::FlipFlop(ref mut state) => match signal {
              Signal::High => (), // FlipFlop ignores high signals
              Signal::Low => {
                // Flip state
                *state = match state {
                  FlipFlopState::On => FlipFlopState::Off,
                  FlipFlopState::Off => FlipFlopState::On,
                };

                // Send signal to outputs -> High if on, Low if off
                let signal = match state {
                  FlipFlopState::On => Signal::High,
                  FlipFlopState::Off => Signal::Low,
                };

                for output in &module.outputs {
                  queue.push_back((module.name.clone(), output.clone(), signal.clone()));
                }
              }
            },
            ModuleType::Conjunction(ref mut state) => {
              // Update memory
              state.insert(sender, signal);

              // if all signals are high, send low signal to outputs otherwise send high signal
              let signal = if state.values().all(|x| *x == Signal::High) {
                Signal::Low
              } else {
                Signal::High
              };

              for output in &module.outputs {
                queue.push_back((module.name.clone(), output.clone(), signal.clone()));
              }
            }
          }
        }
        None => continue,
      }
    }
  }

  let result = cycle_lengths
    .values()
    .fold(1, |acc, x| lcm(acc as usize, *x as usize));

  Ok(result)
}
