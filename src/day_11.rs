use std::{collections::HashSet, fmt::Debug, io, vec};

use crate::fs::read_day;

fn parse_input() -> io::Result<Vec<Vec<char>>> {
  let result = read_day(11)?
    .map(|line| line.trim().chars().collect())
    .collect();

  Ok(result)
}

#[derive(Clone, Copy)]
struct Tile {
  value: char,
  xs_to_left: usize,
  xs_to_top: usize,
}

impl Debug for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}, {}, {}]",
      self.value, self.xs_to_left, self.xs_to_top
    )
  }
}

fn handle_expansion(input: &mut Vec<Vec<char>>) -> Vec<Vec<Tile>> {
  let width = input[0].len();
  let height = input.len();

  let all_vaccum_rows = input
    .iter()
    .enumerate()
    .filter_map(|(index, row)| {
      if row.iter().all(|c| *c == '.') {
        Some(index)
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  let all_vacuum_columns = {
    (0..width).enumerate().filter_map(|(index, col_index)| {
      if (0..height).all(|row_index| input[row_index][col_index] == '.') {
        Some(index)
      } else {
        None
      }
    })
  }
  .collect::<Vec<_>>();

  for idx in &all_vaccum_rows {
    let expanded = vec!['X'; width];
    input[*idx] = expanded;
  }

  for idx in &all_vacuum_columns {
    for row in input.iter_mut() {
      row[*idx] = 'X';
    }
  }

  let all_vaccum_rows = all_vaccum_rows.into_iter().collect::<HashSet<_>>();
  let all_vacuum_columns = all_vacuum_columns.into_iter().collect::<HashSet<_>>();

  let mut result: Vec<Vec<Tile>> = Vec::new();

  let mut xs_to_left = 0;
  let mut xs_to_top = 0;

  for row_index in 0..height {
    if all_vaccum_rows.contains(&row_index) {
      xs_to_top += 1;

      let value = Tile {
        value: 'X',
        xs_to_left,
        xs_to_top,
      };

      result.push(vec![value; width]);
    } else {
      let mut row = Vec::new();
      for col_index in 0..width {
        if all_vacuum_columns.contains(&col_index) {
          xs_to_left += 1;
          row.push(Tile {
            value: 'X',
            xs_to_left,
            xs_to_top,
          });
        } else {
          row.push(Tile {
            value: input[row_index][col_index],
            xs_to_left,
            xs_to_top,
          });
        }
      }
      result.push(row);
    }
    xs_to_left = 0;
  }

  result
}

fn find_all_galaxies(input: &Vec<Vec<Tile>>) -> Vec<(Tile, usize, usize)> {
  let width = input[0].len();
  let height = input.len();

  let mut result = Vec::new();

  for row_index in 0..height {
    for col_index in 0..width {
      let tile = input[row_index][col_index];
      if tile.value == '#' {
        result.push((tile, col_index, row_index));
      }
    }
  }

  result
}

fn distance_between(
  u: &(Tile, usize, usize),
  v: &(Tile, usize, usize),
  value_per_x: usize,
) -> usize {
  let (t1, x1, y1) = u;
  let (t2, x2, y2) = v;
  let x = if x1 > x2 {
    let diff = x1 - x2;
    let xs = t1.xs_to_left - t2.xs_to_left;
    diff + (xs * value_per_x)
  } else {
    let diff = x2 - x1;
    let xs = t2.xs_to_left - t1.xs_to_left;
    diff + (xs * value_per_x)
  };

  let y = if y1 > y2 {
    let diff = y1 - y2;
    let ys = t1.xs_to_top - t2.xs_to_top;
    diff + (ys * value_per_x)
  } else {
    let diff = y2 - y1;
    let ys = t2.xs_to_top - t1.xs_to_top;
    diff + (ys * value_per_x)
  };

  x + y
}

pub fn part_1() -> io::Result<usize> {
  let mut input = parse_input()?;
  let with_expansion = handle_expansion(&mut input);

  let galaxy_positions = find_all_galaxies(&with_expansion);

  let value_per_x = 1;
  let mut total = 0;

  for x in &galaxy_positions {
    for y in &galaxy_positions {
      let distance = distance_between(x, y, value_per_x);
      total += distance;
    }
  }

  Ok(total / 2)
}

pub fn part_2() -> io::Result<usize> {
  let mut input = parse_input()?;
  let with_expansion = handle_expansion(&mut input);

  let galaxy_positions = find_all_galaxies(&with_expansion);

  let value_per_x = 999_999;
  let mut total = 0;

  for x in &galaxy_positions {
    for y in &galaxy_positions {
      let distance = distance_between(x, y, value_per_x);
      total += distance;
    }
  }

  Ok(total / 2)
}
