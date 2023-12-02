use std::{collections::HashMap, io};

use once_cell::sync::Lazy;

use crate::fs::read_lines;

pub fn part_1() -> io::Result<u32> {
    let result: u32 = read_lines("day_1.txt")?
        .flat_map(|line| {
            let digits = line
                .chars()
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            digits
                .first()
                .and_then(|tens| digits.last().map(|ones| 10 * tens + ones))
        })
        .sum();

    Ok(result)
}

static SPELLED_OUT_DIGITS_MAP: Lazy<HashMap<Vec<char>, u32>> = Lazy::new(|| {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
    .into_iter()
    .map(|(key, value)| (key.chars().collect(), value))
    .collect()
});

fn parse_out_digits(line: &String) -> Vec<u32> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut index = 0;
    let mut collected_digits = vec![];

    while index < chars.len() {
        let c = chars[index];
        let maybe_digit = c.to_digit(10);

        if let Some(digit) = maybe_digit {
            collected_digits.push(digit);
            index += 1;
            continue;
        }

        for (key, value) in SPELLED_OUT_DIGITS_MAP.iter() {
            if index + key.len() > chars.len() {
                continue;
            }

            let key_matches = key
                .iter()
                .enumerate()
                .all(|(i, key_char)| chars[index + i] == *key_char);

            if key_matches {
                collected_digits.push(*value);
                break;
            }
        }
        index += 1;
    }

    collected_digits
}

pub fn part_2() -> io::Result<u32> {
    let result: u32 = read_lines("day_1.txt")?
        .flat_map(|line| {
            let digits = parse_out_digits(&line);
            digits
                .first()
                .and_then(|tens| digits.last().map(|ones| 10 * tens + ones))
        })
        .sum();

    Ok(result)
}
