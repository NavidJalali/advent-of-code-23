use std::{collections::HashMap, io};

use crate::fs::read_day;

fn read_input() -> io::Result<Vec<Vec<char>>> {
  let result = read_day(14)?
    .map(|line| line.chars().collect())
    .collect::<Vec<Vec<char>>>();
  Ok(result)
}

fn rotate_left(matrix: &mut Vec<Vec<char>>) {
  let mut new_matrix = vec![vec!['.'; matrix.len()]; matrix.len()];
  for i in 0..matrix.len() {
    for j in 0..matrix.len() {
      new_matrix[i][j] = matrix[j][matrix.len() - i - 1];
    }
  }
  *matrix = new_matrix;
}

fn rotate_right(matrix: &mut Vec<Vec<char>>) {
  let mut new_matrix = vec![vec!['.'; matrix.len()]; matrix.len()];
  for i in 0..matrix.len() {
    for j in 0..matrix.len() {
      new_matrix[i][j] = matrix[matrix.len() - j - 1][i];
    }
  }
  *matrix = new_matrix;
}

fn push_left(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  matrix
    .iter()
    .map(|row| {
      row
        .split(|c| *c == '#')
        .map(|group| {
          let mut new_group = group.to_vec();
          new_group.sort_by(|a, b| b.cmp(a));
          new_group.iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("#")
        .chars()
        .collect::<Vec<char>>()
    })
    .collect::<Vec<Vec<char>>>()
}

fn push_north(matrix: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
  rotate_left(matrix);
  let mut matrix = push_left(matrix);
  rotate_right(&mut matrix);
  matrix
}

fn push_west(matrix: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
  push_left(&matrix)
}

fn push_south(matrix: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
  rotate_right(matrix);
  let mut matrix = push_left(matrix);
  rotate_left(&mut matrix);
  matrix
}

fn push_east(matrix: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
  rotate_left(matrix);
  rotate_left(matrix);
  let mut matrix = push_left(matrix);
  rotate_right(&mut matrix);
  rotate_right(&mut matrix);
  matrix
}

fn cycle(matrix: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut matrix = push_north(matrix);
  matrix = push_west(&mut matrix);
  matrix = push_south(&mut matrix);
  matrix = push_east(&mut matrix);

  matrix
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
  for line in matrix {
    for c in line {
      print!("{}", c);
    }
    print!("\n");
  }
}

pub fn part_1() -> io::Result<usize> {
  let mut input = read_input()?;
  rotate_left(&mut input);
  let mut input = push_left(&input);
  rotate_right(&mut input);

  let height = input.len();

  let result = input
    .iter()
    .enumerate()
    .map(|(i, row)| {
      let rock_count = row.iter().filter(|c| **c == 'O').count();
      rock_count * (height - i)
    })
    .sum::<usize>();

  Ok(result)
}

pub fn part_2() -> io::Result<usize> {
  let mut input = read_input()?;

  let total_iterations: usize = 1000000000;

  // Count how many iterations it took to loop

  let mut seen_at_index = HashMap::new();
  seen_at_index.insert(input.clone(), 0);
  let mut matrices = vec![input.clone()];
  let mut iterations: usize = 0;

  let first = loop {
    iterations += 1;
    input = cycle(&mut input);

    match seen_at_index.get(&input) {
      Some(&first_seen) => break first_seen,
      None => {
        seen_at_index.insert(input.clone(), iterations);
        matrices.push(input.clone());
      }
    }
  };

  let index = (total_iterations - first) % (iterations - first) + first;

  let after_iters = matrices[index].clone();

  let height = after_iters.len();

  let result = after_iters
    .iter()
    .enumerate()
    .map(|(i, row)| {
      let rock_count = row.iter().filter(|c| **c == 'O').count();
      rock_count * (height - i)
    })
    .sum::<usize>();

  Ok(result)
}
