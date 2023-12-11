use std::cmp;
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

fn read_galaxies_from_stdin() -> Vec<(i64, i64)> {
    io::stdin().lines().enumerate().map(|(r, l)| {
        l.unwrap().chars().enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(|(c, _)| (r as i64, c as i64))
            .collect::<Vec<_>>()
    }).flatten().collect()
}

fn expand_space_between_galaxies(gs: &mut Vec<(i64, i64)>, factor: i64) {
    gs.sort_by_key(|g| g.0);
    let r_shifts: Vec<i64> = gs.windows(2)
        .map(|gs| (factor - 1) * cmp::max(0, gs[1].0 - gs[0].0 - 1))
        .scan(0, |acc, e| {
            *acc += e;
            Some(*acc)
        })
        .collect();
    gs.iter_mut().skip(1).zip(r_shifts).for_each(|(g, d)| g.0 += d);
    gs.sort_by_key(|g| g.1);
    let c_shifts: Vec<i64> = gs.windows(2)
        .map(|gs| (factor - 1) * cmp::max(0, gs[1].1 - gs[0].1 - 1))
        .scan(0, |acc, e| {
            *acc += e;
            Some(*acc)
        })
        .collect();
    gs.iter_mut().skip(1).zip(c_shifts).for_each(|(g, d)| g.1 += d);
}

fn p1() {
    let mut gs = read_galaxies_from_stdin();
    expand_space_between_galaxies(&mut gs, 2);
    let mut total = 0;
    for i in 0..gs.len() - 1 {
        for j in i + 1..gs.len() {
            total += (gs[i].0 - gs[j].0).abs() + (gs[i].1 - gs[j].1).abs();
        }
    }
    println!("{}", total);
}

fn p2() {
    let mut gs = read_galaxies_from_stdin();
    expand_space_between_galaxies(&mut gs, 1_000_000);
    let mut total = 0;
    for i in 0..gs.len() - 1 {
        for j in i + 1..gs.len() {
            total += (gs[i].0 - gs[j].0).abs() + (gs[i].1 - gs[j].1).abs();
        }
    }
    println!("{}", total);
}