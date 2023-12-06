use std::io;

use crate::fs::read_day;

#[derive(Debug)]
struct Race {
  time: u64,
  distance: u64,
}

fn parse_input_1() -> io::Result<Vec<Race>> {
  let lines = read_day(6)?
    .map(|line| {
      line
        .trim()
        .split(' ')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .collect::<Vec<u64>>()
    })
    .collect::<Vec<_>>();

  let races = lines[0]
    .iter()
    .zip(lines[1].iter())
    .map(|(time, distance)| Race {
      time: *time,
      distance: *distance,
    })
    .collect::<Vec<_>>();

  Ok(races)
}

fn parse_input_2() -> io::Result<Race> {
  let lines = read_day(6)?
    .flat_map(|line| {
      let result: Option<u64> = line
        .trim()
        .split(" ")
        .filter(|s| !(*s).is_empty())
        .skip(1)
        .fold("".to_string(), |x, y| (x.to_string() + y))
        .parse()
        .ok();
      result
    })
    .collect::<Vec<_>>();

  lines
    .get(0)
    .zip(lines.get(1))
    .map(|(&time, &distance)| Race { time, distance })
    .ok_or(io::Error::new(io::ErrorKind::Other, "Input borken"))
}

// distance travelled for max time t when holding the button for v seconds:
// x(t, v) = -v² + tv
// Ideal time to hold is then t/2
// If the current record travelled distance d then it held the button for:
// v² - tv + d = 0 => (t ± √(t² - 4d))/2
// We can find all the integer numbers between the ideal time and the current record.
// Since this forms a parabola its symmetric around the ideal time
// so we can reflect half the solution to get the full one.

// Count integes between x and y where smaller side is exclusive and bigger side is inclusives
fn ints_between(x: f64, y: f64) -> u64 {
  match f64::total_cmp(&x, &y) {
    std::cmp::Ordering::Less => {
      let lower: u64 = f64::floor(x + 1f64) as u64;
      let higher: u64 = y as u64;
      higher - lower + 1
    }
    std::cmp::Ordering::Equal => 0u64,
    std::cmp::Ordering::Greater => ints_between(y, x),
  }
}

pub fn part_1() -> io::Result<u64> {
  let result: u64 = parse_input_1()?
    .iter()
    .map(|&Race { time, distance }| {
      let t: f64 = time as f64;
      let d: f64 = distance as f64;
      let ideal_time = t / 2f64;
      let opponent_time: f64 = (t - f64::sqrt(t * t - 4f64 * d)) / 2f64;
      let between = ints_between(opponent_time, ideal_time);
      let mut result = 2 * between;
      if time % 2 == 0 {
        result = result.saturating_sub(1);
      }
      result
    })
    .product();

  Ok(result)
}

pub fn part_2() -> io::Result<u64> {
  let result = match parse_input_2()? {
    Race { time, distance } => {
      let t: f64 = time as f64;
      let d: f64 = distance as f64;
      let ideal_time = t / 2f64;
      let opponent_time: f64 = (t - f64::sqrt(t * t - 4f64 * d)) / 2f64;
      let between = ints_between(opponent_time, ideal_time);
      let mut result = 2 * between;
      if time % 2 == 0 {
        result = result.saturating_sub(1);
      }
      result
    }
  };

  Ok(result)
}
