use std::cmp;
use std::collections::HashSet;
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
struct Game {
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
}

fn parse_line(line: &str) -> Game {
    let delim = line.find(":").unwrap() + 1;
    let parts: Vec<String> = line[delim..].split("|").map(|x| x.trim().into()).collect();
    let winning_numbers: Vec<i32> = parts[0].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let my_numbers: Vec<i32> = parts[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    Game { winning_numbers, my_numbers }
}

fn get_num_matching(game: &Game) -> u32 {
    let winning_num_set: HashSet<i32> = game.winning_numbers.iter().copied().collect();
    let my_num_set: HashSet<i32> = game.my_numbers.iter().copied().collect();
    winning_num_set.intersection((&my_num_set)).count() as u32
}

fn p1() {
    let mut total = 0;
    for line in io::stdin().lines().map(|x| x.unwrap()) {
        let game = parse_line(&line);
        let num_matching = get_num_matching(&game);
        if num_matching > 0 {
            total += 2i32.pow(num_matching - 1);
        }
    }
    println!("{}", total);
}

fn p2() {
    let games: Vec<Game> = io::stdin().lines().map(|x| parse_line(&x.unwrap())).collect();
    let mut bonus_by_card = vec![0; games.len()];
    let mut total = 0;
    for (i, game) in games.iter().enumerate() {
        let num_copies = 1 + bonus_by_card[i];
        total += num_copies;

        let num_matching = get_num_matching(&game) as usize;
        for j in (i + 1)..cmp::min(i + 1 + num_matching, bonus_by_card.len()) {
            bonus_by_card[j] += num_copies;
        }
    }

    println!("{}", total);
}