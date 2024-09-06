use std::collections::HashMap;

#[repr(u16)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Ord, Hash)]
enum Card {
    Spot(u8),
    Jack = 257,
    Ten = 256,
    Queen = 258,
    King = 259,
    Ace = 260,
}

#[derive(PartialEq, Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    kind: HandKind,
}

#[derive(PartialEq, PartialOrd, Debug, Eq, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn get_kind(cards: &[Card; 5]) -> HandKind {
        let mut map = HashMap::new();
        for card in cards {
            *map.entry(*card).or_insert(0) += 1
        }

        let mut appearances: Vec<i32> = map.into_values().collect();
        appearances.sort();
        appearances.reverse();

        let first = appearances.first().unwrap();
        let second = appearances.get(1);

        match (*first, second) {
            (5, _) => HandKind::FiveOfAKind,
            (4, _) => HandKind::FourOfAKind,
            (3, Some(2)) => HandKind::FullHouse,
            (3, _) => HandKind::ThreeOfAKind,
            (2, Some(2)) => HandKind::TwoPair,
            (2, _) => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            c if ('2'..='9').contains(&c) => Ok(Card::Spot(c as u8 - b'0')),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a str> for Hand {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            return Err(());
        }

        let cards: Vec<char> = value.chars().collect();
        let cards: [Card; 5] = cards
            .into_iter()
            .map(|card| Card::try_from(card).unwrap())
            .take(5)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ())?;

        let kind = HandKind::get_kind(&cards);

        Ok(Hand { cards, kind })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind
            .cmp(&other.kind)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("input.a");
    let mut hands = vec![];

    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();

        let hand = Hand::try_from(hand).unwrap();
        let bid = bid.parse::<u32>().unwrap();

        hands.push((hand, bid));
    }

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum = 0;
    for (idx, (_, bid)) in hands.into_iter().enumerate() {
        sum += (idx + 1) as u32 * bid;
    }

    println!("{sum}")
}

#[cfg(test)]
mod test {
    use crate::{Card, Hand};

    #[test]
    fn test_parse() {
        let hand = Hand::try_from("JQQJQ").unwrap();
        assert_eq!(
            hand,
            Hand {
                cards: [
                    Card::Jack,
                    Card::Queen,
                    Card::Queen,
                    Card::Jack,
                    Card::Queen,
                ],
                kind: crate::HandKind::FullHouse
            }
        );
    }
}
