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

#[derive(Clone, Copy)]
struct RaceInfo {
    time: i64,
    dist: i64,
}

fn read_race_infos_from_stdin() -> Vec<RaceInfo> {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let times: Vec<i64> = lines[0].split_whitespace().skip(1).map(|t| t.parse().unwrap()).collect();
    let dists: Vec<i64> = lines[1].split_whitespace().skip(1).map(|t| t.parse().unwrap()).collect();
    times.iter().copied().zip(dists.iter().copied()).map(|(t, d)| RaceInfo {time: t, dist: d }).collect()
}

fn p1() {
    let rs = read_race_infos_from_stdin();
    let mut total = 1;
    for r in rs.iter().copied() {
        let lower = 1;
        let upper = r.time - 1;
        let mut num_ways_to_win = 0;
        for i in lower..=upper {
            let dist = i * (r.time - i);
            if dist > r.dist {
                num_ways_to_win += 1;
            }
        }
        total *= num_ways_to_win;
    }
    println!("{}", total);
}

fn combine_race_infos(rs: &[RaceInfo]) -> RaceInfo {
    let mut time_str = String::new();
    let mut dist_str = String::new();
    for race_info in rs {
        time_str.push_str(&race_info.time.to_string());
        dist_str.push_str(&race_info.dist.to_string());
    }
    let time: i64 = time_str.parse().unwrap();
    let dist: i64 = dist_str.parse().unwrap();
    RaceInfo { time, dist }
}

fn partition_point(lo: i64, hi: i64, cond: &dyn Fn(i64) -> bool) -> i64 {
    let mut hi = hi;
    let mut lo = lo;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if cond(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    return hi;
}

fn p2 () {
    let rs = read_race_infos_from_stdin();
    let r = combine_race_infos(&rs);
    let t_mid = r.time / 2;
    let lo = partition_point(1, t_mid, &|v| {v * (r.time - v) > r.dist});
    let hi = partition_point(t_mid, r.time, &|v| {v * (r.time - v) < r.dist});
    println!("{}", hi - lo)
}