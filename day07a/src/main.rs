use std::collections::HashMap;

/// A Card in a hand.
///
/// By deriving PartialOrd, these are sorted in lexicographic order:
/// Number cards are the lowest, and A is the highest. Num variants are sorted
/// by their contained value.
#[derive(PartialOrd, PartialEq, Ord, Eq, Debug)]
enum Card {
    Num(char),
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '2'..='9' => Self::Num(c),
            _ => panic!("Invalid card: {}", c),
        }
    }
}

/// The type of a hand.
///
/// Like Card, these are sorted in lexicographic order, with the lowest
/// ordered value being HighCard and the highest FiveKind. The final Hand
/// iterator will need to be reversed to get the highest value first.
#[derive(PartialOrd, PartialEq, Ord, Eq, Debug)]
enum HandType {
    // All distinct cards
    HighCard,
    // One pair of identical cards and three unique
    OnePair,
    // Two pairs of identical cards and one unique
    TwoPair,
    // Three identical cards and two unique
    ThreeKind,
    // A pair and trio of identical cards
    FullHouse,
    // Four identical cards
    FourKind,
    // All identical cards
    FiveKind,
}

impl From<&str> for HandType {
    fn from(s: &str) -> Self {
        assert_eq!(s.len(), 5);

        let counts: HashMap<char, usize> = s.chars().fold(HashMap::new(), |mut map, val| {
            *map.entry(val).or_default() += 1;
            map
        });

        let mut sorted_counts = counts.values().cloned().collect::<Vec<_>>();
        sorted_counts.sort();

        match counts.len() {
            1 => Self::FiveKind,
            2 => {
                if sorted_counts[..] == [1, 4] {
                    Self::FourKind
                } else {
                    // Guaranteed that sorted_values will be [2, 3]
                    Self::FullHouse
                }
            }

            3 => {
                if sorted_counts[..] == [1, 1, 3] {
                    Self::ThreeKind
                } else {
                    // Guaranteed that sorted_values will be [1, 2, 2]
                    Self::TwoPair
                }
            }

            4 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

/// A hand of cards.
///
/// Like the types above, this is ordered in lexicographic order (by field).
/// Hands are sorted by HandType first, then failing that by the value of their
/// cards. This ordering is from lowest to highest.
#[derive(PartialOrd, PartialEq, Ord, Eq, Debug)]
struct Hand {
    // type is a reserved keyword, could do "r#type" but this looks better
    hand_type: HandType,
    cards: Vec<Card>,
    bid: usize,
}

fn solution(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');

            let cards = parts.next().unwrap();
            let hand_type = HandType::from(cards);
            let bid: usize = parts.next().unwrap().parse().unwrap();

            let cards = cards.chars().map(Card::from).collect();

            Hand {
                hand_type,
                cards,
                bid,
            }
        })
        .collect();

    hands.sort();

    // The iterator is from lowest to highest, so the rank of card i is i+1
    hands.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 6440);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 249204891);
    }
}
