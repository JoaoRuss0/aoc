use crate::day7part1::HandType::*;
use crate::read_file_into_lines;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Ord)]
struct Card {
    representation: char,
    order: u8,
}

impl Card {
    pub const A: Card = Card {
        representation: 'A',
        order: 1,
    };
    pub const K: Card = Card {
        representation: 'K',
        order: 2,
    };
    pub const Q: Card = Card {
        representation: 'Q',
        order: 3,
    };
    pub const J: Card = Card {
        representation: 'J',
        order: 4,
    };
    pub const T: Card = Card {
        representation: 'T',
        order: 5,
    };
    pub const NINE: Card = Card {
        representation: '9',
        order: 6,
    };
    pub const EIGHT: Card = Card {
        representation: '8',
        order: 7,
    };
    pub const SEVEN: Card = Card {
        representation: '7',
        order: 8,
    };
    pub const SIX: Card = Card {
        representation: '6',
        order: 9,
    };
    pub const FIVE: Card = Card {
        representation: '5',
        order: 10,
    };
    pub const FOUR: Card = Card {
        representation: '4',
        order: 11,
    };
    pub const THREE: Card = Card {
        representation: '3',
        order: 12,
    };
    pub const TWO: Card = Card {
        representation: '2',
        order: 13,
    };
    pub const ONE: Card = Card {
        representation: '1',
        order: 14,
    };

    pub fn from_char(character: char) -> Card {
        return Self::get_all()
            .into_iter()
            .find(|card| card.representation == character)
            .unwrap();
    }

    pub fn get_all() -> Vec<Card> {
        return vec![
            Self::A,
            Self::K,
            Self::Q,
            Self::J,
            Self::T,
            Self::NINE,
            Self::EIGHT,
            Self::SEVEN,
            Self::SIX,
            Self::FIVE,
            Self::FOUR,
            Self::THREE,
            Self::TWO,
            Self::ONE,
        ];
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.representation == other.representation
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.order > other.order {
            true => Ordering::Greater,
            false => match self.order < other.order {
                true => Ordering::Less,
                false => Ordering::Equal,
            },
        })
    }
}

#[derive(Clone, Debug, Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
    bid: u16,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter().enumerate().all(|(i, card)| card.eq(&other.cards[i])) && self.bid == other.bid
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let last_index = self.cards.len() - 1;

        for (i, card) in self.cards.iter().enumerate().take(last_index) {
            let comparison: Option<Ordering> = card.partial_cmp(&other.cards[i]);
            if Ordering::Equal == comparison.unwrap() {
                continue;
            }
            return comparison;
        }
        return self.cards[last_index].partial_cmp(&other.cards[last_index]);
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn get_ordered_types() -> Vec<HandType> {
        vec![
            FiveOfAKind,
            FourOfAKind,
            FullHouse,
            ThreeOfAKind,
            TwoPair,
            OnePair,
            HighCard,
        ]
    }
}

pub fn solve() {
    let lines = read_file_into_lines("input7.txt");
    println!("{}", process_lines(lines));
}

fn process_lines(lines: Vec<String>) -> u64 {
    let hands: Vec<Hand> = lines
        .iter()
        .map(|line| get_hands_from_line(line))
        .collect::<Vec<Hand>>();

    let mut grouped_hands: HashMap<HandType, Vec<Hand>> = HashMap::with_capacity(7);
    init_hand_type_map(&mut grouped_hands);
    group_hands_by_type(&mut grouped_hands, &hands);
    return get_total_winnings(&mut grouped_hands, hands.len());
}

fn get_hands_from_line(line: &String) -> Hand {
    let split_line: Vec<&str> = (*line).trim().split(' ').collect();
    let cards = get_cards_from_string(split_line.get(0).unwrap().to_string());

    return Hand {
        cards,
        bid: split_line.get(1).unwrap().parse().unwrap(),
    };
}

fn get_cards_from_string(cards_string: String) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for c in cards_string.chars() {
        cards.push(Card::from_char(c));
    }
    return cards;
}

fn group_hands_by_type(grouped_hands: &mut HashMap<HandType, Vec<Hand>>, hands: &Vec<Hand>) {
    init_hand_type_map(grouped_hands);

    for hand in hands {
        grouped_hands
            .entry(check_hand_type(&hand.cards))
            .or_default()
            .push(hand.clone());
    }
}

fn init_hand_type_map(grouped_hands: &mut HashMap<HandType, Vec<Hand>>) {
    grouped_hands.insert(FiveOfAKind, Vec::new());
    grouped_hands.insert(FourOfAKind, Vec::new());
    grouped_hands.insert(FullHouse, Vec::new());
    grouped_hands.insert(ThreeOfAKind, Vec::new());
    grouped_hands.insert(TwoPair, Vec::new());
    grouped_hands.insert(OnePair, Vec::new());
    grouped_hands.insert(HighCard, Vec::new());
}

fn check_hand_type(cards: &Vec<Card>) -> HandType {
    let mut card_quantity: HashMap<char, u8> = HashMap::with_capacity(5);

    for card in cards {
        match card_quantity.get(&card.representation) {
            None => card_quantity.insert(card.representation, 1),
            Some(entry) => card_quantity.insert(card.representation, *entry + 1),
        };
    }

    let keys = card_quantity.keys();
    match keys.len() {
        1 => FiveOfAKind,
        2 => match card_quantity.get(&cards[0].representation).unwrap() {
            4 | 1 => FourOfAKind,
            _ => FullHouse,
        },
        3 => {
            if *card_quantity.get(&cards[0].representation).unwrap() == 1 {
                return match card_quantity.get(&cards[1].representation).unwrap() {
                    2 => TwoPair,
                    _ => ThreeOfAKind,
                };
            }
            match card_quantity.get(&cards[0].representation).unwrap() {
                2 => TwoPair,
                _ => ThreeOfAKind,
            }
        }
        4 => OnePair,
        _ => HighCard,
    }
}

fn get_total_winnings(grouped_hands: &mut HashMap<HandType, Vec<Hand>>, mut total_hands: usize) -> u64 {
    let mut winnings: u64 = 0;
    for hand_type in HandType::get_ordered_types() {
        let mut group = grouped_hands.get_mut(&hand_type).unwrap();
        group.sort();

        for hand in group {
            winnings += hand.bid as u64 * total_hands as u64;
            total_hands -= 1;
        }
    }
    return winnings;
}
