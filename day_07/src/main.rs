use std::env;
use std::io;

fn main() {
    if env::args().any(|x| x == "--p1") {
        println!("Part 1:");
        p1();
    } else {
        println!("Part 2:");
        p2();
    }
}

#[derive(Debug)]
struct CardPlay {
    hand: String,
    bid: i32,
}

fn read_card_plays_from_stdin() -> Vec<CardPlay> {
    io::stdin().lines().map(|l| {
        let l = l.unwrap();
        let parts: Vec<&str> = l.split(" ").collect();
        let hand: String = parts[0].into();
        let bid: i32 = parts[1].parse().unwrap();
        CardPlay { hand, bid }
    }).collect()
}

enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

fn hand_type_from_groups(groups: &[i32]) -> HandType {
    if groups[5] > 0 {
        HandType::FiveOfAKind
    } else if groups[4] > 0 {
        HandType::FourOfAKind
    } else if groups[3] > 0 && groups[2] > 0 {
        HandType::FullHouse
    } else if groups[3] > 0 {
        HandType::ThreeOfAKind
    } else if groups[2] > 1 {
        HandType::TwoPair
    } else if groups[2] > 0 {
        HandType::Pair
    } else {
        HandType::HighCard
    }
}

fn hand_type(hand: &str) -> HandType {
    let mut hand: Vec<char> = hand.chars().collect();
    hand.sort();
    let mut groups = vec![0; 6];
    let mut it = 0;
    while it < hand.len() {
        let begin = it;
        while it < hand.len() && hand[it] == hand[begin] {
            it += 1;
        }
        let size = it - begin;
        groups[size] += 1;
    }
    hand_type_from_groups(&groups)
}

fn card_rank(c: char) -> i32 {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        c => c.to_digit(10).unwrap() as i32,
    }
}

fn hand_to_sortable_tuple(hand: &str, rank_fn: &dyn Fn(char) -> i32) -> (i32, i32, i32, i32, i32) {
    let ranks: Vec<i32> = hand.chars().map(rank_fn).collect();
    (ranks[0], ranks[1], ranks[2], ranks[3], ranks[4])
}

fn p1() {
    let mut cps = read_card_plays_from_stdin();
    cps.sort_by_key(|cp| (hand_type(&cp.hand) as i32, hand_to_sortable_tuple(&cp.hand, &card_rank)));
    let total: i32 = cps.iter().enumerate().map(|(i, cp)| (i + 1) as i32 * cp.bid).sum();
    println!("{}", total);
}

fn hand_type_with_joker(hand: &str) -> HandType {
    let mut hand: Vec<char> = hand.chars().collect();
    hand.sort();
    let mut groups = vec![0; 6];
    let mut n_jokers = 0;
    let mut it = 0;
    while it < hand.len() {
        let begin = it;
        while it < hand.len() && hand[it] == hand[begin] {
            it += 1;
        }
        let kind = it - begin;
        match hand[begin] {
            'J' => n_jokers = kind, 
            _ => groups[kind] += 1,
        }
    }
    if let Some(size) = groups.iter().rposition(|&s| s > 0) {
        groups[size] -= 1;
        groups[size + n_jokers] += 1;
    } else if n_jokers > 0 {
        groups[n_jokers] += 1;
    }
    hand_type_from_groups(&groups)
}

fn card_rank_with_joker(c: char) -> i32 {
    match c {
        'J' => 0,
        c => card_rank(c),
    }
}

fn p2() {
    let mut cps = read_card_plays_from_stdin();
    cps.sort_by_key(|cp| (
        hand_type_with_joker(&cp.hand) as i32,
        hand_to_sortable_tuple(&cp.hand, &card_rank_with_joker)
    ));
    let total: i32 = cps.iter().enumerate().map(|(i, cp)| (i + 1) as i32 * cp.bid).sum();
    println!("{}", total);
}