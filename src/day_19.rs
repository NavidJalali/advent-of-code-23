use std::{collections::HashMap, fmt::Debug, io};

use crate::fs::read_day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
  X, // extremely cool looking
  M, // Musical
  A, // Aerodynamic
  S, // Shiny
}

impl Category {
  fn all() -> Vec<Category> {
    vec![Category::X, Category::M, Category::A, Category::S]
  }
}

impl From<char> for Category {
  fn from(c: char) -> Self {
    match c {
      'x' => Category::X,
      'm' => Category::M,
      'a' => Category::A,
      's' => Category::S,
      _ => panic!("Invalid category"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Decision {
  Accept,
  Reject,
}

impl TryFrom<char> for Decision {
  type Error = ();

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      'A' => Ok(Decision::Accept),
      'R' => Ok(Decision::Reject),
      _ => Err(()),
    }
  }
}

#[derive(Debug, Clone)]
enum Outcome {
  NextWorkflow { name: String },
  Terminal(Decision),
}

impl From<String> for Outcome {
  fn from(s: String) -> Self {
    if s.len() == 1 {
      let maybe_decision = Decision::try_from(s.chars().next().unwrap());
      match maybe_decision {
        Ok(d) => Outcome::Terminal(d),
        Err(_) => Outcome::NextWorkflow { name: s },
      }
    } else {
      Outcome::NextWorkflow { name: s }
    }
  }
}

enum Rule {
  GreaterThan(Category, u64, Outcome),
  LessThan(Category, u64, Outcome),
}

impl Rule {
  fn get_category(&self) -> Category {
    match self {
      Rule::GreaterThan(c, _, _) => *c,
      Rule::LessThan(c, _, _) => *c,
    }
  }

  fn get_outcome(&self) -> &Outcome {
    match self {
      Rule::GreaterThan(_, _, o) => o,
      Rule::LessThan(_, _, o) => o,
    }
  }
}

impl Debug for Rule {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Rule::GreaterThan(c, v, o) => write!(f, "{:?} > {} -> {:?}", c, v, o),
      Rule::LessThan(c, v, o) => write!(f, "{:?} < {} -> {:?}", c, v, o),
    }
  }
}

impl From<String> for Rule {
  // m>1548:next_workflow
  fn from(s: String) -> Self {
    let mut chars = s.chars();
    let category = chars.next().unwrap().into();
    let op = chars.next().unwrap();
    let (value, outcome) = chars.as_str().split_once(':').unwrap();
    let value = value.parse::<u64>().unwrap();
    let outcome = Outcome::from(outcome.to_string());
    match op {
      '>' => Rule::GreaterThan(category, value, outcome),
      '<' => Rule::LessThan(category, value, outcome),
      _ => panic!("Invalid operator"),
    }
  }
}

#[derive(Debug)]
struct Part {
  x: u64,
  m: u64,
  a: u64,
  s: u64,
}

impl Part {
  fn get_category(&self, category: Category) -> u64 {
    match category {
      Category::X => self.x,
      Category::M => self.m,
      Category::A => self.a,
      Category::S => self.s,
    }
  }
}

impl From<String> for Part {
  // {x=787,m=2655,a=1222,s=2876}
  fn from(s: String) -> Self {
    let trimmed = s.trim_start_matches('{').trim_end_matches('}');
    let mut parts = trimmed.split(',').collect::<Vec<&str>>();
    let x = parts
      .remove(0)
      .trim_start_matches("x=")
      .parse::<u64>()
      .unwrap();
    let m = parts
      .remove(0)
      .trim_start_matches("m=")
      .parse::<u64>()
      .unwrap();
    let a = parts
      .remove(0)
      .trim_start_matches("a=")
      .parse::<u64>()
      .unwrap();
    let s = parts
      .remove(0)
      .trim_start_matches("s=")
      .parse::<u64>()
      .unwrap();
    Part { x, m, a, s }
  }
}

#[derive(Debug)]
struct Workflow {
  name: String,
  rules: Vec<Rule>,
  fallback: Outcome,
}

impl Workflow {
  fn outcome_of(&self, part: &Part) -> Outcome {
    for rule in &self.rules {
      match rule {
        Rule::GreaterThan(category, value, outcome) => {
          if part.get_category(*category) > *value {
            return outcome.clone();
          }
        }
        Rule::LessThan(category, value, outcome) => {
          if part.get_category(*category) < *value {
            return outcome.clone();
          }
        }
      }
    }
    self.fallback.clone()
  }
}

impl From<String> for Workflow {
  // tb{s>428:tf,s<233:qcp,a<1563:shd,rj}
  fn from(s: String) -> Self {
    let stripped = s.trim_end_matches('}');
    let (name, parts) = stripped.split_once('{').unwrap();
    let name = name.to_string();
    let mut parts = parts
      .split(',')
      .map(|s| s.to_string())
      .collect::<Vec<String>>();

    let fallback = parts.pop().unwrap();
    let fallback = Outcome::from(fallback);

    let rules = parts
      .iter()
      .map(|s| Rule::from(s.to_string()))
      .collect::<Vec<Rule>>();

    Self {
      name,
      rules,
      fallback,
    }
  }
}

fn parse_input() -> io::Result<(Vec<Workflow>, Vec<Part>)> {
  let input = read_day(19)?.collect::<Vec<String>>();

  let [workflows, parts] = input.split(|s| s.is_empty()).collect::<Vec<_>>()[..2] else {
    panic!("Invalid input");
  };

  let workflows = workflows
    .to_vec()
    .into_iter()
    .map(|s| Workflow::from(s))
    .collect::<Vec<Workflow>>();

  let parts = parts
    .to_vec()
    .into_iter()
    .map(|s| Part::from(s))
    .collect::<Vec<Part>>();

  Ok((workflows, parts))
}

fn decision_for_part(workflows: &HashMap<String, Workflow>, part: &Part) -> Decision {
  let mut workflow = workflows.get("in").unwrap();
  loop {
    let outcome = workflow.outcome_of(part);
    match outcome {
      Outcome::Terminal(decision) => return decision,
      Outcome::NextWorkflow { name } => {
        workflow = workflows.get(&name).unwrap();
      }
    }
  }
}

pub fn part_1() -> io::Result<u64> {
  let (workflows, parts) = parse_input()?;
  let workflows = workflows
    .into_iter()
    .map(|w| (w.name.clone(), w))
    .collect::<HashMap<String, Workflow>>();

  let sum = parts
    .iter()
    .map(|p| {
      let decision = decision_for_part(&workflows, p);
      match decision {
        Decision::Accept => p.x + p.m + p.a + p.s,
        Decision::Reject => 0,
      }
    })
    .sum::<u64>();

  Ok(sum)
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Range {
  min_inclusive: u64,
  max_inclusive: u64,
}

impl Range {
  fn new(min_inclusive: u64, max_inclusive: u64) -> Self {
    Self {
      min_inclusive,
      max_inclusive,
    }
  }

  fn non_empty(&self) -> bool {
    self.min_inclusive <= self.max_inclusive
  }
}

impl Debug for Range {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}, {}]", self.min_inclusive, self.max_inclusive)
  }
}

fn count_accepted_ranges(
  ranges: &HashMap<Category, Range>,
  outcome: &Outcome,
  workflows: &HashMap<String, Workflow>,
) -> u64 {
  let mut ranges = ranges.to_owned();

  match outcome {
    Outcome::Terminal(Decision::Reject) => return 0,
    Outcome::Terminal(Decision::Accept) => {
      let mut product = 1;
      for range in ranges.values() {
        product *= range.max_inclusive - range.min_inclusive + 1;
      }
      return product;
    }
    Outcome::NextWorkflow { name } => {
      let workflow = workflows.get(name).unwrap();

      let mut total = 0;

      let mut covered_all_cases = false;

      for rule in workflow.rules.iter() {
        let category = rule.get_category();
        let &Range {
          min_inclusive,
          max_inclusive,
        } = ranges.get(&category).unwrap();

        let (matching, not_matching) = match rule {
          Rule::GreaterThan(_, value, _) => {
            let matching_half = Range::new(*value + 1, max_inclusive);
            let non_matching_half = Range::new(min_inclusive, *value);
            (matching_half, non_matching_half)
          }
          Rule::LessThan(_, value, _) => {
            let matching_half = Range::new(min_inclusive, *value - 1);
            let non_matching_half = Range::new(*value, max_inclusive);
            (matching_half, non_matching_half)
          }
        };

        if matching.non_empty() {
          let mut new_ranges = ranges.clone();
          new_ranges.insert(category, matching);
          let count = count_accepted_ranges(&new_ranges, rule.get_outcome(), workflows);
          total += count;
        }

        if not_matching.non_empty() {
          ranges = ranges.clone();
          ranges.insert(category, not_matching);
        } else {
          covered_all_cases = true;
          break;
        }
      }

      if covered_all_cases {
        return total;
      } else {
        return total + count_accepted_ranges(&ranges, &workflow.fallback, workflows);
      }
    }
  }
}

pub fn part_2() -> io::Result<u64> {
  let (workflows, _) = parse_input()?;

  let mut ranges = HashMap::<Category, Range>::new();

  for category in Category::all() {
    ranges.insert(
      category,
      Range {
        min_inclusive: 1,
        max_inclusive: 4000,
      },
    );
  }

  let workflows = workflows
    .into_iter()
    .map(|w| (w.name.clone(), w))
    .collect::<HashMap<String, Workflow>>();

  let count = count_accepted_ranges(
    &ranges,
    &Outcome::NextWorkflow {
      name: "in".to_string(),
    },
    &workflows,
  );

  Ok(count)
}
