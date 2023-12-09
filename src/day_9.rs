use std::io;

use crate::fs::read_day;

fn pairwise_diff(numbers: Vec<i64>) -> Vec<i64> {
  numbers
    .iter()
    .zip(numbers.iter().skip(1))
    .map(|(a, b)| b - a)
    .collect()
}

fn solve<F>(predict: F) -> io::Result<i64>
where
  F: Fn(Vec<i64>) -> i64,
{
  let result = read_day(9)?
    .map(|line| {
      line
        .split(" ")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>()
    })
    .map(predict)
    .sum();
  Ok(result)
}

pub fn part_1() -> io::Result<i64> {
  fn predict_next(numbers: Vec<i64>) -> i64 {
    match numbers.last() {
      Some(&last) => last + predict_next(pairwise_diff(numbers)),
      None => 0,
    }
  }

  solve(predict_next)
}

pub fn part_2() -> io::Result<i64> {
  fn predict_next(numbers: Vec<i64>) -> i64 {
    match numbers.first() {
      Some(&head) => head - predict_next(pairwise_diff(numbers)),
      None => 0,
    }
  }

  solve(predict_next)
}
