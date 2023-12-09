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

fn derivative(l: &[i32]) -> Vec<i32> {
    l.windows(2).map(|w| w[1] - w[0]).collect()
}

fn p1() {
    let mut total = 0;
    for l in io::stdin().lines().map(|l| l.unwrap()) {
        let hist: Vec<i32> = l.split_whitespace().map(|t| t.parse().unwrap()).collect();
        let mut ds: Vec<i32> = vec![*hist.last().unwrap()];
        let mut curr = hist;
        while !curr.iter().all(|&v| v == 0) {
            curr = derivative(&curr);
            ds.push(*curr.last().unwrap());
        }
        let ext = ds.iter().rev().fold(0, |acc, &e| acc + e);
        total += ext
    }
    println!("{}", total);
}

fn p2() {
    let mut total = 0;
    for l in io::stdin().lines().map(|l| l.unwrap()) {
        let hist: Vec<i32> = l.split_whitespace().map(|t| t.parse().unwrap()).collect();
        let mut ds: Vec<i32> = vec![*hist.first().unwrap()];
        let mut curr = hist;
        while !curr.iter().all(|&v| v == 0) {
            curr = derivative(&curr);
            ds.push(*curr.first().unwrap());
        }
        let ext = ds.iter().rev().fold(0, |acc, &e| e - acc);
        total += ext
    }
    println!("{}", total);
}
