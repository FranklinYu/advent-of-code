use std::cmp;
use std::io;
use std::vec::Vec;

use regex::Regex;

// Colors are RGB.
#[derive(PartialEq, Debug, Default)]
struct Hand(i32, i32, i32);

fn parse_hand(hand_str: &str) -> Hand {
    let stripped = hand_str.strip_suffix("; ").unwrap_or(hand_str);
    let mut hand = Hand::default();
    for atom in stripped.split(", ") {
        if let Some(num_str) = atom.strip_suffix(" red") {
            hand.0 = num_str.parse().unwrap()
        } else if let Some(num_str) = atom.strip_suffix(" green") {
            hand.1 = num_str.parse().unwrap()
        } else if let Some(num_str) = atom.strip_suffix(" blue") {
            hand.2 = num_str.parse().unwrap()
        } else {
            panic!("unexpected atom: {atom}");
        }
    }
    hand
}

fn banner_regex() -> Regex {
    Regex::new(r"Game (\d+): ").unwrap()
}

#[derive(PartialEq, Debug)]
struct Game {
    id: i32,
    hands: Vec<Hand>,
}

impl Game {
    fn parse(banner_re: &Regex, line: &str) -> Game {
        let banner_cap = banner_re.captures(line).unwrap();
        let id: i32 = banner_cap[1].parse().unwrap();
        let trailer = line.strip_prefix(&banner_cap[0]).unwrap();
        Game {
            id: id,
            hands: trailer.split("; ").map(parse_hand).collect(),
        }
    }

    fn is_possible(&self) -> bool {
        let mut min = Hand::default();
        for hand in &self.hands {
            min.0 = cmp::max(min.0, hand.0);
            min.1 = cmp::max(min.1, hand.1);
            min.2 = cmp::max(min.2, hand.2);
        }
        min.0 <= 12 && min.1 <= 13 && min.2 <= 14
    }
}

pub fn part_1<B: io::BufRead>(input: io::Lines<B>) -> io::Result<String> {
    let banner_re = banner_regex();
    let mut sum = 0;
    for line in input {
        let game = Game::parse(&banner_re, &line?);
        if game.is_possible() {
            sum += game.id;
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("1 red, 23 green, 6 blue" => crate::day_02::Hand(1, 23, 6))]
    #[test_case("2 red, 5 green, 62 blue; " => crate::day_02::Hand(2, 5, 62))]
    fn it_parses_a_hand(hand_str: &str) -> super::Hand {
        super::parse_hand(hand_str)
    }

    #[test]
    fn it_parses_games() {
        use super::Hand;
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = super::Game {
            id: 1,
            hands: vec![Hand(4, 0, 3), Hand(1, 2, 6), Hand(0, 2, 0)],
        };
        assert_eq!(super::Game::parse(&super::banner_regex(), line), game);
    }

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => true)]
    #[test_case("Game 310: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red" => false)]
    fn it_makes_the_right_decision(line: &str) -> bool {
        let banner_re = super::banner_regex();
        super::Game::parse(&banner_re, line).is_possible()
    }
}
