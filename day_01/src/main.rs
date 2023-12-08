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

fn p1() {
    let mut acc = 0;

    for line in io::stdin().lines().map(|x| x.unwrap()) {
        let mut first = 0;
        let mut last = 0;

        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                first = n;
                break;
            }
        }

        for c in line.chars().rev() {
            if let Some(n) = c.to_digit(10) {
                last = n;
                break;
            }
        }

        acc += first * 10 + last;
    }

    println!("{}", acc);
}

fn p2() {
    let mut acc = 0;

    for line in io::stdin().lines().map(|x| x.unwrap()) {
        let mut first = 0;
        let mut last = 0;

        for i in 0..line.len() {
            if let Some(n) = get_number_at_index(&line, i) {
                first = n;
                break;
            }
        }

        for i in (0..line.len()).rev() {
            if let Some(n) = get_number_at_index(&line, i) {
                last = n;
                break;
            }
        }

        acc += first * 10 + last;
    }

    println!("{}", acc);
}

const NUMBER_LITERALS: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn get_number_at_index(s: &str, i: usize) -> Option<i32> {
    if let Some(n) = s.chars().nth(i).unwrap().to_digit(10) {
        return Some(n as i32);
    }

    for (n, &n_str) in NUMBER_LITERALS.iter().enumerate() {
        if s[i..].starts_with(n_str) {
            return Some(n as i32);
        }
    }

    None
}