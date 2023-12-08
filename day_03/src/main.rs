use std::{env, io, collections::HashSet};

const ADJ_OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    ( 0, -1),
    ( 1, -1),
    (-1,  0),
    ( 1,  0),
    (-1,  1),
    ( 0,  1),
    ( 1,  1)
];

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
    let schem: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let schem_h = schem.len() as i32;
    let schem_w = schem[0].len() as i32;

    let mut engine_adj: Vec<Vec<bool>> = vec![vec![false; schem_w as usize]; schem_h as usize];
    for r_i in 0..schem_h {
        for c_i in 0..schem_w {
            let ch = schem[r_i as usize][c_i as usize];
            if !ch.is_digit(10) && ch != '.' {
                for (dr, dc) in ADJ_OFFSETS {
                    let r_j = r_i + dr;
                    let c_j = c_i + dc;
                    if 0 <= r_j && r_j < schem_h && 0 <= c_j && c_j < schem_w {
                        engine_adj[r_j as usize][c_j as usize] = true;
                    }
                }
            }
        }
    }

    let mut total = 0;
    for r in 0..schem_h {
        let mut c = 0;
        while c < schem_w {
            if !schem[r as usize][c as usize].is_digit(10) {
                c += 1;
                continue;
            }

            let mut n = 0;
            let mut is_adj = false;

            while c < schem_w && schem[r as usize][c as usize].is_digit(10) {
                let d = schem[r as usize][c as usize].to_digit(10).unwrap();
                n = n * 10 + d;
                if engine_adj[r as usize][c as usize] {
                    is_adj = true;
                }
                c += 1;
            }

            if is_adj {
                total += n;
            }
        }
    }

    println!("{}", total);
}

fn p2() {
    let schem: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let schem_h = schem.len() as i32;
    let schem_w = schem[0].len() as i32;

    let mut engine_adj: Vec<Vec<Vec<i32>>> = vec![vec![vec![]; schem_w as usize]; schem_h as usize];
    let mut next_gear = 0;
    for r_i in 0..schem_h {
        for c_i in 0..schem_w {
            let ch = schem[r_i as usize][c_i as usize];
            if ch == '*' {
                for (dr, dc) in ADJ_OFFSETS {
                    let r_j = r_i + dr;
                    let c_j = c_i + dc;
                    if 0 <= r_j && r_j < schem_h && 0 <= c_j && c_j < schem_w {
                        engine_adj[r_j as usize][c_j as usize].push(next_gear);
                    }
                }
                next_gear += 1;
            }
        }
    }

    let mut nums_by_gear: Vec<Vec<i32>> = vec![vec![]; next_gear as usize];
    for r in 0..schem_h {
        let mut c = 0;
        while c < schem_w {
            if !schem[r as usize][c as usize].is_digit(10) {
                c += 1;
                continue;
            }

            let mut acc = 0;
            let mut adj_gears = HashSet::new();

            while c < schem_w && schem[r as usize][c as usize].is_digit(10) {
                let n = schem[r as usize][c as usize].to_digit(10).unwrap();
                acc = acc * 10 + n;
                for &gear_id in engine_adj[r as usize][c as usize].iter() {
                    adj_gears.insert(gear_id);
                }
                c += 1;
            }

            for gear_id in adj_gears {
                nums_by_gear[gear_id as usize].push(acc as i32);
            }
        }
    }

    let mut total = 0;
    for nums in nums_by_gear {
        if nums.len() == 2 {
            total += nums[0] * nums[1];
        }
    }

    println!("{}", total);
}
