use std::io;

// Absolutely shameful

use crate::fs::*;

type ParseResult<A> = Option<(usize, A)>;

fn parser_or_else<A>(
    parse_result: ParseResult<A>,
    f: impl FnOnce() -> ParseResult<A>,
) -> ParseResult<A> {
    parse_result.or_else(f)
}

fn parser_and_then<A, B>(
    parse_result: ParseResult<A>,
    f: impl FnOnce((usize, A)) -> ParseResult<B>,
) -> ParseResult<B> {
    parse_result.and_then(f)
}

fn parser_zip_left<A: Clone, B>(
    parse_result: ParseResult<A>,
    f: impl FnOnce((usize, A)) -> ParseResult<B>,
) -> ParseResult<A> {
    parse_result.and_then(|(idx1, value)| f((idx1, value.clone())).map(|(idx2, _)| (idx2, value)))
}

fn parser_zip<A: Clone, B>(
    parse_result: ParseResult<A>,
    f: impl FnOnce((usize, A)) -> ParseResult<B>,
) -> ParseResult<(A, B)> {
    parse_result.and_then(|(idx1, value)| {
        f((idx1, value.clone())).map(|(idx2, value2)| (idx2, (value, value2)))
    })
}

fn parser_at_least_one<A>(parse_result: ParseResult<Vec<A>>) -> ParseResult<Vec<A>> {
    parse_result.and_then(|(idx1, value)| {
        if value.len() > 0 {
            Some((idx1, value))
        } else {
            None
        }
    })
}

// parse a vector of A's separated by B's
fn parser_repeatedly_intercalated_with<A, B>(
    parser: impl Fn(&Vec<char>, usize) -> ParseResult<A>,
    separator: impl Fn(&Vec<char>, usize) -> ParseResult<B>,
) -> impl Fn(&Vec<char>, usize) -> ParseResult<Vec<A>> {
    move |input, start_index| {
        let mut index = start_index;
        let mut result = vec![];

        while index < input.len() {
            match parser(input, index) {
                Some((next_index, value)) => {
                    result.push(value);
                    index = next_index;
                    match separator(input, next_index) {
                        Some((next_index, _)) => {
                            index = next_index;
                        }
                        None => break,
                    }
                }
                None => break,
            }
        }

        Some((index, result))
    }
}

fn parse_string(input: &Vec<char>, index: usize, string: &str) -> ParseResult<String> {
    // check if there are enough characters left in the input to parse the string
    if index + string.len() > input.len() {
        return None;
    }
    // check if the characters in the input match the characters in the string
    let string_matches = string
        .chars()
        .enumerate()
        .all(|(i, string_char)| input[index + i] == string_char);

    if string_matches {
        Some((index + string.len(), string.to_string()))
    } else {
        None
    }
}

fn parse_spaces(input: &Vec<char>, index: usize) -> ParseResult<()> {
    let mut index = index;
    while index < input.len() {
        let maybe_whitespace = input[index] == ' ';
        if maybe_whitespace {
            index += 1;
        } else {
            break;
        }
    }
    Some((index, ()))
}

fn parse_digit(input: &Vec<char>, index: usize) -> ParseResult<u32> {
    let maybe_digit = input[index].to_digit(10);
    maybe_digit.map(|digit| (index + 1, digit))
}

fn parse_u32(input: &Vec<char>, index: usize) -> ParseResult<u32> {
    let mut index = index;
    let mut result = None;

    while index < input.len() {
        let maybe_digit = parse_digit(input, index);
        match maybe_digit {
            Some((next_index, digit)) => {
                index = next_index;
                result = match result {
                    Some(result) => Some(10 * result + digit),
                    None => Some(digit),
                }
            }
            None => break,
        }
    }

    result.map(|result| (index, result))
}

fn parse_game_count(input: &Vec<char>, index: usize) -> ParseResult<u32> {
    let result = parse_string(input, index, "Game");
    let result = parser_and_then(result, |(index, _)| parse_spaces(input, index));
    let result = parser_and_then(result, |(index, _)| parse_u32(input, index));
    let result = parser_and_then(result, |(index, game_count)| {
        parse_string(input, index, ":").map(|(index, _)| (index, game_count))
    });
    let result = parser_and_then(result, |(index, game_count)| {
        parse_spaces(input, index).map(|(index, _)| (index, game_count))
    });
    result
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct RGB {
    red: u32,
    blue: u32,
    green: u32,
}

impl RGB {
    fn add_red(mut self, red: u32) -> Self {
        self.red += red;
        self
    }

    fn add_blue(mut self, blue: u32) -> Self {
        self.blue += blue;
        self
    }

    fn add_green(mut self, green: u32) -> Self {
        self.green += green;
        self
    }

    fn is_pairwise_less_than_or_equal_to(&self, other: &RGB) -> bool {
        self.red <= other.red && self.blue <= other.blue && self.green <= other.green
    }

    fn pair_wise_max(&self, other: &RGB) -> RGB {
        RGB {
            red: std::cmp::max(self.red, other.red),
            blue: std::cmp::max(self.blue, other.blue),
            green: std::cmp::max(self.green, other.green),
        }
    }
}

impl Default for RGB {
    fn default() -> Self {
        RGB {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn from_string(string: &str) -> Option<Color> {
        match string {
            "red" => Some(Color::Red),
            "blue" => Some(Color::Blue),
            "green" => Some(Color::Green),
            _ => None,
        }
    }
}

fn parse_color(input: &Vec<char>, index: usize) -> ParseResult<Color> {
    let result = parse_string(input, index, "red");
    let result = parser_or_else(result, || parse_string(input, index, "blue"));
    let result = parser_or_else(result, || parse_string(input, index, "green"));
    result.map(|(index, color_string)| (index, Color::from_string(&color_string).unwrap()))
}

// Example: 1 green
fn parse_color_with_count(input: &Vec<char>, index: usize) -> ParseResult<(u32, Color)> {
    let result = parse_u32(input, index);
    let result = parser_zip_left(result, |(index, _)| parse_spaces(input, index));
    let result = parser_zip(result, |(index, _)| parse_color(input, index));
    result
}

// example 1 green, 2 blue, 15 red
fn parse_rgb(input: &Vec<char>, index: usize) -> ParseResult<RGB> {
    let result = parser_repeatedly_intercalated_with(
        |input, index| parse_color_with_count(input, index),
        |input, index| parse_string(input, index, ", "),
    )(input, index);

    let result = parser_at_least_one(result);

    result.map(|(index, colors_with_count)| {
        let rgb =
            colors_with_count
                .iter()
                .fold(RGB::default(), |rgb, (count, color)| match color {
                    Color::Red => rgb.add_red(*count),
                    Color::Blue => rgb.add_blue(*count),
                    Color::Green => rgb.add_green(*count),
                });
        (index, rgb)
    })
}

fn parse_rgbs(input: &Vec<char>, index: usize) -> ParseResult<Vec<RGB>> {
    let result = parser_repeatedly_intercalated_with(
        |input, index| parse_rgb(input, index),
        |input, index| parse_string(input, index, "; "),
    )(input, index);

    let result = parser_at_least_one(result);

    result
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    count: u32,
    rgbs: Vec<RGB>,
}

fn parse_game(input: &Vec<char>, index: usize) -> ParseResult<Game> {
    let result = parse_game_count(input, index);
    let result: Option<(usize, u32)> =
        parser_zip_left(result, |(index, _)| parse_spaces(input, index));
    let result: Option<(usize, (u32, Vec<RGB>))> =
        parser_zip(result, |(index, _)| parse_rgbs(input, index));
    let result = result.map(|(index, (count, rgbs))| (index, Game { count, rgbs }));
    result
}

pub fn part_1() -> io::Result<u32> {
    let max = RGB {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result: u32 = read_day(2)?
        .map(|line| {
            let input = line.chars().collect::<Vec<char>>();
            let parse_result = parse_game(&input, 0);
            let game = parse_result.map(|(_, game)| game);
            game.unwrap()
        })
        .filter_map(|game| {
            if game
                .rgbs
                .iter()
                .all(|rgb| rgb.is_pairwise_less_than_or_equal_to(&max))
            {
                Some(game.count)
            } else {
                None
            }
        })
        .sum();

    Ok(result)
}

pub fn part_2() -> io::Result<u32> {
    let result: u32 = read_day(2)?
        .flat_map(|line| {
            let input = line.chars().collect::<Vec<char>>();
            let parse_result = parse_game(&input, 0);
            let game = parse_result.map(|(_, game)| game);
            game
        })
        .flat_map(|game| {
            game.rgbs
                .into_iter()
                .reduce(|left, right| left.pair_wise_max(&right))
        })
        .map(|rgb| rgb.red * rgb.blue * rgb.green)
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_string_succeed() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let string = "hello";
        let parse_result = parse_string(&input, index, string);
        assert_eq!(parse_result, Some((5, "hello".to_string())));
    }

    #[test]
    fn test_parse_string_fail() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let string = "world";
        let parse_result = parse_string(&input, index, string);
        assert_eq!(parse_result, None);
    }

    #[test]
    fn test_parse_digit_succeed() {
        let input = "123".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_digit(&input, index);
        assert_eq!(parse_result, Some((1, 1)));
    }

    #[test]
    fn test_parse_digit_fail() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_digit(&input, index);
        assert_eq!(parse_result, None);
    }

    #[test]
    fn test_parse_u32_succeed() {
        let input = "123".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_u32(&input, index);
        assert_eq!(parse_result, Some((3, 123)));
    }

    #[test]
    fn test_parse_u32_fail() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_u32(&input, index);
        assert_eq!(parse_result, None);
    }

    #[test]
    fn test_parse_game_count_succeed() {
        let input = "Game 79: 1 green, 2 blue; 15 blue, 12 red, 2 green; 4 red, 6 blue; 10 blue, 8 red; 3 red, 12 blue; 1 green, 12 red, 8 blue"
            .chars()
            .collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_game_count(&input, index);
        assert_eq!(parse_result, Some((9, 79)));
    }

    #[test]
    fn test_parse_game_count_fail() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_game_count(&input, index);
        assert_eq!(parse_result, None);
    }

    #[test]
    fn test_parse_color_succeed() {
        let input = "red".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_color(&input, index);
        assert_eq!(parse_result, Some((3, Color::Red)));

        let input = "blue".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_color(&input, index);
        assert_eq!(parse_result, Some((4, Color::Blue)));

        let input = "green".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_color(&input, index);
        assert_eq!(parse_result, Some((5, Color::Green)));
    }

    #[test]
    fn test_parse_color_fail() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_color(&input, index);
        assert_eq!(parse_result, None);
    }

    #[test]
    fn test_parse_color_with_count_succeed() {
        let input = "1 green".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_color_with_count(&input, index);
        assert_eq!(parse_result, Some((7, (1, Color::Green))));

        let input = "69 blue".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_color_with_count(&input, index);
        assert_eq!(parse_result, Some((7, (69, Color::Blue))));

        let input = "420 red".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_color_with_count(&input, index);
        assert_eq!(parse_result, Some((7, (420, Color::Red))));
    }

    #[test]
    fn test_parse_color_with_count_fail() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_color_with_count(&input, index);
        assert_eq!(parse_result, None);
    }

    #[test]
    fn test_parse_rgb_succeed() {
        let input = "1 green, 2 blue, 15 red".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_rgb(&input, index);
        assert_eq!(
            parse_result,
            Some((
                23,
                RGB {
                    red: 15,
                    blue: 2,
                    green: 1
                }
            ))
        );
    }

    #[test]
    fn test_parse_rgb_fail() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_rgb(&input, index);
        assert_eq!(parse_result, None);
    }

    #[test]
    fn test_reaptedly() {
        let input = "1,2,3,4,5,6".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parser_repeatedly_intercalated_with(
            |input, index| parse_u32(input, index),
            |input, index| parse_string(input, index, ","),
        )(&input, index);
        assert_eq!(parse_result, Some((11, vec![1, 2, 3, 4, 5, 6])));
    }

    #[test]
    fn test_parse_game_succeed() {
        let input = "Game 17: 14 green, 4 red; 1 green, 5 blue, 15 red; 5 green, 14 red, 5 blue";
        let input = input.chars().collect::<Vec<char>>();
        let parse_result = parse_game(&input, 0);
        assert_eq!(
            parse_result,
            Some((
                74,
                Game {
                    count: 17,
                    rgbs: vec![
                        RGB {
                            red: 4,
                            blue: 0,
                            green: 14
                        },
                        RGB {
                            red: 15,
                            blue: 5,
                            green: 1
                        },
                        RGB {
                            red: 14,
                            blue: 5,
                            green: 5
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_parse_game_fail() {
        let input = "hello world".chars().collect::<Vec<char>>();
        let index = 0;
        let parse_result = parse_game(&input, index);
        assert_eq!(parse_result, None);
    }
}
