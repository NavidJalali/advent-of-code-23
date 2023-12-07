#![allow(dead_code)]
use std::{cmp::Ordering, fmt::Debug, io};

use crate::fs::read_day;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Ord)]
enum Card {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
  Joker,
}

impl Card {
  fn value(&self) -> u8 {
    match self {
      Card::Joker => 1,
      Card::Two => 2,
      Card::Three => 3,
      Card::Four => 4,
      Card::Five => 5,
      Card::Six => 6,
      Card::Seven => 7,
      Card::Eight => 8,
      Card::Nine => 9,
      Card::Ten => 10,
      Card::Jack => 11,
      Card::Queen => 12,
      Card::King => 13,
      Card::Ace => 14,
    }
  }
}

impl From<char> for Card {
  fn from(c: char) -> Self {
    match c {
      '2' => Card::Two,
      '3' => Card::Three,
      '4' => Card::Four,
      '5' => Card::Five,
      '6' => Card::Six,
      '7' => Card::Seven,
      '8' => Card::Eight,
      '9' => Card::Nine,
      'T' => Card::Ten,
      'J' => Card::Jack,
      'Q' => Card::Queen,
      'K' => Card::King,
      'A' => Card::Ace,
      '*' => Card::Joker,
      _ => panic!("Invalid card"),
    }
  }
}

impl Debug for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Card::Two => write!(f, "2"),
      Card::Three => write!(f, "3"),
      Card::Four => write!(f, "4"),
      Card::Five => write!(f, "5"),
      Card::Six => write!(f, "6"),
      Card::Seven => write!(f, "7"),
      Card::Eight => write!(f, "8"),
      Card::Nine => write!(f, "9"),
      Card::Ten => write!(f, "T"),
      Card::Jack => write!(f, "J"),
      Card::Queen => write!(f, "Q"),
      Card::King => write!(f, "K"),
      Card::Ace => write!(f, "A"),
      Card::Joker => write!(f, "J"),
    }
  }
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.value().cmp(&other.value()))
  }
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, Copy, Clone)]
enum HandType {
  HighCard,
  OnePair,
  TwoPairs,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

impl PartialOrd for HandType {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.value().cmp(&other.value()))
  }
}

impl HandType {
  fn value(&self) -> u8 {
    match self {
      HandType::HighCard => 0,
      HandType::OnePair => 1,
      HandType::TwoPairs => 2,
      HandType::ThreeOfAKind => 3,
      HandType::FullHouse => 4,
      HandType::FourOfAKind => 5,
      HandType::FiveOfAKind => 6,
    }
  }

  fn new(cards: &Vec<Card>) -> Self {
    let mut counts = cards.iter().fold([0; 15], |mut counts, card| {
      counts[card.value() as usize] += 1;
      counts
    });

    let joker_count = counts[1];

    if joker_count == 5 {
      return HandType::FiveOfAKind;
    }

    // now set jokers to 0
    counts[1] = 0;

    let mut counts = counts
      .iter()
      .filter(|&&count| count > 0)
      .map(|&count| count)
      .collect::<Vec<u8>>();

    counts.sort();

    let last_index = counts.len() - 1;
    // now add jokers to the last element
    counts[last_index] += joker_count;

    match counts.as_slice() {
      [1, 1, 1, 1, 1] => HandType::HighCard,
      [1, 1, 1, 2] => HandType::OnePair,
      [1, 2, 2] => HandType::TwoPairs,
      [1, 1, 3] => HandType::ThreeOfAKind,
      [2, 3] => HandType::FullHouse,
      [1, 4] => HandType::FourOfAKind,
      [5] => HandType::FiveOfAKind,
      other => panic!("Invalid hand type: {:?}", other),
    }
  }
}

#[derive(Eq, PartialEq, Ord, Clone)]
struct Hand {
  cards: Vec<Card>,
  hand_type: HandType,
}

impl Debug for Hand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let card_repr = self
      .cards
      .iter()
      .map(|card| format!("{:?}", card))
      .collect::<Vec<_>>()
      .join("");

    write!(f, "[{}, {:?}]", card_repr, self.hand_type)
  }
}

impl Hand {
  fn new(cards: Vec<Card>) -> Self {
    let hand_type = HandType::new(&cards);
    Hand { cards, hand_type }
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    // first compare hand types
    let hand_type_ordering = self.hand_type.cmp(&other.hand_type);

    match hand_type_ordering {
      Ordering::Equal => Some(
        self
          .cards
          .iter()
          .zip(other.cards.iter())
          .find_map(|(&self_card, &other_card)| {
            let card_ordering = self_card.partial_cmp(&other_card);
            match card_ordering {
              Some(Ordering::Equal) => None,
              _ => card_ordering,
            }
          })
          .unwrap_or(Ordering::Equal),
      ),
      _ => Some(hand_type_ordering),
    }
  }
}

#[derive(PartialEq, Eq, Ord, Debug)]
struct Entry {
  hand: Hand,
  bid: u64,
}

impl PartialOrd for Entry {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.hand.partial_cmp(&other.hand)
  }
}

impl Entry {
  fn from_string_part_1(value: String) -> Self {
    let parts = value.split(" ").collect::<Vec<_>>();
    let cards = parts[0].trim().chars().map(Card::from).collect::<Vec<_>>();
    let hand = Hand::new(cards);
    let bid = parts[1].trim().parse::<u64>().unwrap();
    Entry { hand, bid }
  }

  fn from_string_part_2(value: String) -> Self {
    let parts = value.split(" ").collect::<Vec<_>>();
    let cards = parts[0]
      .trim()
      .chars()
      .map(|c| if c == 'J' { '*' } else { c })
      .map(Card::from)
      .collect::<Vec<_>>();
    let hand = Hand::new(cards);
    let bid = parts[1].trim().parse::<u64>().unwrap();
    Entry { hand, bid }
  }
}

fn winnings(entries: Vec<Entry>) -> u64 {
  entries
    .iter()
    .enumerate()
    .map(|(i, entry)| {
      let rank: u64 = i as u64 + 1;
      (entry, rank)
    })
    .map(|(entry, rank)| (entry.bid * rank))
    .sum()
}

pub fn part_1() -> io::Result<u64> {
  let mut entries = read_day(7)?
    .map(Entry::from_string_part_1)
    .collect::<Vec<_>>();

  entries.sort();

  Ok(winnings(entries))
}

pub fn part_2() -> io::Result<u64> {
  let mut entries = read_day(7)?
    .map(Entry::from_string_part_2)
    .collect::<Vec<_>>();

  entries.sort();

  Ok(winnings(entries))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hand_type() {
    let cards = "QQQJA".chars().map(Card::from).collect::<Vec<_>>();
    let hand_type = HandType::new(&cards);
    assert_eq!(
      cards,
      vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
    );
    assert_eq!(hand_type, HandType::ThreeOfAKind);
  }

  #[test]
  fn test_order_differ_by_hand_type() {
    // 2 pair
    let cards_1: Vec<Card> = "KK677".chars().map(Card::from).collect::<Vec<_>>();

    // 3 of a kind
    let cards_2 = "T55J5".chars().map(Card::from).collect::<Vec<_>>();

    let hand_1 = Hand::new(cards_1);
    let hand_2 = Hand::new(cards_2);

    assert!(hand_1 < hand_2);
  }

  #[test]
  fn test_order_differ_by_card() {
    let cards_1: Vec<Card> = "T55J5".chars().map(Card::from).collect::<Vec<_>>();
    let cards_2 = "QQQJA".chars().map(Card::from).collect::<Vec<_>>();

    let hand_1 = Hand::new(cards_1);
    let hand_2 = Hand::new(cards_2);

    assert!(hand_1 < hand_2);
  }

  #[test]
  fn test_ranks() {
    let cards = vec!["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"];
    let hands = cards
      .iter()
      .map(|&cards| Hand::new(cards.chars().map(Card::from).collect::<Vec<_>>()))
      .collect::<Vec<_>>();

    let mut sorted = hands.clone();
    sorted.sort();

    assert_eq!(hands[0], sorted[0]);
    assert_eq!(hands[3], sorted[1]);
    assert_eq!(hands[2], sorted[2]);
    assert_eq!(hands[1], sorted[3]);
    assert_eq!(hands[4], sorted[4]);
  }

  #[test]
  fn test_cmp() {
    let entry_1 = Entry::from_string_part_2("22QKQ 620".to_string());
    let entry_2 = Entry::from_string_part_2("J2382 26".to_string());

    assert!(entry_1 < entry_2);
  }
}
