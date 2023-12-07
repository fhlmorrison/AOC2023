use std::fs::read_to_string;

fn main() {
    // let reader = read_to_string("./inputs/test7A.txt").unwrap();
    let reader = read_to_string("./inputs/day7.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl HandKind {
    fn from_cards(cards: &[u32; 5]) -> HandKind {
        let mut counts = [0; 13];

        cards.iter().for_each(|i| counts[*i as usize - 2] += 1);

        counts.sort();

        match counts[12] {
            1 => HandKind::HighCard,
            2 => match counts[11] {
                2 => HandKind::TwoPairs,
                _ => HandKind::OnePair,
            },
            3 => match counts[11] {
                2 => HandKind::FullHouse,
                _ => HandKind::ThreeOfAKind,
            },
            4 => HandKind::FourOfAKind,
            5 => HandKind::FiveOfAKind,
            _ => panic!("Invalid card count"),
        }
    }
    fn from_cards_with_jokers(cards: &[u32; 5]) -> HandKind {
        let mut counts = [0; 14];

        cards.iter().for_each(|i| counts[*i as usize - 1] += 1);

        let jokers = counts[0];
        counts[0] = 0;

        counts.sort();

        counts[13] += jokers;

        match counts[13] {
            1 => HandKind::HighCard,
            2 => match counts[12] {
                2 => HandKind::TwoPairs,
                _ => HandKind::OnePair,
            },
            3 => match counts[12] {
                2 => HandKind::FullHouse,
                _ => HandKind::ThreeOfAKind,
            },
            4 => HandKind::FourOfAKind,
            5 => HandKind::FiveOfAKind,
            _ => panic!("Invalid card count"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Box<[u32; 5]>,
    kind: HandKind,
}

impl Hand {
    fn compare(&self, other: &Hand) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    match other.cards[i].cmp(&self.cards[i]) {
                        std::cmp::Ordering::Equal => continue,
                        other_ord => return other_ord,
                    }
                }
                std::cmp::Ordering::Equal
            }
            other => other,
        }
    }
    fn from_str(s: &str) -> Hand {
        let cards: Box<[u32; 5]> = s
            .chars()
            .take(5)
            .map(|s| match s {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => s.to_digit(10).unwrap(),
            })
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        let kind = HandKind::from_cards(&cards);
        Hand { cards, kind }
    }
    fn from_str_with_jokers(s: &str) -> Hand {
        let cards: Box<[u32; 5]> = s
            .chars()
            .take(5)
            .map(|s| match s {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => s.to_digit(10).unwrap(),
            })
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        let kind = HandKind::from_cards_with_jokers(&cards);
        Hand { cards, kind }
    }
}

fn part_1(reader: &str) {
    let lines = reader.lines();

    let mut hands = lines
        .map(|s| {
            let mut split = s.split_whitespace();
            let hand = Hand::from_str(split.next().unwrap());
            let bid = split.next().unwrap().parse::<u32>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|(a, _), (b, _)| b.compare(a));

    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| ((i + 1) as u32) * bid)
        .sum();

    println!("Part 1: {}", sum);
}

fn part_2(reader: &str) {
    let lines = reader.lines();

    let mut hands = lines
        .map(|s| {
            let mut split = s.split_whitespace();
            let hand = Hand::from_str_with_jokers(split.next().unwrap());
            let bid = split.next().unwrap().parse::<u32>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|(a, _), (b, _)| b.compare(a));

    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| ((i + 1) as u32) * bid)
        .sum();

    println!("Part 2: {}", sum);
}
