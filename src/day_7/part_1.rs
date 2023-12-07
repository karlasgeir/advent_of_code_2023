use std::{collections::HashMap, fs};

pub fn run() {
    let filename = "./src/day_7/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut hands = lines.iter().map(|l| parse_hand(l)).collect::<Vec<Hand>>();

    hands.sort_by(|a, b| hands_sorted_by(a, b));
    hands.reverse();

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as i32 + 1) * hand.bid)
        .collect::<Vec<i32>>();

    println!("The answer is: {:?}", winnings.iter().sum::<i32>());
}

fn hands_sorted_by(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let a_index = HandType::index(&a.hand_type) as i32;
    let b_index = HandType::index(&b.hand_type) as i32;

    if a_index - b_index == 0 {
        for i in 0..a.cards.len() {
            let a_card = &a.cards[i];
            let b_card = &b.cards[i];
            let card_comparison = b_card.value - a_card.value;
            if card_comparison != 0 {
                return card_comparison.cmp(&0);
            }
        }

        return a.bid.cmp(&0);
    } else {
        return (a_index - b_index).cmp(&0);
    }
}

fn parse_hand(line: &str) -> Hand {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let cards = parse_cards(parts[0]);
    let hand_type = get_hand_type(&cards);
    let bid = parts[1].parse::<i32>().unwrap();
    return Hand {
        cards,
        hand_type,
        bid,
    };
}

fn get_hand_type(cards: &Vec<Card>) -> HandType {
    let counted_cards = count_cards(&cards);

    let max_same_card = counted_cards.values().max().unwrap();

    if max_same_card == &5 {
        return HandType::FiveOfAKind;
    }
    if max_same_card == &4 {
        return HandType::FourOfAKind;
    }
    if is_full_house(&counted_cards) {
        return HandType::FullHouse;
    }
    if max_same_card == &3 {
        return HandType::ThreeOfAKind;
    }

    let pairs = count_pairs(&counted_cards);

    if pairs == 2 {
        return HandType::TwoPair;
    }
    if pairs == 1 {
        return HandType::OnePair;
    }

    return HandType::HighCard;
}

fn count_cards(cards: &Vec<Card>) -> HashMap<i32, i32> {
    let mut card_counts = HashMap::new();
    for card in cards {
        let card_count = card_counts.entry(card.value).or_insert(0);
        *card_count += 1;
    }

    return card_counts;
}

fn is_full_house(counted_cards: &HashMap<i32, i32>) -> bool {
    let mut is_full_house = false;
    let mut has_three_of_a_kind = false;
    let mut has_two_of_a_kind = false;
    for card_count in counted_cards.values() {
        if card_count == &3 {
            has_three_of_a_kind = true;
        } else if card_count == &2 {
            has_two_of_a_kind = true;
        }
    }
    if has_three_of_a_kind && has_two_of_a_kind {
        is_full_house = true;
    }
    return is_full_house;
}

fn count_pairs(counted_cards: &HashMap<i32, i32>) -> i32 {
    return counted_cards
        .iter()
        .map(|(_, v)| v)
        .filter(|v| *v == &2)
        .count() as i32;
}

fn parse_cards(line: &str) -> Vec<Card> {
    let characters = line.chars().collect::<Vec<char>>();
    return characters
        .iter()
        .map(|c| card_label_to_card(c))
        .collect::<Vec<Card>>();
}

fn card_label_to_card(card_label: &char) -> Card {
    let card_value = match card_label {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card_label.to_digit(10).unwrap() as i32,
    };

    return Card {
        value: card_value,
        label: card_label.to_string(),
    };
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: i32,
}

#[derive(Debug, Clone)]
struct Card {
    value: i32,
    label: String,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Debug, Copy, Clone)]
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
    pub fn index(&self) -> usize {
        *self as usize
    }
}
