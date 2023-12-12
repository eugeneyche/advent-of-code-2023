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

fn read_springs_and_patterns_from_stdin() -> Vec<(Vec<Spring>, Vec<i32>)> {
    io::stdin().lines().map(|l| {
        let l = l.unwrap();
        let parts: Vec<&str> = l.split_whitespace().collect();
        let springs: Vec<_> = parts[0].chars().map(|ch| {
            match ch {
                '?' => Spring::Unknown,
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                ch => panic!("Unexpected spring char {}.", ch)
            }
        }).collect();
        let pattern: Vec<i32> = parts[1].split(',').map(|n| n.parse().unwrap()).collect();
        (springs, pattern)
    }).collect()
}

#[derive(Clone, Copy, PartialEq)]
enum Spring {
    Unknown,
    Operational,
    Damaged
}

fn search(springs: &[Spring], pattern: &[i32]) -> i64 {
    let s_len = springs.len();
    let p_len = pattern.len();
    let mut dp = vec![vec![0; s_len + 1]; p_len + 1];
    dp[0][0] = 1;
    for ip in 0..=p_len {
        for is in 0..s_len {
            if dp[ip][is] == 0 {
                continue;
            }
            let s = springs[is];
            if s == Spring::Unknown || s == Spring::Operational {
                dp[ip][is + 1] += dp[ip][is];
            }
            if (s == Spring::Unknown || s == Spring::Damaged) && ip < p_len {
                let n = pattern[ip];
                let start = is;
                let end = is + n as usize;
                let has_prefix = end <= s_len && springs[start..end].iter().all(|&s| {
                    s == Spring::Damaged || s == Spring::Unknown
                });

                if !has_prefix {
                    continue;
                }

                if end == s_len {
                    dp[ip + 1][end] += dp[ip][is];
                } else if springs[end] == Spring::Unknown || springs[end] == Spring::Operational {
                    dp[ip + 1][end + 1] += dp[ip][is];
                }
            }
        }
    }
    dp[p_len][s_len]
}

fn p1() {
    let mut total = 0;
    for (mut springs, pattern) in read_springs_and_patterns_from_stdin() {
        let perms = search(&mut springs, &pattern);
        total += perms;
    }
    println!("{}", total);
}

fn p2() {
    let mut total = 0;
    for (mut springs, pattern) in read_springs_and_patterns_from_stdin() {
        springs.insert(0, Spring::Unknown);
        let mut exp_springs: Vec<Spring> = springs.iter().cycle().skip(1).take(5 * springs.len() - 1).cloned().collect();
        let exp_pattern: Vec<i32> = pattern.iter().cycle().take(5 * pattern.len()).cloned().collect();
        let perms = search(&mut exp_springs, &exp_pattern);
        total += perms;
    }
    println!("{}", total);
}