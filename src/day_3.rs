use std::io;

use crate::fs::read_day;

#[derive(Debug, PartialEq)]
enum CharKind {
    Digit(u32),
    Empty,
    Symbol(char),
}

impl CharKind {
    fn from_char(c: char) -> Self {
        match c.to_digit(10) {
            Some(digit) => Self::Digit(digit),
            None => match c {
                '.' => Self::Empty,
                _ => Self::Symbol(c),
            },
        }
    }

    fn is_potentially_gear(&self) -> bool {
        match self {
            Self::Symbol(c) => *c == '*',
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Cell {
    value: CharKind,
    flagged: bool,
}

impl Cell {
    fn from_char(c: char) -> Self {
        Self {
            value: CharKind::from_char(c),
            flagged: false,
        }
    }

    fn find_number_at<F>(row: &mut Vec<Cell>, index: usize, flag_fn: F) -> Option<u32>
    where
        F: Fn(&mut Cell) -> (),
    {
        match row.get(index) {
            Some(cell) => {
                if cell.flagged {
                    None
                } else {
                    match cell.value {
                        CharKind::Digit(digit) => {
                            let mut left_bound = index.saturating_sub(1);
                            let mut right_bound = (index + 1).min(row.len() - 1);

                            let mut digits = vec![digit];

                            flag_fn(row.get_mut(index).unwrap());

                            while let Some(Cell {
                                value: CharKind::Digit(next),
                                flagged: _,
                            }) = row.get(left_bound)
                            {
                                digits.insert(0, *next);

                                flag_fn(row.get_mut(left_bound).unwrap());

                                if left_bound == 0 {
                                    break;
                                } else {
                                    left_bound = left_bound.saturating_sub(1);
                                }
                            }

                            while let Some(Cell {
                                value: CharKind::Digit(next),
                                flagged: _,
                            }) = row.get(right_bound)
                            {
                                digits.push(*next);

                                flag_fn(row.get_mut(right_bound).unwrap());

                                if right_bound == row.len() - 1 {
                                    break;
                                } else {
                                    right_bound += 1;
                                }
                            }

                            Some(digits.iter().fold(0, |acc, digit| 10 * acc + digit))
                        }
                        _ => None,
                    }
                }
            }
            None => None,
        }
    }
}

fn adjacent(row: usize, col: usize, max_height: usize, max_width: usize) -> Vec<(usize, usize)> {
    vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .iter()
    .filter_map(|(row, col)| {
        if *row < max_height && *col < max_width {
            Some((*row, *col))
        } else {
            None
        }
    })
    .collect()
}

pub fn part_1() -> io::Result<u32> {
    let mut total: u32 = 0;
    let mut matrix = read_day(3)?
        .map(|line| line.trim().chars().map(Cell::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_height = matrix.len();
    let max_width = matrix.get(0).map_or_else(|| 0, |v| v.len());

    for row_index in 0..max_height {
        for col_index in 0..max_width {
            if let CharKind::Symbol(_) = matrix[row_index][col_index].value {
                for (adj_row, adj_col) in adjacent(row_index, col_index, max_height, max_width) {
                    if let Some(number) =
                        Cell::find_number_at(&mut matrix[adj_row], adj_col, |cell| {
                            cell.flagged = true
                        })
                    {
                        total += number;
                    }
                }
            }
        }
    }

    Ok(total)
}

pub fn part_2() -> io::Result<u32> {
    let mut matrix = read_day(3)?
        .map(|line| line.trim().chars().map(Cell::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_height = matrix.len();
    let max_width = matrix.get(0).map_or_else(|| 0, |v| v.len());

    let mut total: u32 = 0;

    for row_index in 0..max_height {
        for col_index in 0..max_width {
            if matrix[row_index][col_index].value.is_potentially_gear() {
                let adj = adjacent(row_index, col_index, max_height, max_width);
                let mut nums = vec![];
                for (adj_row, adj_col) in &adj {
                    if let Some(number) =
                        Cell::find_number_at(&mut matrix[*adj_row], *adj_col, |cell| {
                            cell.flagged = true
                        })
                    {
                        nums.push(number);
                    }
                }

                match nums.len() {
                    0 => continue,
                    1 => continue,
                    2 => {
                        total += nums[0] * nums[1];
                    }
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Too many numbers found",
                        ));
                    } // So you could consider this a gear or not the question is vague.
                }

                // Reset flags for situations like . . 1 * 2 * 3 . .
                for (adj_row, adj_col) in adj {
                    matrix[adj_row][adj_col].flagged = false;
                }
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number_at() {
        let mut row = vec![
            Cell::from_char('.'),
            Cell::from_char('.'),
            Cell::from_char('*'),
            Cell::from_char('4'),
            Cell::from_char('2'),
            Cell::from_char('0'),
            Cell::from_char('.'),
            Cell::from_char('#'),
            Cell::from_char('.'),
        ];

        let set_as_adjacent_to_symbol = |cell: &mut Cell| {
            cell.flagged = true;
        };

        assert_eq!(
            Cell::find_number_at(&mut row, 4, set_as_adjacent_to_symbol),
            Some(420)
        );

        assert_eq!(
            row.iter().map(|c| c.flagged).collect::<Vec<_>>(),
            vec![false, false, false, true, true, true, false, false, false]
        )
    }
}
