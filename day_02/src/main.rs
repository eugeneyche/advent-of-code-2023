use std::{env, io};

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
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_line(line: &str) -> Game {
    let game_delim = line.find(":").unwrap();
    let game_id = line[5..game_delim].parse::<u32>().unwrap();
    let line = &line[game_delim + 2..];
    let set_inputs = line.split("; ");

    let mut sets = Vec::new();
    for set_input in set_inputs {
        let mut set = Set { red: 0, green: 0, blue: 0 };
        for dice_input in set_input.split(", ") {
            let mut dice_parts = dice_input.split(" ");
            let count = dice_parts.next().unwrap().parse::<u32>().unwrap();
            let color = dice_parts.next().unwrap();
            match color {
                "red" => set.red += count,
                "green" => set.green += count,
                "blue" => set.blue += count,
                _ => (),
            }
        }
        sets.push(set);
    }

    Game { id: game_id, sets }
}

fn p1() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut acc = 0;
    for line in io::stdin().lines().map(|x| x.unwrap()) {
        let game = parse_line(&line);
        if !game.sets.iter().any(|set| {
            set.red > max_red || set.green > max_green || set.blue > max_blue
        }) {
            acc += game.id;
        }
    }

    println!("{}", acc);
}

fn p2() {
    let mut acc = 0;

    for line in io::stdin().lines().map(|x| x.unwrap()) {
        let game = parse_line(&line);
        let min_red = game.sets.iter().map(|x| x.red).max().unwrap();
        let min_green = game.sets.iter().map(|x| x.green).max().unwrap();
        let min_blue = game.sets.iter().map(|x| x.blue).max().unwrap();
        let power = min_red * min_green * min_blue;
        acc += power;
    }

    println!("{}", acc);
}
