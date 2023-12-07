use std::collections::HashMap;

/// A Card in a hand.
///
/// By deriving PartialOrd, these are sorted in lexicographic order:
/// A is ordered lowest, and Two the highest. When combined with how HandType
/// and Hard are also ordered, this means that the highest value hand will be
/// first when calling .sort() on a Vec of Hands.
#[derive(PartialOrd, PartialEq, Ord, Eq, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

/// The type of a hand.
///
/// Like Card, these are sorted in lexicographic order, with the lowest
/// ordered value being FiveKind and the highest HighCard. When a Hand is sorted,
/// this means that the highest value hand will be first.
#[derive(PartialOrd, PartialEq, Ord, Eq, Debug)]
enum HandType {
    // All identical cards
    FiveKind,
    // Four identical cards
    FourKind,
    // A pair and trio of identical cards
    FullHouse,
    // Three identical cards and two unique
    ThreeKind,
    // Two pairs of identical cards and one unique
    TwoPair,
    // One pair of identical cards and three unique
    OnePair,
    // All distinct cards
    HighCard,
}

impl From<&str> for HandType {
    fn from(s: &str) -> Self {
        assert_eq!(s.len(), 5);

        let counts: HashMap<char, usize> = s.chars().fold(HashMap::new(), |mut map, val| {
            *map.entry(val).or_default() += 1;
            map
        });

        let mut sorted_values = counts.values().cloned().collect::<Vec<_>>();
        sorted_values.sort();

        match counts.len() {
            1 => Self::FiveKind,
            2 => {
                if sorted_values[..] == [1, 4] {
                    Self::FourKind
                } else {
                    // Guaranteed that sorted_values will be [2, 3]
                    Self::FullHouse
                }
            }

            3 => {
                if sorted_values[..] == [1, 1, 3] {
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

            let cards = cards.chars().map(Card::from).collect::<Vec<_>>();

            Hand {
                hand_type,
                cards,
                bid,
            }
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (hands.len() - i))
        .sum()
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
