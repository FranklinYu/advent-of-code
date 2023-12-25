use std::collections::HashMap;
use std::io;

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Hash, Clone, Copy)]
struct Card(usize);

const RANKS: &str = "23456789TJQKA";

impl Card {
    fn from(ch: char) -> Option<Card> {
        Some(Card(RANKS.find(ch)?))
    }

    fn key(&self) -> char {
        RANKS.chars().nth(self.0).unwrap()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn type_of(cards: [Card; 5]) -> Type {
    let mut card_count = HashMap::<Card, usize>::default();
    for card in cards {
        *card_count.entry(card).or_default() += 1;
    }
    let mut counts = [0, 0, 0, 0, 0];
    for (_card, count) in card_count {
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

impl std::str::FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        let mut cards = <[Card; 5]>::default();
        for (i, ch) in parts.next().unwrap().char_indices() {
            cards[i] = Card::from(ch).unwrap();
        }

        Ok(Hand {
            ty: type_of(cards),
            cards,
            bid: parts.next().unwrap().parse().unwrap(),
        })
    }
}

pub fn part_1<B: io::BufRead>(input: io::Lines<B>) -> io::Result<String> {
    let mut hands = Vec::<Hand>::default();
    for line in input {
        hands.push(line?.parse().unwrap());
    }
    hands.sort_unstable();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as i32 + 1) * h.bid)
        .sum::<i32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn it_parses_a_line() {
        assert_eq!(
            "32T3K 765".parse(),
            Ok(super::Hand {
                cards: [Card(1), Card(0), Card(8), Card(1), Card(11)],
                bid: 765,
                ty: super::Type::OnePair
            })
        );
    }
}
