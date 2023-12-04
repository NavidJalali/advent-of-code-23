use std::{
    collections::{HashSet, VecDeque},
    io,
};

use crate::fs::read_day;

#[derive(Debug, PartialEq)]
struct Card {
    number: u32,
    winning: HashSet<u32>,
    given: HashSet<u32>,
}

impl Card {
    fn winning_points(&self) -> u32 {
        let intersection_size = self.same_card_count();

        if intersection_size == 0 {
            0
        } else {
            2u32.pow(intersection_size as u32 - 1)
        }
    }

    fn same_card_count(&self) -> usize {
        self.winning.intersection(&self.given).count()
    }

    fn from_string(line: String) -> Option<Card> {
        let mut parts = line.split(":");

        let card_number: u32 = parts
            .next()?
            .trim()
            .split(" ")
            .filter(|s| !(*s).is_empty())
            .nth(1)?
            .parse()
            .ok()?;

        let numbers_half: &str = parts.next()?.trim();

        let mut winning_and_given = numbers_half.split("|").filter(|s| !s.is_empty()).map(|s| {
            s.trim()
                .split(" ")
                .filter(|s| !(*s).is_empty())
                .flat_map(|s| s.trim().parse())
                .collect::<HashSet<u32>>()
        });

        let winning = winning_and_given.next()?;
        let given = winning_and_given.next()?;

        let card = Card {
            number: card_number,
            winning,
            given,
        };

        Some(card)
    }
}

pub fn part_1() -> io::Result<u32> {
    let result = read_day(4)?
        .map(|line| Card::from_string(line).unwrap())
        .map(|card| card.winning_points())
        .sum::<u32>();

    Ok(result)
}

pub fn part_2() -> io::Result<u32> {
    // Kinda unfortunate that I have no immutable VecDeque but as a wise man once said "it is what it is"
    let result = read_day(4)?
        .map(|line| Card::from_string(line).unwrap())
        .fold(
            (0u32, VecDeque::<u32>::new()),
            |(current_total, mut instances), card| {
                let instances_of_this_card = instances.pop_front().unwrap_or(1);

                let cards_won = card.same_card_count();

                if instances.len() < cards_won {
                    instances.resize(cards_won, 1);
                }

                for i in 0..cards_won {
                    instances[i] += instances_of_this_card;
                }

                (current_total + instances_of_this_card, instances)
            },
        )
        .0;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_line() {
        let line = "Card   8: 48 59 27  1 38 92 65 44 80 87 |  1 92 38 44 18 46 80 59 87 48 67 81 10 71 36 34 89 27 23 33 88 84 83 16 65".to_string();

        let card = Card::from_string(line).unwrap();

        assert_eq!(
            card,
            Card {
                number: 8,
                winning: vec![48, 59, 27, 1, 38, 92, 65, 44, 80, 87]
                    .into_iter()
                    .collect(),
                given: vec![
                    1, 92, 38, 44, 18, 46, 80, 59, 87, 48, 67, 81, 10, 71, 36, 34, 89, 27, 23, 33,
                    88, 84, 83, 16, 65
                ]
                .into_iter()
                .collect()
            }
        );
    }
}
