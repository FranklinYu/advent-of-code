use std::collections::HashMap;
use std::io;

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Hash, Clone, Copy)]
struct Card(usize);

const RANKS_V1: &str = "23456789TJQKA";
const RANKS_V2: &str = "J23456789TQKA";

impl Card {
    fn from(ch: char, ranks: &str) -> Option<Card> {
        Some(Card(ranks.find(ch)?))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn type_of(cards: [Card; 5], joker: Option<Card>) -> Type {
    let mut card_count = HashMap::<Card, usize>::default();
    for card in cards {
        *card_count.entry(card).or_default() += 1;
    }
    if let Some(j) = joker {
        let j_count = card_count.remove(&j).unwrap_or(0);
        if j_count == 5 {
            card_count.insert(j, j_count);
        } else {
            *card_count.iter_mut().max_by_key(|(_, v)| **v).unwrap().1 += j_count;
        }
    }
    let mut counts = [0, 0, 0, 0, 0];
    for (_, count) in card_count {
        counts[count - 1] += 1;
    }
    match counts {
        [0, 0, 0, 0, 1] => Type::FiveOfAKind,
        [1, 0, 0, 1, 0] => Type::FourOfAKind,
        [0, 1, 1, 0, 0] => Type::FullHouse,
        [2, 0, 1, 0, 0] => Type::ThreeOfAKind,
        [1, 2, 0, 0, 0] => Type::TwoPair,
        [3, 1, 0, 0, 0] => Type::OnePair,
        [5, 0, 0, 0, 0] => Type::HighCard,
        c => panic!("unexpected comination: {:?}", c),
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    ty: Type,
    cards: [Card; 5],
    bid: i32,
}

impl Hand {
    fn from_str(s: &str, joker: Option<Card>) -> Self {
        let mut parts = s.split(' ');

        let mut cards = <[Card; 5]>::default();
        let ranks = if joker.is_some() { RANKS_V2 } else { RANKS_V1 };
        for (i, ch) in parts.next().unwrap().char_indices() {
            cards[i] = Card::from(ch, ranks).unwrap();
        }

        Hand {
            ty: type_of(cards, joker),
            cards,
            bid: parts.next().unwrap().parse().unwrap(),
        }
    }
}

fn main<B: io::BufRead>(input: io::Lines<B>, joker: Option<Card>) -> io::Result<String> {
    let mut hands = Vec::<Hand>::default();
    for line in input {
        hands.push(Hand::from_str(&line?, joker));
    }
    hands.sort_unstable();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as i32 + 1) * h.bid)
        .sum::<i32>()
        .to_string())
}

pub fn part_1<B: io::BufRead>(input: io::Lines<B>) -> io::Result<String> {
    main(input, None)
}

pub fn part_2<B: io::BufRead>(input: io::Lines<B>) -> io::Result<String> {
    let joker = Card::from('J', RANKS_V2).unwrap();
    main(input, Some(joker))
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn it_parses_a_line() {
        assert_eq!(
            super::Hand::from_str("32T3K 765", None),
            super::Hand {
                cards: [Card(1), Card(0), Card(8), Card(1), Card(11)],
                bid: 765,
                ty: super::Type::OnePair
            }
        );
    }
}
